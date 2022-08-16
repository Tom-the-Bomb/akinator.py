use crate::{
    enums::{
        Theme,
        Answer,
        Language,
    },
    error::Error,
    models::Guess,
};

use akinator_rs::Akinator as AkinatorStruct;

use std::sync::Arc;
use tokio::sync::RwLock;

use pyo3::prelude::*;
use pyo3_asyncio::tokio::local_future_into_py as to_coro;

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
    ) -> Self {
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

        Self(
            Arc::new(RwLock::new(akinator))
        )
    }

    fn start_game<'a>(&'a mut self, py: Python<'a>) -> PyResult<&'a pyo3::PyAny> {
        let cloned = self.0.clone();

        to_coro(py, async move {
            let mut writer = cloned.write()
                .await;

            writer.start().await
                .map_err(|e| Error::from(e).into())
        })
    }

    fn answer<'a>(&'a mut self, py: Python<'a>, answer: Answer) -> PyResult<&'a pyo3::PyAny> {
        let cloned = self.0.clone();

        to_coro(py, async move {
            let mut writer = cloned.write()
                .await;

            writer.answer(answer.into()).await
                .map_err(|e| Error::from(e).into())
        })
    }

    fn win<'a>(&'a mut self, py: Python<'a>) -> PyResult<&'a pyo3::PyAny> {
        let cloned = self.0.clone();

        to_coro(py, async move {
            let mut writer = cloned.write()
                .await;

            writer.win().await
                .map(|result| {
                    result.map(Guess)
                })
                .map_err(|e| Error::from(e).into())
        })
    }

    fn back<'a>(&'a mut self, py: Python<'a>) -> PyResult<&'a pyo3::PyAny> {
        let cloned = self.0.clone();

        to_coro(py, async move {
            let mut writer = cloned.write()
                .await;

            writer.back().await
                .map_err(|e| Error::from(e).into())
        })
    }

    #[getter]
    fn theme(&self) -> Theme {
        let reader = self.0
            .blocking_read();

        reader.theme.into()
    }

    #[getter]
    fn language(&self) -> Language {
        let reader = self.0
            .blocking_read();

        reader.language.into()
    }

    #[getter]
    fn child_mode(&self) -> bool {
        let reader = self.0
            .blocking_read();

        reader.child_mode
    }

    #[getter]
    fn question(&self) -> Option<String> {
        let reader = self.0
            .blocking_read();

        reader.current_question.clone()
    }

    #[getter]
    fn progression(&self) -> f32 {
        let reader = self.0
            .blocking_read();

        reader.progression
    }

    #[getter]
    fn step(&self) -> usize {
        let reader = self.0
            .blocking_read();

        reader.step
    }

    #[getter]
    fn first_guess(&self) -> Option<Guess> {
        let reader = self.0
            .blocking_read();

        reader.first_guess
            .clone()
            .map(Guess)
    }

    #[getter]
    fn guesses(&self) -> Vec<Guess> {
        let reader = self.0
            .blocking_read();

        reader.guesses
            .clone()
            .into_iter()
            .map(Guess)
            .collect()
    }

    #[setter]
    fn set_theme(&mut self, theme: Theme) {
        let mut writer = self.0
            .blocking_write();

        writer.theme = theme.into();
    }

    #[setter]
    fn set_language(&mut self, language: Language) {
        let mut writer = self.0
            .blocking_write();

        writer.language = language.into();
    }

    #[setter]
    fn set_child_mode(&mut self, child_mode: bool) {
        let mut writer = self.0
            .blocking_write();

        writer.child_mode = child_mode;
    }
}

#[pymodule]
fn akinator(py: Python<'_>, module: &PyModule) -> PyResult<()> {
    module.add_class::<Akinator>()?;
    module.add_class::<Guess>()?;

    module.add_class::<Theme>()?;
    module.add_class::<Answer>()?;
    module.add_class::<Language>()?;

    error::add_exceptions(py, module)?;

    Ok(())
}
