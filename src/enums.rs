use pyo3::prelude::*;
use pyo3::types::PyType;

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

#[pyclass]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Answer {
    Yes = 0,
    No = 1,
    Idk = 2,
    Probably = 3,
    ProbablyNot = 4,
}

#[pyclass]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Theme {
    Characters = 1,
    Animals = 14,
    Objects = 2,
}

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
    #[classmethod]
    fn from_str(_cls: &PyType, answer: String) -> PyResult<Self> {
        AnswerEnum::try_from(answer)
            .map_err(|e| Error::from(e).into())
            .map(Self::from)
    }

    fn __repr__(&self) -> String {
        format!("<Answer answer=\"{:?}\"", self)
    }

    fn __str__(&self) -> String {
        format!("{:?}", self)
    }
}

#[pymethods]
impl Theme {
    #[classmethod]
    fn from_str(_cls: &PyType, theme: String) -> Self {
        Self::from(ThemeEnum::from(theme))
    }

    fn __repr__(&self) -> String {
        format!("<Theme theme=\"{:?}\"", self)
    }

    fn __str__(&self) -> String {
        format!("{:?}", self)
    }
}

#[pymethods]
impl Language {
    #[classmethod]
    fn from_str(_cls: &PyType, language: String) -> PyResult<Self> {
        LanguageEnum::try_from(language)
            .map_err(|e| Error::from(e).into())
            .map(Self::from)
    }

    fn __repr__(&self) -> String {
        format!("<Language lang=\"{:?}\"", self)
    }

    fn __str__(&self) -> String {
        format!("{:?}", self)
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