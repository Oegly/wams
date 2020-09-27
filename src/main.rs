#![allow(unused)]

pub mod ai;
pub mod asteroid;
pub mod game;
pub mod physics;
pub mod ship;
pub mod shape;
pub mod spawner;
pub mod broadcast;
pub mod piston_bindings;

fn main() {
    piston_bindings::wrapper::main();
}
