use crate::foreground_window::{Error, WindowInformation, X11Error};
use x11rb::connection::Connection;
use x11rb::protocol::xproto::{
    AtomEnum, ChangeWindowAttributesAux, ConnectionExt, EventMask, Window,
};
use x11rb::protocol::Event;

fn get_focus_window_data<C: ConnectionExt>(conn: &C) -> Result<(), Error> {
    let window = get_focus_window(conn)?;

    let window_name_atom = conn
        .intern_atom(true, "WM_NAME".as_bytes())
        .map_err(|e| Error::WMError(X11Error::ConnectionError(e)))?
        .reply()
        .map_err(|e| Error::WMError(X11Error::ReplyError(e)))?;

    let property = conn
        .get_property(false, window, AtomEnum::WM_NAME, AtomEnum::STRING, 0, 512)
        .map_err(|e| Error::WMError(X11Error::ConnectionError(e)))?
        .reply()
        .map_err(|e| Error::WMError(X11Error::ReplyError(e)))?;
    println!("{:?}", property.value);
    Ok(())
}

fn get_focus_window<C: ConnectionExt>(conn: &C) -> Result<Window, Error> {
    let input_focus = conn
        .get_input_focus()
        .map_err(|e| Error::WMError(X11Error::ConnectionError(e)))?
        .reply()
        .map_err(|e| Error::WMError(X11Error::ReplyError(e)))?;
    // Now query the tree to get the real window
    let mut window = input_focus.focus;
    loop {
        let tree = conn
            .query_tree(window)
            .map_err(|e| Error::WMError(X11Error::ConnectionError(e)))?
            .reply()
            .map_err(|e| Error::WMError(X11Error::ReplyError(e)))?;
        if window == tree.root || tree.parent == tree.root {
            return Ok(window);
        } else {
            window = tree.parent;
        }
    }
}

pub fn foreground_window_observer<F>(cb: F) -> Result<(), Error>
where
    F: Fn(WindowInformation) -> (),
    F: 'static,
{
    // connect to x11
    let (conn, screen_num) =
        x11rb::connect(None).map_err(|e| Error::WMError(X11Error::ConnectError(e)))?;
    // Get the screen
    let screen = &conn.setup().roots[screen_num];
    let values = ChangeWindowAttributesAux::default().event_mask(EventMask::PROPERTY_CHANGE);
    conn.change_window_attributes(
        screen.root,
        &ChangeWindowAttributesAux::new().event_mask(EventMask::PROPERTY_CHANGE),
    )
    .map_err(|e| Error::WMError(X11Error::ConnectionError(e)))?;

    conn.flush()
        .map_err(|e| Error::WMError(X11Error::ConnectionError(e)))?;

    let active_window_atom = conn
        .intern_atom(true, "_NET_ACTIVE_WINDOW".as_bytes())
        .map_err(|e| Error::WMError(X11Error::ConnectionError(e)))?
        .reply()
        .map_err(|e| Error::WMError(X11Error::ReplyError(e)))?;

    println!("Start wait");
    loop {
        let event = conn
            .wait_for_event()
            .map_err(|e| Error::WMError(X11Error::ConnectionError(e)))?;
        if let Event::PropertyNotify(e) = event {
            if e.atom == active_window_atom.atom {
                // Grab the server
                // conn.grab_server().map_err(|e| Error::WMError(X11Error::ConnectionError(e)))?;

                let window_data = get_focus_window_data(&conn);

                // Ungrap the server
                // conn.ungrab_server().map_err(|e| Error::WMError(X11Error::ConnectionError(e)))?;

                let window_data = window_data?;

                println!("Active window: {:?}!", window_data);
            }
            println!("{:?}", e);
        }
    }

    Ok(())
}
