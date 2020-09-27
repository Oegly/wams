#![allow(unused)]

#[cfg(target_arch = "wasm32")]
extern crate console_error_panic_hook;

#[cfg(target_arch = "wasm32")]
extern crate wasm_bindgen;

pub mod ai;
pub mod game;
pub mod physics;
pub mod ship;
pub mod shape;
pub mod spawner;
pub mod broadcast;

#[cfg(target_arch = "wasm32")]
pub mod wasm_bindings;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start() -> wasm_bindings::wrapper::GameWrapper {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    wasm_bindings::wrapper::GameWrapper::new()
}
