#![allow(dead_code)]

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
