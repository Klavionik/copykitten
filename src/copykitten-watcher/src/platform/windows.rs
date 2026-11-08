use crate::{Event, Platform};
use log::debug;
use std::fmt::Debug;
use windows::Win32::Foundation::HWND;
use windows::Win32::System::DataExchange::{
    AddClipboardFormatListener, RemoveClipboardFormatListener,
};
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DestroyWindow, DispatchMessageW, GetMessageW, HWND_MESSAGE, MSG,
    TranslateMessage, WM_CLIPBOARDUPDATE,
};
use windows::core::w;

#[derive(Debug)]
pub struct Windows {
    // TODO: HWND isn't Send. Store as isize?
    hwnd: HWND,
}

impl Windows {
    pub fn new() -> Self {
        debug!("Create Windows platform");

        // Create a UTF-16 string with the name of the window class.
        // Here we use the built-in system Static class. That way we
        // don't have to register our own window class and
        // define a window procedure, making things easier.
        let class_name = w!("STATIC");

        let hwnd = unsafe {
            // Create a hidden window that only listens to messages and get a handle to it.
            let h_window = CreateWindowExW(
                Default::default(), // Skip the styles.
                class_name,         // The window class name.
                w!(""),             // No need for a window title.
                Default::default(), // Skip the styles, again.
                0,
                0,
                0,
                0,                  // No need for position.
                Some(HWND_MESSAGE), // Mark this window as message-only.
                None,               // No menu.
                None,               // No specific application.
                None,               // No extra params.
            )
            .unwrap();

            // Subscribe our dummy window to the clipboard content changes.
            AddClipboardFormatListener(h_window).unwrap();

            h_window
        };

        Self { hwnd }
    }
}

impl Default for Windows {
    fn default() -> Self {
        Self::new()
    }
}

struct Message(MSG);

impl Drop for Message {
    fn drop(&mut self) {
        unsafe {
            let _ = TranslateMessage(&self.0);
            DispatchMessageW(&self.0);
        }
    }
}

impl Platform for Windows {
    fn listen(&self) -> impl Iterator<Item = Event> {
        debug!("Start listening");
        // Allocate memory for incoming messages.
        let mut msg = MSG::default();

        std::iter::from_fn(move || {
            // Block until a new WM_CLIPBOARDUPDATE message arrive.
            let ok = unsafe {
                GetMessageW(
                    &mut msg,
                    Some(self.hwnd),
                    WM_CLIPBOARDUPDATE,
                    WM_CLIPBOARDUPDATE,
                )
                .as_bool()
            };
            let msg = Message(msg);
            debug!("Received new message");

            if ok {
                match msg.0.message {
                    WM_CLIPBOARDUPDATE => {
                        debug!("Clipboard update event");
                        let event = Event::change();
                        return Some(event);
                    }
                    _ => {
                        debug!("Another type of message: {:?}", msg.0.message)
                    }
                }
            }

            None
        })
    }
}

impl Drop for Windows {
    fn drop(&mut self) {
        unsafe {
            let _ = RemoveClipboardFormatListener(self.hwnd);
            let _ = DestroyWindow(self.hwnd);
        }
    }
}
