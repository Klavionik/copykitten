use copykitten_watcher::{Watcher, platform::Windows};
use flexi_logger::{Logger, WriteMode, opt_format};
use log::debug;

fn main() {
    let _logger = Logger::try_with_str("copykitten_watcher=debug")
        .unwrap()
        .format(opt_format)
        .write_mode(WriteMode::BufferAndFlush)
        .start()
        .unwrap();

    let _handle = Watcher::new(|e| debug!("{e:?}"), Windows::new()).run();

    // handle.join().unwrap();
}
