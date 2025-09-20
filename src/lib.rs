extern crate core;

#[cfg(target_os = "linux")]
use arboard::SetExtLinux;
#[cfg(target_os = "linux")]
use daemonize::{Daemonize, Outcome};
use pyo3::create_exception;
use pyo3::prelude::*;
use std::borrow::Cow;
use std::fs::File;
use std::sync::{LazyLock, Mutex, MutexGuard};

create_exception!(copykitten, CopykittenError, pyo3::exceptions::PyException);

static CLIPBOARD: LazyLock<Option<Mutex<arboard::Clipboard>>> = LazyLock::new(initialize_clipboard);

fn initialize_clipboard() -> Option<Mutex<arboard::Clipboard>> {
    let maybe_clipboard = arboard::Clipboard::new().ok();
    maybe_clipboard.map(Mutex::new)
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
    let image = arboard::ImageData {
        bytes: content,
        width,
        height,
    };

    let mut cb = get_clipboard()?;
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
fn clear() -> PyResult<()> {
    let mut cb = get_clipboard()?;

    cb.clear().map_err(to_exc)
}

#[pymodule]
fn _copykitten(py: Python, module: &PyModule) -> PyResult<()> {
    module.add("CopykittenError", py.get_type::<CopykittenError>())?;
    module.add_function(wrap_pyfunction!(copy, module)?)?;
    module.add_function(wrap_pyfunction!(copy_wait, module)?)?;
    module.add_function(wrap_pyfunction!(paste, module)?)?;
    module.add_function(wrap_pyfunction!(clear, module)?)?;
    module.add_function(wrap_pyfunction!(copy_image, module)?)?;
    module.add_function(wrap_pyfunction!(copy_image_wait, module)?)?;
    module.add_function(wrap_pyfunction!(paste_image, module)?)?;
    Ok(())
}
