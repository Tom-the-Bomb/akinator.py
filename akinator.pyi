from __future__ import annotations

from typing import Optional, List

class Theme:
    Characters: Theme
    Animals: Theme
    Objects: Theme

class Answer:
    Yes: Answer
    No: Answer
    Idk: Answer
    Probably: Answer
    ProbablyNot: Answer

class Language:
    English: Language
    Arabic: Language
    Chinese: Language
    German: Language
    Spanish: Language
    French: Language
    Hebrew: Language
    Italian: Language
    Japanese: Language
    Korean: Language
    Dutch: Language
    Polish: Language
    Portugese: Language
    Russian: Language
    Turkish: Language
    Indonesian: Language

class Guess:
    @property
    def id(self) -> str:
        ...

    @property
    def name(self) -> str:
        ...

    @property
    def award_id(self) -> str:
        ...

    @property
    def flag_photo(self) -> int:
        ...

    @property
    def confidence(self) -> float:
        ...

    @property
    def description(self) -> str:
        ...

    @property
    def ranking(self) -> str:
        ...

    @property
    def picture_path(self) -> str:
        ...

    @property
    def absolute_picture_path(self) -> str:
        ...

class Akinator:
    def __init__(
        self,
        *,
        theme: Optional[Theme] = None,
        language: Optional[Language] = None,
        child_mode: Optional[bool] = None,
    ) -> None:
        ...

    def start_game(self) -> Optional[str]:
        ...

    def answer(self, answer: Answer) -> Optional[str]:
        ...

    def win(self) -> Optional[Guess]:
        ...

    def back(self) -> Optional[str]:
        ...

    @property
    def theme(self) -> Theme:
        ...

    @property
    def language(self) -> Language:
        ...

    @property
    def child_mode(self) -> bool:
        ...

    @property
    def question(self) -> Optional[str]:
        ...

    @property
    def progression(self) -> float:
        ...

    @property
    def step(self) -> int:
        ...

    @property
    def first_guess(self) -> Optional[Guess]:
        ...

    @property
    def guesses(self) -> List[Guess]:
        ...

    @property
    def absolute_picture_path(self) -> str:
        ...

    @property.setter
    def set_theme(self, theme: Theme) -> None:
        ...

    @property.setter
    def set_language(self, language: Language) -> None:
        ...

    @property.setter
    def set_child_mode(self, child_mode: bool) -> None:
        ...