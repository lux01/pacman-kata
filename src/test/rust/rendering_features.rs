use pacman::*;

steps! {
    world: super::PacmanWorld;

    given regex r"the screen column width is (\d+)", (usize)
    |world, screen_width, _step| {
        world.screen_width = screen_width;
    };

    given regex r"the player has (\d+) lives", (u64)
    |world, player_lives, _step| {
        world.game.stats.lives = player_lives;
    };

    given regex r"the player score is (\d+)", (u64)
    |world, player_score, _step| {
        world.game.stats.score = player_score;
    };

    when "we render the status line"
    |world, _step| {
        world.render_result = world.game.stats.render(&RenderOptions { screen_width: world.screen_width });
    };

    when "we render the game"
    |world, _step| {
        world.render_result = world.game.render_game();
    };

    when regex r"we play (\d+) turns?", (usize)
    |world, turns, _step| {
        for _ in 0..turns {
            world.game.tick();
        }
    };

    then "I should get the following output:"
    |world, step| {
        let expected_output = step.docstring().expect("No docstring set");

        assert_eq!(*expected_output, world.render_result);
    };

    then "the game screen should be"
    |world, step| {
        let expeceted_screen = step.docstring().expect("No docstring set");
        assert_eq!(expeceted_screen, &world.render_result,
            "\nExpected game state: \n{}\n\nActual game state: \n{}", expeceted_screen, world.render_result);
    };
}
