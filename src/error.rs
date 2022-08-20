use pyo3::{
    prelude::*,
    create_exception,
    exceptions::{
        PyException,
        PyValueError,
        PyRuntimeError,
    }
};

use std::sync::PoisonError;
use akinator_rs::error::Error as AkiError;


macro_rules! create_exceptions {
    ( $(( $name:ident, $doc:expr )),* $(,)* ) => {
        $(
            create_exception!(
                akinator,
                $name,
                PyException,
                $doc
            );
        )*
    }
}

create_exceptions![
    (CantGoBackAnyFurther, "Raised when the akinator is already on the 1st question / there are no more questions to go back on"),
    (InvalidAnswer, "Raised when an invalid answer string is used when instantiating a Language enum from str"),
    (InvalidLanguage, "Raised when an invalid language string is used when instantiating a Language enum from str"),
    (ConnectionError, "Raised when we fail the connect to the akinator servers for whatever reason"),
    (NoMoreQuestions, "Raised when there are no more questions the akinator can offer"),
    (TimeoutError, "Raised when the akinator session timed out waiting for a response"),
    (TechnicalError, "Raised when there is a technical internal error with the akinator servers"),
    (ServersDown, "Raised when the akinator servers in the requested region are down"),
];

#[derive(Debug)]
pub enum Error {
    AkiError(AkiError),
    PoisonError,
}

pub(crate) fn add_exceptions(py: Python<'_>, module: &PyModule) -> PyResult<()> {
    module.add("CantGoBackAnyFurther", py.get_type::<CantGoBackAnyFurther>())?;
    module.add("InvalidAnswer", py.get_type::<InvalidAnswer>())?;
    module.add("InvalidLanguage", py.get_type::<InvalidLanguage>())?;
    module.add("ConnectionError", py.get_type::<ConnectionError>())?;
    module.add("NoMoreQuestions", py.get_type::<NoMoreQuestions>())?;
    module.add("TimeoutError", py.get_type::<TimeoutError>())?;
    module.add("TechnicalError", py.get_type::<TechnicalError>())?;
    module.add("ServersDown", py.get_type::<ServersDown>())?;

    Ok(())
}

impl From<Error> for PyErr {
    fn from(error: Error) -> Self {
        match error {
            Error::AkiError(err) => match err {
                AkiError::CantGoBackAnyFurther =>
                    CantGoBackAnyFurther::new_err("Cannot go back any more questions, we are already on the 0th question"),
                AkiError::InvalidAnswer =>
                    InvalidAnswer::new_err("Invalid answer string"),
                AkiError::InvalidLanguage =>
                    InvalidLanguage::new_err("Invalid language string"),
                AkiError::ConnectionError =>
                    ConnectionError::new_err("Failed to connect to akinator servers"),
                AkiError::NoMoreQuestions =>
                    NoMoreQuestions::new_err("The akinator has no more questions to ask"),
                AkiError::TimeoutError =>
                    TimeoutError::new_err("The akinator session timed out"),
                AkiError::TechnicalError =>
                    TechnicalError::new_err("An unknown technical error occured within the akinator servers"),
                AkiError::ServersDown =>
                    ServersDown::new_err("The requested akinator servers are down"),
                AkiError::NoDataFound  | AkiError::UpdateInfoError(_) |
                AkiError::TimeError(_) | AkiError::RequestError(_) =>
                    PyRuntimeError::new_err(err.to_string()),
                AkiError::JsonParseError(_) =>
                    PyValueError::new_err(err.to_string()),
            },
            Error::PoisonError =>
                PyRuntimeError::new_err("Failed to read attribute values"),
        }
    }
}

impl<T> From<PoisonError<T>> for Error {
    fn from(_error: PoisonError<T>) -> Self {
        Self::PoisonError
    }
}

impl From<AkiError> for Error {
    fn from(error: AkiError) -> Self {
        Self::AkiError(error)
    }
}