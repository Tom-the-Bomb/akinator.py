use akinator_rs::Akinator as AkinatorStruct;
use std::sync::{Arc, RwLock};
use pyo3::prelude::*;
use pyo3_asyncio::tokio::local_future_into_py as to_coro;

use crate::models::Guess;
use crate::error::Error;
use crate::enums::{
    Theme,
    Answer,
    Language,
};

pub mod enums;
pub mod error;
pub mod models;

#[pyclass]
#[derive(Debug, Clone)]
pub struct Akinator(
    Arc<RwLock<AkinatorStruct>>,
);

#[pymethods]
impl Akinator {
    #[new]
    #[args("*", theme, language, child_mode)]
    fn constructor(
        theme: Option<Theme>,
        language: Option<Language>,
        child_mode: Option<bool>,
    ) -> PyResult<Self> {
        let mut akinator =
            AkinatorStruct::new();

        if let Some(theme) = theme {
            akinator = akinator.with_theme(theme.into());
        }

        if let Some(language) = language {
            akinator = akinator.with_language(language.into());
        }

        if child_mode.unwrap_or(false) {
            akinator = akinator.with_child_mode();
        }

        Ok(Self(
            Arc::new(RwLock::new(akinator))
        ))
    }

    fn start_game<'a>(&'a mut self, py: Python<'a>) -> PyResult<&'a pyo3::PyAny> {
        let cloned = self.0.clone();

        to_coro(py, async move {
            let mut writer = cloned.write()
                .map_err(|e| Error::from(e))?;

            writer.start().await
                .map_err(|e| Error::from(e).into())
        })
    }

    fn answer<'a>(&'a mut self, py: Python<'a>, answer: Answer) -> PyResult<&'a pyo3::PyAny> {
        let cloned = self.0.clone();

        to_coro(py, async move {
            let mut writer = cloned.write()
                .map_err(|e| Error::from(e))?;

            writer.answer(answer.into()).await
                .map_err(|e| Error::from(e).into())
        })
    }

    fn win<'a>(&'a mut self, py: Python<'a>) -> PyResult<&'a pyo3::PyAny> {
        let cloned = self.0.clone();

        to_coro(py, async move {
            let mut writer = cloned.write()
                .map_err(|e| Error::from(e))?;

            writer.win().await
                .map(|result| {
                    result.map(|g| Guess(g))
                })
                .map_err(|e| Error::from(e).into())
        })
    }

    fn back<'a>(&'a mut self, py: Python<'a>) -> PyResult<&'a pyo3::PyAny> {
        let cloned = self.0.clone();

        to_coro(py, async move {
            let mut writer = cloned.write()
                .map_err(|e| Error::from(e))?;

            writer.back().await
                .map_err(|e| Error::from(e).into())
        })
    }

    #[getter]
    fn theme(&self) -> PyResult<Theme> {
        let reader = self.0.read()
            .map_err(|e| Error::from(e))?;
        Ok(reader.theme.into())
    }

    #[getter]
    fn language(&self) -> PyResult<Language> {
        let reader = self.0.read()
            .map_err(|e| Error::from(e))?;
        Ok(reader.language.into())
    }

    #[getter]
    fn child_mode(&self) -> PyResult<bool> {
        let reader = self.0.read()
            .map_err(|e| Error::from(e))?;
        Ok(reader.child_mode)
    }

    #[getter]
    fn question(&self) -> PyResult<Option<String>> {
        let reader = self.0.read()
            .map_err(|e| Error::from(e))?;
        Ok(reader.current_question.clone())
    }

    #[getter]
    fn progression(&self) -> PyResult<f32> {
        let reader = self.0.read()
            .map_err(|e| Error::from(e))?;
        Ok(reader.progression)
    }

    #[getter]
    fn step(&self) -> PyResult<usize> {
        let reader = self.0.read()
            .map_err(|e| Error::from(e))?;
        Ok(reader.step)
    }

    #[getter]
    fn first_guess(&self) -> PyResult<Option<Guess>> {
        let reader = self.0.read()
            .map_err(|e| Error::from(e))?;

        Ok(reader.first_guess
            .clone()
            .map(|g| Guess(g)))
    }

    #[getter]
    fn guesses(&self) -> PyResult<Vec<Guess>> {
        let reader = self.0.read()
            .map_err(|e| Error::from(e))?;

        Ok(reader.guesses
            .clone()
            .into_iter()
            .map(|g| Guess(g))
            .collect())
    }

    #[setter]
    fn set_theme(&mut self, theme: Theme) -> PyResult<()> {
        let mut writer = self.0.write()
            .map_err(|e| Error::from(e))?;
        writer.theme = theme.into();

        Ok(())
    }

    #[setter]
    fn set_language(&mut self, language: Language) -> PyResult<()> {
        let mut writer = self.0.write()
            .map_err(|e| Error::from(e))?;
        writer.language = language.into();

        Ok(())
    }

    #[setter]
    fn set_child_mode(&mut self, child_mode: bool) -> PyResult<()> {
        let mut writer = self.0.write()
            .map_err(|e| Error::from(e))?;
        writer.child_mode = child_mode;

        Ok(())
    }
}

#[pymodule]
fn akinator(py: Python<'_>, module: &PyModule) -> PyResult<()> {
    module.add_class::<Akinator>()?;
    module.add_class::<Guess>()?;

    module.add_class::<Theme>()?;
    module.add_class::<Answer>()?;
    module.add_class::<Language>()?;

    error::add_exceptions(&py, module)?;

    Ok(())
}