use crate::{Event, platform::Listener};
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
    hwnd: Option<isize>,
}

impl Windows {
    pub fn new() -> Self {
        debug!("Create Windows platform");

        Self { hwnd: None }
    }

    pub fn setup(&mut self) -> Result<(), anyhow::Error> {
        // Create a UTF-16 string with the name of the window class.
        // Here we use the built-in system Static class. That way we
        // don't have to register our own window class and
        // define a window procedure, making things easier.
        let class_name = w!("STATIC");

        let hwnd = unsafe {
            // Create a hidden window that only listens to messages and get a handle to it.
            let hwnd = CreateWindowExW(
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
            )?;

            // Subscribe our dummy window to the clipboard content changes.
            AddClipboardFormatListener(hwnd)?;

            hwnd
        };

        // Store the actual window handle.
        self.hwnd = Some(hwnd.0 as _);

        Ok(())
    }

    fn hwnd(&self) -> Option<HWND> {
        self.hwnd.map(|value| HWND(value as _))
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

impl Listener for Windows {
    fn next_event(&mut self) -> Option<Event> {
        // Allocate memory for the incoming message.
        let mut msg = MSG::default();

        // Block until a new WM_CLIPBOARDUPDATE message arrive.
        let ok = unsafe {
            GetMessageW(
                &mut msg,
                self.hwnd(),
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
    }
}

impl Drop for Windows {
    fn drop(&mut self) {
        unsafe {
            if let Some(hwnd) = self.hwnd() {
                let _ = RemoveClipboardFormatListener(hwnd);
                let _ = DestroyWindow(hwnd);
            }
        }
    }
}
