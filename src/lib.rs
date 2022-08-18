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
use lazy_static::lazy_static;
use tokio::runtime::Runtime;
use pyo3::prelude::*;

pub mod enums;
pub mod error;
pub mod models;

lazy_static! {
    static ref RUNTIME: Runtime = Runtime::new().unwrap();
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct Akinator(
    AkinatorStruct,
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

        Self(akinator)
    }

    fn start_game<'a>(&'a mut self, _py: Python<'a>) -> PyResult<Option<String>> {
        RUNTIME.block_on(
            async move {
                self.0.start().await
                    .map_err(|e| Error::from(e).into())
            }
        )
    }

    fn answer<'a>(&'a mut self, _py: Python<'a>, answer: Answer) -> PyResult<Option<String>> {
        RUNTIME.block_on(
            async move {
                self.0.answer(answer.into()).await
                    .map_err(|e| Error::from(e).into())
            }
        )
    }

    fn win<'a>(&'a mut self, _py: Python<'a>) -> PyResult<Option<Guess>> {
        RUNTIME.block_on(
            async move {
                self.0.win().await
                    .map(|result| {
                        result.map(Guess)
                    })
                    .map_err(|e| Error::from(e).into())
            }
        )
    }

    fn back<'a>(&'a mut self, _py: Python<'a>) -> PyResult<Option<String>> {
        RUNTIME.block_on(
            async move {
                self.0.back().await
                    .map_err(|e| Error::from(e).into())
            }
        )
    }

    #[getter]
    fn theme(&self) -> Theme {
        self.0.theme.into()
    }

    #[getter]
    fn language(&self) -> Language {
        self.0.language.into()
    }

    #[getter]
    const fn child_mode(&self) -> bool {
        self.0.child_mode
    }

    #[getter]
    fn question(&self) -> Option<String> {
        self.0.current_question.clone()
    }

    #[getter]
    const fn progression(&self) -> f32 {
        self.0.progression
    }

    #[getter]
    const fn step(&self) -> usize {
        self.0.step
    }

    #[getter]
    fn first_guess(&self) -> Option<Guess> {
        self.0.first_guess
            .clone()
            .map(Guess)
    }

    #[getter]
    fn guesses(&self) -> Vec<Guess> {
        self.0.guesses
            .clone()
            .into_iter()
            .map(Guess)
            .collect()
    }

    #[setter]
    fn set_theme(&mut self, theme: Theme) {
        self.0.theme = theme.into();
    }

    #[setter]
    fn set_language(&mut self, language: Language) {
        self.0.language = language.into();
    }

    #[setter]
    fn set_child_mode(&mut self, child_mode: bool) {
        self.0.child_mode = child_mode;
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
