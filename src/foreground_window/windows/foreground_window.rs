use windows::core::PWSTR;
use windows::Win32::Foundation::*;
use windows::Win32::System::Threading::{
    OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_WIN32, PROCESS_QUERY_LIMITED_INFORMATION,
};
use windows::Win32::UI::Accessibility::*;

use super::super::{Error, Win32Error, WindowInformation};
use windows::Win32::UI::WindowsAndMessaging::*;

/// The callback function we use to get informed from windows!
static mut WINDOW_FOREGROUND_CALLBACK: Option<Box<dyn Fn(HWND) -> ()>> = None;

extern "system" fn callback(
    _hwineventhook: HWINEVENTHOOK,
    _event: u32,
    hwnd: HWND,
    _idobject: i32,
    _idchild: i32,
    _ideventthread: u32,
    _dwmseventtime: u32,
) {
    unsafe {
        if let Some(cb) = &WINDOW_FOREGROUND_CALLBACK {
            cb(hwnd)
        }
    }
}

/// Helper function, returning the title of a window
fn get_window_title(hwnd: &HWND) -> Result<String, Error> {
    let mut text: [u16; 512] = [0; 512];

    let len = unsafe { GetWindowTextW(hwnd, PWSTR(text.as_mut_ptr()), text.len() as i32) };
    Ok(String::from_utf16_lossy(&text[..len as usize]))
}

/// Helper function, returning the class name of a window
fn get_window_class_name(hwnd: &HWND) -> Result<String, Error> {
    let mut text: [u16; 512] = [0; 512];

    let len = unsafe { GetClassNameW(hwnd, PWSTR(text.as_mut_ptr()), text.len() as i32) };
    Ok(String::from_utf16_lossy(&text[..len as usize]))
}

/// Helper function, returning the executable of a window
fn get_window_executable_name(hwnd: &HWND) -> Result<String, Error> {
    let mut text: [u16; 512] = [0; 512];
    let mut process_name_length: u32 = 512;
    let mut process_id: u32 = 0;

    unsafe {
        GetWindowThreadProcessId(hwnd, &mut process_id);
        let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, true, process_id);
        if !QueryFullProcessImageNameW(
            handle,
            PROCESS_NAME_WIN32,
            PWSTR(text.as_mut_ptr()),
            &mut process_name_length,
        )
        .as_bool()
        {
            return Err(Error::WMError(Win32Error::UnspecificError));
        }
    }
    Ok(String::from_utf16_lossy(
        &text[..process_name_length as usize],
    ))
}

/// Observe foreground window.
///
/// The callback will be called whenever the foreground changes
/// and ones on initialization!
///
/// # Arguments
///
/// cb - The Callback function to be called when a new window gets focus.
pub fn foreground_window_observer<F>(cb: F) -> Result<(), Error>
where
    F: Fn(WindowInformation),
    F: 'static,
{
    unsafe {
        if WINDOW_FOREGROUND_CALLBACK.is_some() {
            return Err(Error::AlreadyStarted);
        }
        WINDOW_FOREGROUND_CALLBACK = Some(Box::new(move |hwnd| {
            let title = get_window_title(&hwnd).unwrap_or_else(|_| "".to_string());
            let executable = get_window_executable_name(&hwnd).unwrap_or_else(|_| "".to_string());
            let class_name = get_window_class_name(&hwnd).unwrap_or_else(|_| "".to_string());
            cb(WindowInformation {
                title,
                executable,
                class_name,
            });
        }));

        // Register the callback
        if let ::windows::core::Result::Err(e) = SetWinEventHook(
            EVENT_SYSTEM_FOREGROUND,
            EVENT_SYSTEM_FOREGROUND,
            HINSTANCE { 0: 0 },
            Some(callback),
            0,
            0,
            WINEVENT_OUTOFCONTEXT | WINEVENT_SKIPOWNPROCESS,
        )
        .ok()
        {
            return Err(Error::WMError(Win32Error::CoreError(e)));
        };

        let mut msg: MSG = MSG {
            hwnd: Default::default(),
            message: 0,
            wParam: Default::default(),
            lParam: Default::default(),
            time: 0,
            pt: Default::default(),
        };

        while GetMessageW(&mut msg, HWND { 0: 0 }, 0, 0).as_bool() {
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }

        WINDOW_FOREGROUND_CALLBACK = None;
    }

    Ok(())
}
