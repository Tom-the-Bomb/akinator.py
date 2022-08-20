//! Python bindings for ``akinator-rs``, a wrapper around the undocumented akinator API
//!
//! designed for easy implementation of an akinator game in code, providing a simple and easy to use API.

use crate::{
    enums::{
        Theme,
        Answer,
        Language,
    },
    async_akinator::AsyncAkinator,
    blocking_akinator::Akinator,
    models::Guess,
};

use pyo3::prelude::*;

pub mod blocking_akinator;
pub mod async_akinator;
pub mod enums;
pub mod error;
pub mod models;


/// Python bindings for ``akinator-rs``, a wrapper around the undocumented akinator API
///
/// designed for easy implementation of an akinator game in code, providing a simple and easy to use API.
#[pymodule]
fn akinator(py: Python<'_>, module: &PyModule) -> PyResult<()> {
    module.add_class::<AsyncAkinator>()?;
    module.add_class::<Akinator>()?;
    module.add_class::<Guess>()?;

    module.add_class::<Theme>()?;
    module.add_class::<Answer>()?;
    module.add_class::<Language>()?;

    error::add_exceptions(py, module)?;

    Ok(())
}
