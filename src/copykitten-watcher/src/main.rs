use copykitten_watcher::{platform::X11, watch};

fn main() {
    watch(|_e| println!("wow"), X11::new())
}
