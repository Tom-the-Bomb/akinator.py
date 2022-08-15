use pyo3::prelude::*;
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