#![feature(fnbox)]

#[macro_use]
extern crate cucumber_rust;
extern crate pacman;

mod pacman_step_def;

const FEATURES_LOCATION: &'static str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/test/resources/rust-wip-features"
);

cucumber! {
    features: FEATURES_LOCATION;
    world: pacman_step_def::PacmanWorld;
    steps: &[
        pacman_step_def::steps,
    ]
}
