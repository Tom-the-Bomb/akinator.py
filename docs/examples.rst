
Examples
=========

**Installation**

wheels are prebuilt and uploaded to pypi, so if your platform is supported, simply doing

.. code-block:: shell

    $ py -m pip install akinator.py

will do it

You can also manually build from source, but that requires rust to be installed.

**importing**

.. code-block:: python

    import akinator

**Full sync example**

Here is a full working example of using the sync akinator class

.. code-block:: python

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

**Full async example**

Here is a full working example of using the async akinator class with ``asyncio``

(pretty much the same except all methods are awaited)

.. code-block:: python

    import asyncio

    from akinator import (
        CantGoBackAnyFurther,
        InvalidAnswer,
        AsyncAkinator,
        Answer,
        Theme,
    )

    async def test() -> None:
        # create akinator instance
        aki = AsyncAkinator(
            child_mode=True,
            theme=Theme.from_str('characters'),
        )

        # start the game, and get the first question
        first_question = await aki.start_game()
        # recieve console input for first question
        answer = input(f'{first_question}: ')

        # keep asking and recieving answers while akinator's progression is <=80
        while aki.progression <= 80:
            if answer == 'back':
                # go back a question if response is "back"
                try:
                    await aki.back()
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
                    await aki.answer(answer)

            # recieving console input for next question
            answer = input(f'{aki.question}: ')

        # tell akinator to end the game and make its guess
        first_guess = await aki.win()

        if first_guess:
            # print out its first guess's details
            print('name:', first_guess.name)
            print('desc:', first_guess.description)
            print('image:', first_guess.absolute_picture_path)

    if __name__ == '__main__':
        asyncio.run(test())