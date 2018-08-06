use pacman::*;

steps! {
    world: super::PacmanWorld;

    given "the game state is" |world, step| {
        world.state = step.docstring().unwrap().clone();
    };

    given regex r"the score is (\d+)", (u64)
    |world, initial_score, _step| {
        world.game.stats.score = initial_score;
    };

    given regex r"the lives are (\d+)", (u64)
    |world, initial_lives, _step| {
        world.game.stats.lives = initial_lives;
    };

    when "we parse the state" |world, _| {
        world.game = world.game.refresh_from_str(&world.state).expect("Failed to parse state");
    };

    then regex r"the game field should be (\d+) x (\d+)", (usize, usize)
    |world, expected_cols, expected_rows, _step| {
        let bounds = world.game.board.bounds();

        assert_eq!(expected_cols, bounds.cols());
        assert_eq!(expected_rows, bounds.cols());
    };

    then regex r"the player should have (\d+) lives", (u64)
    |world, expected_lives, _step| {
        assert_eq!(expected_lives, world.game.stats.lives);
    };

    then regex r"the game lives should be (\d+)", (u64)
    |world, expected_lives, _step| {
            assert_eq!(expected_lives, world.game.stats.lives);
    };

    then regex r"the score should be (\d+)", (u64)
    |world, expected_score, _step| {
        assert_eq!(expected_score, world.game.stats.score);
    };

    then regex r"pacman should be at (\d+) , (\d+)", (usize, usize)
    |world, expected_x, expected_y, _step| {
        assert!(world.game.board.is_pacman_at(&Position { x: expected_x, y: expected_y }));
    };

    then regex r###"pacman should be facing "(LEFT|RIGHT|UP|DOWN)""###,
    (tokens::Orientation)
    |world, expected_orientation, _step| {
        let orientation = world.game.board.get_pacman()
            .map(|pacman| pacman.orientation)
            .expect("Pacman not found");

        assert_eq!(expected_orientation, orientation);
    };

    then "pacman should be dead" |world, _step| {

            let is_pacman_alive = world.game.board.get_pacman()
                .map(|pacman| pacman.alive)
                .expect("Pacman not found");

            assert!(!is_pacman_alive);
    };

    then regex r"ghost should be at (\d+) , (\d+)", (usize, usize)
    |world, expected_x, expected_y, _step| {
        assert!(world.game.board.is_ghost_at(&Position { x: expected_x, y: expected_y }));
    };

    then regex r"there should be a (\d+) point pill at (\d+) , (\d+)", (u64, usize, usize)
    |world, expected_score_value, expected_x, expected_y, _step| {
            let pill = world.game.board.get_pill_at(&Position { x: expected_x, y: expected_y })
                .expect("Pill not found at location");

            assert_eq!(expected_score_value, pill.score_value);
    };

    then regex r"there should be a wall at (\d+) , (\d+)", (usize, usize)
    |world, expected_x, expected_y, _score| {
            assert!(world.game.board.is_wall_at(&Position { x: expected_x, y: expected_y }), "Wall not found");
    };

    then regex r"there should be a gate at (\d+) , (\d+)", (usize, usize)
    |world, expected_x, expected_y, _score| {
            assert!(world.game.board.is_gate_at(&Position { x: expected_x, y: expected_y }), "Gate not found");
    };

    then regex r"there should be a force field at (\d+) , (\d+)", (usize, usize)
    |world, expected_x, expected_y, _score| {
            assert!(world.game.board.is_force_field_at(&Position { x: expected_x, y: expected_y }), "Force field not found");
    };
}
