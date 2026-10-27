extern crate clipboard_master;
extern crate core;
use clipboard_master::{CallbackResult, ClipboardHandler, Master};

#[cfg(target_os = "linux")]
use arboard::SetExtLinux;
use chrono::{DateTime, Utc};
#[cfg(target_os = "linux")]
use daemonize::{Daemonize, Outcome};
use flexi_logger::{FileSpec, Logger, WriteMode, opt_format};
use log::{debug, error};
use pyo3::create_exception;
use pyo3::prelude::*;
use pyo3::types::PyFunction;
use std::borrow::Cow;
use std::collections::HashMap;
#[cfg(target_os = "linux")]
use std::fs::File;
use std::sync::{LazyLock, Mutex, MutexGuard, OnceLock};
use std::time::Duration;
use std::{io, thread};
use uuid::Uuid;

create_exception!(copykitten, CopykittenError, pyo3::exceptions::PyException);

static CLIPBOARD: LazyLock<Option<Mutex<arboard::Clipboard>>> = LazyLock::new(initialize_clipboard);
static CALLBACKS: OnceLock<Mutex<HashMap<Uuid, Py<PyFunction>>>> = OnceLock::new();

struct Handler {}

#[pyclass]
struct ChangeEvent {
    created_at: DateTime<Utc>,
}

impl ChangeEvent {
    fn new() -> Self {
        Self {
            created_at: Utc::now(),
        }
    }
}

#[pymethods]
impl ChangeEvent {
    fn __repr__(&self) -> String {
        format!("ChangeEvent(created_at={})", self.created_at.to_rfc3339())
    }
}

fn run_callback(py: Python, callback: (&Uuid, &Py<PyFunction>)) {
    let (id, func) = callback;
    let event = ChangeEvent::new();

    debug!("Firing callback {}.", id);

    match func.call1(py, (event,)) {
        Ok(_) => debug!("Callback complete."),
        Err(e) => debug!("Callback error: {:?}.", e),
    }
}

fn spawn_event_master() {
    debug!("Spawning event master.");
    let handler = Handler {};

    match Master::new(handler) {
        Ok(mut master) => {
            debug!("Running event master.");

            match master.run() {
                Ok(_) => debug!("Event master complete."),
                Err(e) => debug!("Event master error: {:?}", e),
            }
        }
        Err(err) => error!("Couldn't start event master: {:?}.", err),
    }
}

impl ClipboardHandler for Handler {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        debug!("Received change event.");

        match CALLBACKS.get_or_init(initialize_callbacks).lock() {
            Ok(guard) => Python::attach(|py| {
                if guard.is_empty() {
                    debug!("No callbacks registered, skip event.");
                    return;
                }

                guard.iter().for_each(|callback| run_callback(py, callback));
            }),
            Err(_) => error!("Callbacks lock is poisoned."),
        }

        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: io::Error) -> CallbackResult {
        debug!("Received clipboard error: {}", error);
        CallbackResult::Next
    }

    fn sleep_interval(&self) -> Duration {
        Duration::from_millis(100)
    }
}

fn initialize_clipboard() -> Option<Mutex<arboard::Clipboard>> {
    let maybe_clipboard = arboard::Clipboard::new().ok();
    maybe_clipboard.map(Mutex::new)
}

fn initialize_callbacks() -> Mutex<HashMap<Uuid, Py<PyFunction>>> {
    Mutex::new(HashMap::new())
}

fn raise_exc(text: &'static str) -> PyErr {
    CopykittenError::new_err(text)
}

fn to_exc<E: ToString>(err: E) -> PyErr {
    CopykittenError::new_err(err.to_string())
}

fn get_clipboard() -> Result<MutexGuard<'static, arboard::Clipboard>, PyErr> {
    let mutex = CLIPBOARD
        .as_ref()
        .ok_or(raise_exc("Clipboard was never initialized."))?;

    mutex
        .lock()
        .map_err(|_| raise_exc("Cannot get lock on the clipboard, the lock is poisoned."))
}

#[cfg(target_os = "linux")]
fn with_daemon<F: FnOnce()>(func: F) -> PyResult<()> {
    let stderr = File::create("/tmp/copykitten-daemon").map_err(|e| to_exc(e.to_string()))?;
    let daemon = Daemonize::new().stderr(stderr);

    match daemon.execute() {
        Outcome::Child(Ok(_)) => {
            func();
            Ok(())
        }
        Outcome::Parent(Err(e)) => Err(to_exc(e.to_string())),
        Outcome::Child(Err(e)) => Err(to_exc(e.to_string())),
        Outcome::Parent(Ok(_)) => Ok(()),
    }
}

#[pyfunction]
fn copy(content: &str) -> PyResult<()> {
    let mut cb = get_clipboard()?;

    cb.set_text(content).map_err(to_exc)?;
    Ok(())
}

#[cfg(target_os = "linux")]
#[pyfunction]
fn copy_wait(content: &str) -> PyResult<()> {
    with_daemon(|| {
        let mut cb = arboard::Clipboard::new().unwrap();
        cb.set().wait().text(content).unwrap();
    })?;

    Ok(())
}

