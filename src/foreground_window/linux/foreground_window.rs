use crate::foreground_window::{Error, WindowInformation, X11Error};
use log::warn;
use std::fs;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::{
    Atom, AtomEnum, ChangeWindowAttributesAux, ConnectionExt, EventMask, GetPropertyReply, Window,
};
use x11rb::protocol::Event;
use x11rb::rust_connection::RustConnection;

// Source: https://www.reddit.com/r/rust/comments/f7yrle/get_information_about_current_window_xorg/fijt7f1/

struct Atoms {
    pub net_active_window: Atom,
    pub net_wm_name: Atom,
    pub wm_class: Atom,
    pub net_wm_pid: Atom,
    pub utf8_string: Atom,
    pub string: Atom,
    pub cardinal: Atom,
}

struct WindowData {
    pub window: Window,
    pub window_name: String,
    pub class: String,
    pub instance: String,
    pub pid: u32,
    pub command: String,
}

pub fn foreground_window_observer<F>(cb: F) -> Result<(), Error>
where
    F: Fn(WindowInformation),
    F: 'static,
{
    // connect to x11
    let (conn, screen_num) =
        x11rb::connect(None).map_err(|e| Error::WMError(X11Error::ConnectError(e)))?;
    // Get the screen
    let root = conn.setup().roots[screen_num].root;

    // Get the atoms
    let atoms = Atoms {
        net_active_window: get_or_intern_atom(&conn, b"_NET_ACTIVE_WINDOW"),
        net_wm_name: get_or_intern_atom(&conn, b"_NET_WM_NAME"),
        net_wm_pid: get_or_intern_atom(&conn, b"_NET_WM_PID"),
        utf8_string: get_or_intern_atom(&conn, b"UTF8_STRING"),
        wm_class: AtomEnum::WM_CLASS.into(),
        string: AtomEnum::STRING.into(),
        cardinal: AtomEnum::CARDINAL.into(),
    };

    conn.change_window_attributes(
        root,
        &ChangeWindowAttributesAux::new().event_mask(EventMask::PROPERTY_CHANGE),
    )
    .map_err(|e| Error::WMError(X11Error::ConnectionError(e)))?;

    conn.flush()
        .map_err(|e| Error::WMError(X11Error::ConnectionError(e)))?;

    // Remember last active window, so we don't double send events!
    let mut last_active_window: Window = 0;

    // Send initial window
    send_active_window_information(&cb, &conn, root, &atoms, &mut last_active_window)?;

    loop {
        let event = conn
            .wait_for_event()
            .map_err(|e| Error::WMError(X11Error::ConnectionError(e)))?;
        if let Event::PropertyNotify(e) = event {
            if e.atom == atoms.net_active_window {
                // Grab the server
                // conn.grab_server().map_err(|e| Error::WMError(X11Error::ConnectionError(e)))?;
                send_active_window_information(&cb, &conn, root, &atoms, &mut last_active_window)?;
            }
        }
    }
}

fn send_active_window_information<F>(
    cb: &F,
    conn: &RustConnection,
    root: Window,
    atoms: &Atoms,
    last_active_window: &mut Window,
) -> Result<(), Error>
where
    F: Fn(WindowInformation),
    F: 'static,
{
    let active_window_data = match get_active_window_data(&conn, root, &atoms) {
        None => {
            warn!("No active window selected");
            return Ok(());
        }
        Some(x) => x,
    };

    if *last_active_window == active_window_data.window {
        return Ok(()); // Already known focused!
    }
    *last_active_window = active_window_data.window;

    // Ungrap the server
    // conn.ungrab_server().map_err(|e| Error::WMError(X11Error::ConnectionError(e)))?;
    cb(WindowInformation {
        title: active_window_data.window_name,
        executable: active_window_data.command,
        class_name: active_window_data.class,
    });
    Ok(())
}

