#![allow(unused)]

extern crate serde;
extern crate serde_json;

pub mod ai;
pub mod asteroid;
pub mod camera;
pub mod game;
pub mod physics;
pub mod ship;
pub mod spawner;
pub mod storage;
pub mod broadcast;
pub mod piston_bindings;

fn main() {
    piston_bindings::wrapper::main();
}
