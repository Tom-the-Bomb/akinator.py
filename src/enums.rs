#![allow(clippy::trivially_copy_pass_by_ref)]

use pyo3::{
    prelude::*,
    types::PyType,
};

use crate::error::Error;
use akinator_rs::enums::{
    Theme as ThemeEnum,
    Answer as AnswerEnum,
    Language as LanguageEnum,
};


macro_rules! cast_enum {
    ( $from:ty, $to:ty, $item:expr, $( $var:tt ),* $(,)* ) => {{
        match $item {
            $(
                <$from>::$var => <$to>::$var,
            )*
        }
    }};
}

/// An enum class representing an answer given to the akinator
///
/// This is meant for the user to use to pass into methods such as `Akinator.answer`
#[pyclass]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Answer {
    Yes = 0,
    No = 1,
    Idk = 2,
    Probably = 3,
    ProbablyNot = 4,
}

/// An enum class representing the theme of an akinator game
///
/// This is meant for the user to use to pass into the Akinator constructor, or to set the theme property
#[pyclass]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Theme {
    Characters = 1,
    Animals = 14,
    Objects = 2,
}

/// An enum class representing the language of the akinator game
///
/// This is meant for the user to use to pass into the Akinator constructor, or to set the language property
#[pyclass]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Language {
    English,
    Arabic,
    Chinese,
    German,
    Spanish,
    French,
    Hebrew,
    Italian,
    Japanese,
    Korean,
    Dutch,
    Polish,
    Portugese,
    Russian,
    Turkish,
    Indonesian,
}

#[pymethods]
impl Answer {
    /// a classmethod to return an :class:`Answer` enum variant parsing from a :class:`str`
    /// useful when you have external user input
    ///
    /// aliases for answer variants are also accepted (trims ws & case-insensitive):
    ///     - ``yes | y | 0`` -> ``Answer.Yes``
    ///     - ``no | n | 1`` -> ``Answer.No``
    ///     - ``i don(')?t know | idk | 2`` -> ``Answer.Idk``
    ///     - ``probably | p | 3`` -> ``Answer.Probably``
    ///     - ``probably not | pn | 4`` -> ``Answer.ProbablyNot``
    ///
    /// Parameters
    /// ----------
    /// answer : :class:`str`
    ///     the string representation of the answer to parse from
    ///
    /// Raises
    /// ------
    /// :class:`InvalidAnswer`
    ///     raised if the provided answer cannot match one of the above (is invalid)
    #[classmethod]
    #[pyo3(text_signature = "(self, answer)")]
    fn from_str(_cls: &PyType, answer: String) -> PyResult<Self> {
        AnswerEnum::try_from(answer)
            .map_err(|e| Error::from(e).into())
            .map(Self::from)
    }

    fn __repr__(&self) -> String {
        format!("<Answer answer=\"{self:?}\">")
    }

    fn __str__(&self) -> String {
        format!("{self:?}")
    }
}

#[pymethods]
impl Theme {
    /// a classmethod to return a :class:`Theme` enum variant parsing from a :class:`str`
    /// useful when you have external user input
    ///
    /// Parameters
    /// ----------
    /// theme : :class:`str`
    ///     the string representation of the theme to parse from
    ///
    /// .. note ::
    ///     if an invalid string for the theme is given, no error will be raised
    ///     instead it will just fallback to ``Theme.Characters`` as the default
    #[classmethod]
    #[pyo3(text_signature = "(self, theme)")]
    fn from_str(_cls: &PyType, theme: String) -> Self {
        Self::from(ThemeEnum::from(theme))
    }

    fn __repr__(&self) -> String {
        format!("<Theme theme=\"{self:?}\">")
    }

    fn __str__(&self) -> String {
        format!("{self:?}")
    }
}

#[pymethods]
impl Language {
    /// a classmethod to return a :class:`Language` enum variant parsing from a :class:`str`
    /// useful when you have external user input
    ///
    /// Short forms such as ``en`` or ``fr`` are also accepted along with the full name
    ///
    /// Parameters
    /// ----------
    /// language : :class:`str`
    ///     the string representation of the language to parse from
    ///
    /// Raises
    /// ------
    /// :class:`InvalidLanguage`
    ///     Raised if the given string is of an invalid language
    #[classmethod]
    #[pyo3(text_signature = "(self, language)")]
    fn from_str(_cls: &PyType, language: String) -> PyResult<Self> {
        LanguageEnum::try_from(language)
            .map_err(|e| Error::from(e).into())
            .map(Self::from)
    }

    fn __repr__(&self) -> String {
        format!("<Language lang=\"{self:?}\">")
    }

    fn __str__(&self) -> String {
        format!("{self:?}")
    }
}

impl From<AnswerEnum> for Answer {
    fn from(answer: AnswerEnum) -> Self {
        cast_enum!(
            AnswerEnum,
            Self,
            answer,
            Yes,
            No,
            Idk,
            Probably,
            ProbablyNot,
        )
    }
}

impl From<Answer> for AnswerEnum {
    fn from(answer: Answer) -> Self {
        cast_enum!(
            Answer,
            Self,
            answer,
            Yes,
            No,
            Idk,
            Probably,
            ProbablyNot,
        )
    }
}

impl From<ThemeEnum> for Theme {
    fn from(theme: ThemeEnum) -> Self {
        cast_enum!(
            ThemeEnum,
            Self,
            theme,
            Characters,
            Animals,
            Objects,
        )
    }
}

impl From<Theme> for ThemeEnum {
    fn from(theme: Theme) -> Self {
        cast_enum!(
            Theme,
            Self,
            theme,
            Characters,
            Animals,
            Objects,
        )
    }
}

impl From<LanguageEnum> for Language {
    fn from(language: LanguageEnum) -> Self {
        cast_enum!(
            LanguageEnum,
            Self,
            language,
            English,
            Arabic,
            Chinese,
            German,
            Spanish,
            French,
            Hebrew,
            Italian,
            Japanese,
            Korean,
            Dutch,
            Polish,
            Portugese,
            Russian,
            Turkish,
            Indonesian,
        )
    }
}

impl From<Language> for LanguageEnum {
    fn from(language: Language) -> Self {
        cast_enum!(
            Language,
            Self,
            language,
            English,
            Arabic,
            Chinese,
            German,
            Spanish,
            French,
            Hebrew,
            Italian,
            Japanese,
            Korean,
            Dutch,
            Polish,
            Portugese,
            Russian,
            Turkish,
            Indonesian,
        )
    }
}