fn get_active_window_data(
    conn: &RustConnection,
    root: Window,
    atoms: &Atoms,
) -> Option<WindowData> {
    // Grab the server
    // conn.grab_server().map_err(|e| Error::WMError(X11Error::ConnectionError(e)))?;
    let focus = find_active_window(conn, root, atoms.net_active_window)?;

    let name = get_property(conn, focus, atoms.net_wm_name, atoms.utf8_string);
    let class = get_property(conn, focus, atoms.wm_class, atoms.string);
    let pid = get_property(conn, focus, atoms.net_wm_pid, atoms.cardinal);

    // Parse the result
    let window_name = name
        .map(|v| parse_string_property(&v))
        .unwrap_or_else(|| "".to_string());
    let window_pid = pid
        .map(|v| parse_cardinal_property(&v))
        .unwrap_or_else(|| 0);
    let (instance, class) = class
        .map(|v| parse_wm_class(&v))
        .unwrap_or_else(|| ("".to_string(), "".to_string()));

    // Get the command
    let command =
        fs::read_to_string(format!("/proc/{}/cmdline", window_pid)).unwrap_or("".to_string());

    // Ungrap the server
    // conn.ungrab_server().map_err(|e| Error::WMError(X11Error::ConnectionError(e)))?;
    Some(WindowData {
        window: focus,
        window_name,
        class,
        command,
        pid: window_pid,
        instance,
    })
}

fn get_property(
    conn: &RustConnection,
    window: Window,
    property: Atom,
    type_: Atom,
) -> Option<GetPropertyReply> {
    conn.get_property(false, window, property, type_, 0, u32::MAX)
        .ok()?
        .reply()
        .ok()
}

fn get_or_intern_atom(conn: &RustConnection, name: &[u8]) -> Atom {
    let result = conn
        .intern_atom(false, name)
        .expect("Failed to intern atom")
        .reply()
        .expect("Failed receive interned atom");

    result.atom
}

fn find_active_window(
    conn: &impl Connection,
    root: Window,
    net_active_window: Atom,
) -> Option<Window> {
    let window: Atom = AtomEnum::WINDOW.into();
    let active_window = conn
        .get_property(false, root, net_active_window, window, 0, 1)
        .ok()?
        .reply()
        .ok()?;

    if active_window.format == 32 && active_window.length == 1 {
        active_window
            .value32()
            .expect("Invalid message. Expected value with format = 32")
            .next()
    } else {
        // Query the input focus
        Some(conn.get_input_focus().ok()?.reply().ok()?.focus)
    }
}

fn parse_string_property(property: &GetPropertyReply) -> String {
    std::str::from_utf8(&property.value)
        .unwrap_or("Invalid utf8")
        .to_string()
}

fn parse_cardinal_property(property: &GetPropertyReply) -> u32 {
    let mut res: u32 = 0;
    for index in 0..property.value.len() {
        res = res * 256 + property.value[(property.value.len() - index - 1)] as u32;
    }
    res
}

fn parse_wm_class(property: &GetPropertyReply) -> (String, String) {
    if property.format != 8 {
        return (
            "Malformed property: wrong format".to_string(),
            "Malformed property: wrong format".to_string(),
        );
    }
    let value = &property.value;
    // The property should contain two null-terminated strings. Find them.
    if let Some(middle) = value.iter().position(|&b| b == 0) {
        let (instance, class) = value.split_at(middle);
        // Skip the null byte at the beginning
        let mut class = &class[1..];
        // Remove the last null byte from the class, if it is there.
        if class.last() == Some(&0) {
            class = &class[..class.len() - 1];
        }
        let instance = std::str::from_utf8(instance);
        let class = std::str::from_utf8(class);
        (
            instance.unwrap_or("Invalid utf8").to_string(),
            class.unwrap_or("Invalid utf8").to_string(),
        )
    } else {
        (
            "Missing null byte".to_string(),
            "Missing null byte".to_string(),
        )
    }
}