#[cfg(not(target_os = "linux"))]
#[pyfunction]
fn copy_wait(content: &str) -> PyResult<()> {
    copy(content)
}

#[pyfunction]
fn copy_image(content: Cow<[u8]>, width: usize, height: usize) -> PyResult<()> {
    let mut cb = get_clipboard()?;
    let image = arboard::ImageData {
        bytes: content,
        width,
        height,
    };

    cb.set_image(image).map_err(to_exc)?;
    Ok(())
}

#[cfg(target_os = "linux")]
#[pyfunction]
fn copy_image_wait(content: Cow<[u8]>, width: usize, height: usize) -> PyResult<()> {
    let image = arboard::ImageData {
        bytes: content,
        width,
        height,
    };

    with_daemon(|| {
        let mut cb = arboard::Clipboard::new().unwrap();
        cb.set().wait().image(image).unwrap();
    })?;

    Ok(())
}

#[cfg(not(target_os = "linux"))]
#[pyfunction]
fn copy_image_wait(content: Cow<[u8]>, width: usize, height: usize) -> PyResult<()> {
    copy_image(content, width, height)
}

#[pyfunction]
fn copy_file_list(file_list: Vec<std::path::PathBuf>) -> PyResult<()> {
    let mut cb = get_clipboard()?;

    cb.set().file_list(&file_list).map_err(to_exc)
}

#[cfg(not(target_os = "linux"))]
#[pyfunction]
fn copy_file_list_wait(file_list: Vec<std::path::PathBuf>) -> PyResult<()> {
    copy_file_list(file_list)
}

#[cfg(target_os = "linux")]
#[pyfunction]
fn copy_file_list_wait(file_list: Vec<std::path::PathBuf>) -> PyResult<()> {
    with_daemon(|| {
        let mut cb = arboard::Clipboard::new().unwrap();
        cb.set().wait().file_list(&file_list).unwrap();
    })?;

    Ok(())
}

#[pyfunction]
fn paste() -> PyResult<String> {
    let mut cb = get_clipboard()?;
    let content = cb.get_text().map_err(to_exc)?;

    Ok(content)
}

#[pyfunction]
fn paste_image() -> PyResult<(Cow<'static, [u8]>, usize, usize)> {
    let mut cb = get_clipboard()?;
    let image = cb.get_image().map_err(to_exc)?;

    Ok((image.bytes, image.width, image.height))
}

#[pyfunction]
fn paste_file_list() -> PyResult<Vec<std::path::PathBuf>> {
    let mut cb = get_clipboard()?;
    let file_list = cb.get().file_list().map_err(to_exc)?;

    Ok(file_list)
}

#[pyfunction]
fn clear() -> PyResult<()> {
    let mut cb = get_clipboard()?;

    cb.clear().map_err(to_exc)
}

#[pyfunction]
fn listen(callback: Py<PyFunction>) -> PyResult<Uuid> {
    let mut callbacks = CALLBACKS
        .get_or_init(initialize_callbacks)
        .lock()
        .map_err(|_| raise_exc("Cannot add a new callback, lock is poisoned."))?;

    let id = Uuid::new_v4();
    callbacks.insert(id, callback);

    debug!("Added a new callback {}", id);
    Ok(id)
}

#[pyfunction]
fn unlisten(id: Uuid) -> PyResult<bool> {
    let mut callbacks = CALLBACKS
        .get_or_init(initialize_callbacks)
        .lock()
        .map_err(|_| raise_exc("Cannot remove a callback, lock is poisoned."))?;

    let removed = callbacks.remove(&id).is_some();
    debug!("Removed callback {}, success = {}", id, removed);

    Ok(removed)
}

#[pymodule]
fn _copykitten(module: &Bound<'_, PyModule>) -> PyResult<()> {
    let _logger = Logger::try_with_str("copykitten=debug")
        .unwrap()
        .log_to_file(FileSpec::default())
        .format(opt_format)
        .write_mode(WriteMode::BufferAndFlush)
        .start()
        .unwrap();

    module.add("CopykittenError", module.py().get_type::<CopykittenError>())?;
    module.add_function(wrap_pyfunction!(copy, module)?)?;
    module.add_function(wrap_pyfunction!(copy_wait, module)?)?;
    module.add_function(wrap_pyfunction!(paste, module)?)?;
    module.add_function(wrap_pyfunction!(clear, module)?)?;
    module.add_function(wrap_pyfunction!(copy_image, module)?)?;
    module.add_function(wrap_pyfunction!(copy_image_wait, module)?)?;
    module.add_function(wrap_pyfunction!(copy_file_list, module)?)?;
    module.add_function(wrap_pyfunction!(copy_file_list_wait, module)?)?;
    module.add_function(wrap_pyfunction!(paste_image, module)?)?;
    module.add_function(wrap_pyfunction!(paste_file_list, module)?)?;
    module.add_function(wrap_pyfunction!(listen, module)?)?;
    module.add_function(wrap_pyfunction!(unlisten, module)?)?;
    module.add_class::<ChangeEvent>()?;

    thread::spawn(spawn_event_master);

    Ok(())
}
