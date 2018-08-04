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
        world.render_result = world.game.stats.render(world.screen_width);
    };

    then "I should get the following output:"
    |world, step| {
        let expected_output = step.docstring().expect("No docstring set");

        assert_eq!(*expected_output, world.render_result);
    };
}
