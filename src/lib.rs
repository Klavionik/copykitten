extern crate core;

use std::{time, thread};
use arboard::Clipboard;
use pyo3::create_exception;
use pyo3::prelude::*;

create_exception!(copykitten, CopykittenError, pyo3::exceptions::PyException);

fn get_clipboard() -> Result<Clipboard, PyErr> {
    return match Clipboard::new() {
        Ok(clipboard) => Ok(clipboard),
        Err(e) => Err(CopykittenError::new_err("Cannot initialize clipboard.")),
    };
}

#[pyfunction]
fn copy(content: &str) -> PyResult<()> {
    let mut cb = get_clipboard().unwrap();

    cb.set_text(content).unwrap();
    thread::sleep(time::Duration::from_millis(50));
    Ok(())
}

#[pyfunction]
fn paste() -> PyResult<String> {
    let mut cb = get_clipboard().unwrap();

    return match cb.get_text() {
        Ok(text) => Ok(text),
        Err(_e) => Err(CopykittenError::new_err("Cannot paste content.")),
    };
}

#[pyfunction]
fn clear() -> PyResult<()> {
    let mut cb = get_clipboard().unwrap();

    return match cb.clear() {
        Ok(result) => Ok(result),
        Err(_e) => Err(CopykittenError::new_err("Cannot clear clipboard.")),
    };
}

#[pymodule]
fn copykitten(py: Python, module: &PyModule) -> PyResult<()> {
    module.add("CopykittenError", py.get_type::<CopykittenError>())?;
    module.add_function(wrap_pyfunction!(copy, module)?)?;
    module.add_function(wrap_pyfunction!(paste, module)?)?;
    module.add_function(wrap_pyfunction!(clear, module)?)?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use std::process::{Command, Output};
    use super::*;

    fn read_clipboard() -> Output {
        return Command::new("xsel")
            .arg("-b")
            .output()
            .expect("xsel failure");
    }

    #[test]
    fn test_add() {
        for i in 0..100 {
            let text = format!("text{}", i);
            copy(&text).unwrap();
            thread::sleep(time::Duration::from_millis(100));

            let output = read_clipboard();
            assert_eq!(text, String::from_utf8_lossy(&output.stdout));
        }
    }
}