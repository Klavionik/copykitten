use copykitten_watcher::{Watcher, platform::X11};
use flexi_logger::{Logger, WriteMode, opt_format};
use log::debug;

fn main() {
    let _logger = Logger::try_with_str("copykitten_watcher=debug")
        .unwrap()
        .format(opt_format)
        .write_mode(WriteMode::BufferAndFlush)
        .start()
        .unwrap();

    let handle = Watcher::new(|e| debug!("{e:?}"), X11::new()).spawn();

    handle.join().unwrap();
}
