use pacman::Game;

#[derive(Debug, Default)]
pub struct PacmanWorld {
    state: String,
    game: Option<Game>,
}

impl ::cucumber_rust::World for PacmanWorld {}

steps! {
    world: PacmanWorld;

    given "the game state is" |world, step| {
        world.state = step.docstring().unwrap().clone();
    };

    when "we parse the state" |world, _| {
        world.game = world.state.parse().ok();
    };

    then regex "the game field should be (\\d+) x (\\d+)" |world, matches, _| {
        let expected_cols = matches[1].parse::<usize>().unwrap();
        let expected_rows = matches[2].parse::<usize>().unwrap();

        if let Some(ref game) = world.game {
            assert_eq!(expected_cols, game.field.cols, "Incorrect number of columns");
            assert_eq!(expected_rows, game.field.rows, "Incorrect number of rows");
        } else {
            panic!("The game was not initialised");
        }
    };

    then regex "the player should have (\\d+) lives" |world, matches, _| {
        let expected_lives = matches[1].parse::<u64>().unwrap();

        if let Some(ref game) = world.game {
            assert_eq!(expected_lives, game.stats.lives);
        } else {
            panic!("The game was not initialised");
        }
    };

    then regex "the score should be (\\d+)" |world, matches, _| {
        let expected_score = matches[1].parse::<u64>().unwrap();
        if let Some(ref game) = world.game {
            assert_eq!(expected_score, game.stats.score);
        } else {
            panic!("The game was not initialised");
        }
    };

}
