use pacman::game::Position;
use pacman::tokens::{self, Token};
use pacman::Game;
use pacman::ParseError;

pub struct PacmanWorld {
    state: String,
    game: Result<Game, ParseError>,
}

impl Default for PacmanWorld {
    fn default() -> Self {
        PacmanWorld { state: String::new(), game: Err(ParseError::NotParsed) }
    }
}

macro_rules! use_world_field {
    ($world:ident, $field:ident, $body:expr) => {
        match &$world.$field {
            Ok($field) => $body,
            Err(err) => panic!("The {} was not initialised: {:?}", stringify!($field), err),
        }
    };
}

impl ::cucumber_rust::World for PacmanWorld {}

steps! {
    world: PacmanWorld;

    given "the game state is" |world, step| {
        world.state = step.docstring().unwrap().clone();
    };

    when "we parse the state" |world, _| {
        world.game = world.state.parse();
    };

    then regex r"the game field should be (\d+) x (\d+)", (usize, usize)
    |world, expected_cols, expected_rows, _step| {
        use_world_field!(world, game, {
            assert_eq!(expected_cols, game.field.cols);
            assert_eq!(expected_rows, game.field.rows);
        });
    };

    then regex r"the player should have (\d+) lives", (u64)
    |world, expected_lives, _step| {
        use_world_field!(world, game, {
            assert_eq!(expected_lives, game.stats.lives);
        });
    };

    then regex r"the score should be (\d+)", (u64)
    |world, expected_score, _step| {
        use_world_field!(world, game, {
            assert_eq!(expected_score, game.stats.score);
        });
    };

    then regex r"pacman should be at (\d+) , (\d+)", (usize, usize)
    |world, expected_x, expected_y, _step| {
        use_world_field!(world, game, {
            assert!(game.field.is_pacman_at(&Position { x: expected_x, y: expected_y }));
        });
    };

    then regex r###"pacman should be facing "(LEFT|RIGHT|UP|DOWN)""###,
    (tokens::Orientation)
    |world, expected_orientation, _step| {
        use_world_field!(world, game, {
            if let Some(token) = game.field.pacman_token().clone() {
                match *token {
                    Token::PacmanToken(ref pacman) => assert_eq!(expected_orientation, pacman.orientation),
                    _ => panic!("Pacman token was not actually a pacman!"),
                }
            } else {
                panic!("Pacman not found")
            }
        });
    };

    then "pacman should be dead" |world, _step| {
        use_world_field!(world, game, {
            match game.field.pacman_token().unwrap().as_ref() {
                Token::PacmanToken(pacman) => assert!(!pacman.alive),
                _ => panic!("Pacman token was not actually Pacman"),
            }
        });
    };

    then regex r"ghost should be at (\d+) , (\d+)", (usize, usize)
    |world, expected_x, expected_y, _step| {
        use_world_field!(world, game, assert!(game.field.is_ghost_at(&Position { x: expected_x, y: expected_y })))
    };
}
