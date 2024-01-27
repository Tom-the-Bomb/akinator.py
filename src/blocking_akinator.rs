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

lazy_static! {
    static ref RUNTIME: Runtime = Runtime::new().unwrap();
}


/// Represents an akinator game
///
/// Parameters are also set as properties which also have a setter to change the values if necessary in the future
///
/// Parameters
/// ----------
/// theme : Optional[:class:`Theme`]
///     the theme of the akinator game, would be one of ``Characters``, ``Animals`` or ``Objects``
///     pass in using an answer enum, using the ``from_str`` classmethod if necessary,
///     defaults to ``Characters``
/// language : Optional[:class:`Language`]
///     the language for the akinator game, refer to the ``Language`` enum,
///     defaults to ``English``
/// child_mode : Optional[:class:`bool`]
///     when set to ``True``, NSFW content will not be provided,
///     defaults to ``False``
#[pyclass]
#[derive(Debug, Clone)]
#[pyo3(text_signature = "(*, theme = None, language = None, child_mode = None)")]
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
    ) -> PyResult<Self> {
        let mut akinator =
            AkinatorStruct::new()
            .map_err(Error::from)?;

        if let Some(theme) = theme {
            akinator = akinator.with_theme(theme.into());
        }

        if let Some(language) = language {
            akinator = akinator.with_language(language.into());
        }

        if child_mode.unwrap_or(false) {
            akinator = akinator.with_child_mode();
        }

        Ok(Self(akinator))
    }

    fn __repr__(&self) -> String {
        format!(
            "<Akinator theme=\"{:?}\" language=\"{:?}\" child_mode={}>",
            self.theme(),
            self.language(),
            self.child_mode(),
        )
    }

    /// Starts the akinator game
    /// and returns the first question
    ///
    /// Returns
    /// -------
    /// Optional[:class:`str`]
    ///
    /// Raises
    /// ------
    /// :class:`RuntimeError`
    ///     Something internal went wrong, this could be in this case:
    ///         - getting the starting timestamp failed
    ///         - the data required to start the game such as the server url, frontaddr or game UID could not be found
    ///         - request error: any sort of error when making the HTTP requests
    ///         - updating the internal data fields errored (either a field was missing or was of the wrong type)
    /// :class:`ValueError`
    ///     Could not parse the API returned JSON properly (invalid, missing fields etc.)
    /// ``Other api errors``
    ///     Refer to the exceptions at the bottom of the page
    fn start_game<'a>(&'a mut self, _py: Python<'a>) -> PyResult<Option<String>> {
        RUNTIME.block_on(
            async move {
                self.0.start().await
                    .map_err(|e| Error::from(e).into())
            }
        )
    }

    /// Answers the akinator's current question
    /// with the provided ``answer``
    /// and returns the next question
    ///
    /// Parameters
    /// ----------
    /// answer : :class:`Answer`
    ///     the answer to the current question
    ///
    /// Returns
    /// -------
    /// Optional[:class:`str`]
    ///
    /// Raises
    /// ------
    /// :class:`RuntimeError`
    ///     Something internal went wrong, this could be in this case:
    ///         - missing required data to continue
    ///         - request error: any sort of error when making the HTTP requests
    ///         - updating the internal data fields errored (either a field was missing or was of the wrong type)
    /// :class:`ValueError`
    ///     Could not parse the API returned JSON properly (invalid, missing fields etc.)
    /// ``Other api errors``
    ///     Refer to the exceptions at the bottom of the page
    #[pyo3(text_signature = "(self, answer)")]
    fn answer<'a>(&'a mut self, _py: Python<'a>, answer: Answer) -> PyResult<Option<String>> {
        RUNTIME.block_on(
            async move {
                self.0.answer(answer.into()).await
                    .map_err(|e| Error::from(e).into())
            }
        )
    }

    /// Tells the akinator to end the game and make its guess
    /// should be called once when the ``progression`` is high enough such as ``>=80.0``
    /// and returns its best guess
    ///
    /// Returns
    /// -------
    /// Optional[:class:`Guess`]
    ///
    /// Raises
    /// ------
    /// :class:`RuntimeError`
    ///     Something internal went wrong, this could be in this case:
    ///         - missing required data to continue
    ///         - request error: any sort of error when making the HTTP requests
    ///         - updating the internal data fields errored (either a field was missing or was of the wrong type)
    /// :class:`ValueError`
    ///     Could not parse the API returned JSON properly (invalid, missing fields etc.)
    /// ``Other api errors``
    ///     Refer to the exceptions at the bottom of the page
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

    /// Goes back a question
    /// and returns said (current) question
    ///
    /// Returns
    /// -------
    /// Optional[:class:`str`]
    ///
    /// Raises
    /// ------
    /// :class:`CantGoBackAnyFurther`
    ///     Could not go back anymore, likely that we are already on the first question
    /// :class:`RuntimeError`
    ///     Something internal went wrong, this could be in this case:
    ///         - missing required data to continue
    ///         - request error: any sort of error when making the HTTP requests
    ///         - updating the internal data fields errored (either a field was missing or was of the wrong type)
    /// :class:`ValueError`
    ///     Could not parse the API returned JSON properly (invalid, missing fields etc.)
    /// ``Other api errors``
    ///     Refer to the exceptions at the bottom of the page
    fn back<'a>(&'a mut self, _py: Python<'a>) -> PyResult<Option<String>> {
        RUNTIME.block_on(
            async move {
                self.0.back().await
                    .map_err(|e| Error::from(e).into())
            }
        )
    }

    /// :class:`Theme`: the theme of the akinator game
    #[getter]
    fn theme(&self) -> Theme {
        self.0.theme.into()
    }

    /// :class:`Language`: the language of the akinator game
    #[getter]
    fn language(&self) -> Language {
        self.0.language.into()
    }

    /// :class:`bool`: whether ``child_mode`` is on or off for the akinator game
    #[getter]
    const fn child_mode(&self) -> bool {
        self.0.child_mode
    }

    /// Optional[:class:`str`]: the current question of the akinator game
    #[getter]
    fn question(&self) -> Option<String> {
        self.0.current_question.clone()
    }

    /// :class:`float`: the progression of the akinator
    #[getter]
    const fn progression(&self) -> f32 {
        self.0.progression
    }

    /// :class:`int`: a counter for the question # the akinator is on currently
    #[getter]
    const fn step(&self) -> usize {
        self.0.step
    }

    /// Optional[:class:`Guess`]: the akinator's best guess
    #[getter]
    fn first_guess(&self) -> Option<Guess> {
        self.0.first_guess
            .clone()
            .map(Guess)
    }

    /// List[:class:`Guess`]: a list of all the akinator's potential guesses, ordered
    #[getter]
    fn guesses(&self) -> Vec<Guess> {
        self.0.guesses
            .clone()
            .into_iter()
            .map(Guess)
            .collect()
    }

    /// property setter to set ``self.theme``
    #[setter]
    fn set_theme(&mut self, theme: Theme) {
        self.0.theme = theme.into();
    }

    /// property setter to set ``self.language``
    #[setter]
    fn set_language(&mut self, language: Language) {
        self.0.language = language.into();
    }

    /// property setter to set ``self.child_mode``
    #[setter]
    fn set_child_mode(&mut self, child_mode: bool) {
        self.0.child_mode = child_mode;
    }
}