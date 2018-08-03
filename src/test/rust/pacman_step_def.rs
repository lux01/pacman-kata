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
    (|$world:ident, $field:ident| -> $body:tt) => {
        match &$world.$field {
            Ok($field) => $body,
            Err(err) => panic!("The game was not initialised: {:?}", err),
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
        use_world_field!(|world, game| -> {
            assert_eq!(expected_cols, game.field.cols);
            assert_eq!(expected_rows, game.field.rows);
        });
    };

    then regex r"the player should have (\d+) lives", (u64)
    |world, expected_lives, _step| {
        match &world.game {
            Ok(ref game) => assert_eq!(expected_lives, game.stats.lives),
            Err(err) => panic!("The game was not initialised: {:?}", err),
        }
    };

    then regex r"the score should be (\d+)", (u64)
    |world, expected_score, _step| {
        match &world.game {
            Ok(ref game) => assert_eq!(expected_score, game.stats.score),
            Err(err) => panic!("The game was not initialised: {:?}", err),
        }
    };

    then regex r"pacman should be at (\d+) , (\d+)", (usize, usize)
    |world, expected_x, expected_y, _step| {
        match &world.game {
            Ok(ref game) => assert_eq!(Some(Position { x: expected_x, y: expected_y }), game.field.pacman_posn),
            Err(err) => panic!("The game was not initialised: {:?}", err),
        }
    };

    then regex r###"pacman should be facing "(LEFT|RIGHT|UP|DOWN)""###,
    (tokens::Orientation)
    |world, expected_orientation, _step| {
        match &world.game {
            Ok(ref game) => {
                if let Some(token) = game.field.pacman_token().clone() {
                    match *token {
                        Token::PacmanToken(ref pacman) => assert_eq!(expected_orientation, pacman.orientation),
                        _ => panic!("Pacman token was not actually a pacman"),
                    }
                } else {
                    panic!("Pacman not found")
                }
            }
            Err(err) => panic!("The game was not initialised: {:?}", err),
        }
    };

    then "pacman should be dead" |world, _step| {
        panic!("ruhoh");
    };
}
