use crate::{Event as OutEvent, Platform};
use log::debug;
use std::fmt::{Debug, Formatter};
use x11rb::connection::Connection;
use x11rb::protocol::xfixes::{ConnectionExt as _, SelectionEventMask};
use x11rb::protocol::xproto::{Atom, ConnectionExt as _, CreateWindowAux, WindowClass};
use x11rb::protocol::{Event, xfixes};
use x11rb::{COPY_DEPTH_FROM_PARENT, connect};

use x11rb::rust_connection::RustConnection;

pub struct X11 {
    conn: RustConnection,
    window_id: u32,
}

impl Platform for X11 {
    fn listen(&self) -> impl Iterator<Item = OutEvent> {
        let clipboard = self.get_clipboard_atom();

        self.conn
            .xfixes_query_version(5, 0)
            .unwrap()
            .reply()
            .unwrap();

        xfixes::select_selection_input(
            &self.conn,
            self.window_id,
            clipboard,
            SelectionEventMask::SET_SELECTION_OWNER
                | SelectionEventMask::SELECTION_WINDOW_DESTROY
                | SelectionEventMask::SELECTION_CLIENT_CLOSE,
        )
        .unwrap();

        self.conn.flush().unwrap();

        debug!("All set up, start listening");

        std::iter::from_fn(|| {
            debug!("Waiting for event...");

            match self.conn.wait_for_event().unwrap() {
                Event::XfixesSelectionNotify(event) => {
                    debug!("Received SelectionNotify: {:?}", event);
                    Some(OutEvent::change())
                }
                event => {
                    debug!("Received event {:?}", event);
                    None
                }
            }
        })
    }
}

impl X11 {
    pub fn new() -> Self {
        let (conn, screen_num) = connect(None).unwrap();
        let screen = &conn.setup().roots.get(screen_num).unwrap();
        let win_id = conn.generate_id().unwrap();
        conn.create_window(
            COPY_DEPTH_FROM_PARENT,
            win_id,
            screen.root,
            0,
            0,
            1,
            1,
            0,
            WindowClass::INPUT_OUTPUT,
            screen.root_visual,
            &CreateWindowAux::new(),
        )
        .unwrap()
        .check()
        .unwrap();

        debug!("Acquired X11 connection and created a dummy window");

        Self {
            window_id: win_id,
            conn,
        }
    }

    fn get_clipboard_atom(&self) -> Atom {
        self.conn
            .intern_atom(false, b"CLIPBOARD")
            .unwrap()
            .reply()
            .unwrap()
            .atom
    }
}

impl Default for X11 {
    fn default() -> Self {
        Self::new()
    }
}

impl Debug for X11 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "X11 {{ conn: RustConnection {{ .. }}, window_id: {} }}",
            self.window_id
        )
    }
}
