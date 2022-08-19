from akinator import (
    CantGoBackAnyFurther,
    InvalidAnswer,
    Akinator,
    Answer,
    Theme,
)

def test() -> None:
    # create akinator instance
    aki = Akinator(
        child_mode=True,
        theme=Theme.from_str('characters'),
    )

    # start the game, and get the first question
    first_question = aki.start_game()
    # recieve console input for first question
    answer = input(f'{first_question}: ')

    # keep asking and recieving answers while akinator's progression is <=80
    while aki.progression <= 80:
        if answer == 'back':
            # go back a question if response is "back"
            try:
                aki.back()
                print('went back 1 question')
            except CantGoBackAnyFurther:
                print('cannot go back any further!')
        else:
            try:
                # parse to an answer enum variant
                answer = Answer.from_str(answer)
            except InvalidAnswer:
                print('Invalid answer')
            else:
                # answer current question
                aki.answer(answer)

        # recieving console input for next question
        answer = input(f'{aki.question}: ')

    # tell akinator to end the game and make its guess
    first_guess = aki.win()

    if first_guess:
        # print out its first guess's details
        print('name:', first_guess.name)
        print('desc:', first_guess.description)
        print('image:', first_guess.absolute_picture_path)

if __name__ == '__main__':
    test()