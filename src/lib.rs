extern crate core;

use pyo3::create_exception;
use pyo3::prelude::*;
use std::sync::{Mutex, MutexGuard, OnceLock};

create_exception!(copykitten, CopykittenError, pyo3::exceptions::PyException);

static CLIPBOARD: OnceLock<Mutex<arboard::Clipboard>> = OnceLock::new();

fn raise_exc(text: &'static str) -> PyErr {
    CopykittenError::new_err(text)
}

fn to_exc(err: arboard::Error) -> PyErr {
    CopykittenError::new_err(err.to_string())
}

fn get_clipboard() -> Result<MutexGuard<'static, arboard::Clipboard>, PyErr> {
    CLIPBOARD
        .get()
        .ok_or(raise_exc("Clipboard was never initialized."))?
        .lock()
        .map_err(|_| raise_exc("Cannot get lock on the clipboard, the lock is poisoned."))
}

#[pyfunction]
fn copy(content: &str) -> PyResult<()> {
    let mut cb = get_clipboard()?;

    cb.set_text(content).map_err(to_exc)?;
    Ok(())
}

#[pyfunction]
fn paste() -> PyResult<String> {
    let mut cb = get_clipboard()?;
    let content = cb.get_text().map_err(to_exc)?;

    Ok(content)
}

#[pyfunction]
fn clear() -> PyResult<()> {
    let mut cb = get_clipboard()?;

    cb.clear().map_err(to_exc)
}

#[pymodule]
fn _copykitten(py: Python, module: &PyModule) -> PyResult<()> {
    let clipboard =
        arboard::Clipboard::new().map_err(|_| raise_exc("Cannot initialize clipboard"))?;
    CLIPBOARD
        .set(Mutex::new(clipboard))
        .map_err(|_| raise_exc("Global clipboard already created."))?;
    module.add("CopykittenError", py.get_type::<CopykittenError>())?;
    module.add_function(wrap_pyfunction!(copy, module)?)?;
    module.add_function(wrap_pyfunction!(paste, module)?)?;
    module.add_function(wrap_pyfunction!(clear, module)?)?;
    Ok(())
}
