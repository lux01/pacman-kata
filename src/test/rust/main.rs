#[macro_use]
extern crate cucumber_rust;
extern crate pacman;

#[derive(Default)]
pub struct PacmanWorld {
    state: String,
    game: pacman::Game,
    screen_width: usize,
    render_result: String,
}

impl ::cucumber_rust::World for PacmanWorld {}

mod parsing_features;
mod rendering_features;

const FEATURES_LOCATION: &'static str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/test/resources/rust-wip-features"
);

cucumber! {
    features: FEATURES_LOCATION;
    world: PacmanWorld;
    steps: &[
        parsing_features::steps,
        rendering_features::steps,
    ]
}
