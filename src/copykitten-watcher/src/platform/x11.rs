use crate::{Event as OutEvent, Platform};
use x11rb::connection::Connection;
use x11rb::protocol::xfixes::SelectionEventMask;
use x11rb::protocol::xproto::{Atom, ConnectionExt, CreateWindowAux, WindowClass};
use x11rb::protocol::{Event, xfixes};
use x11rb::{COPY_DEPTH_FROM_PARENT, connect};

use x11rb::rust_connection::RustConnection;

pub struct X11 {
    conn: RustConnection,
    window_id: u32,
}

impl Platform for X11 {
    fn listen(&self) -> impl Iterator<Item = OutEvent> {
        std::iter::from_fn(|| {
            let clipboard = self.get_clipboard_atom();

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

            loop {
                if let Event::XfixesSelectionNotify(e) = self.conn.wait_for_event().unwrap() {
                    println!("clipboard changed, new owner window: {}", e.owner);
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
