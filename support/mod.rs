#![allow(dead_code)]

use glium::{self, Display};
use glium::vertex::VertexBufferAny;

pub mod camera;
pub mod vec3;

pub enum Action {
    Stop,
    Continue,
}

pub fn start_loop<F>(mut callback: F) where F: FnMut() -> Action {
    loop {
        match callback() {
            Action::Stop => break,
            Action::Continue => ()
        };
    }
}
