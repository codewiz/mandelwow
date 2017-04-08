extern crate glutin;

use support::vec3::Vec3;
use support::vec3::norm;

use std::f32;

//use glutin::Event;
//use glutin::VirtualKeyCode;

#[derive(Default)]
pub struct CameraState {
    aspect_ratio: f32,
    pos: Vec3,
    dir: Vec3,

    moving_up: bool,
    moving_left: bool,
    moving_down: bool,
    moving_right: bool,
    moving_forward: bool,
    moving_backward: bool,
    turning_up: bool,
    turning_left: bool,
    turning_down: bool,
    turning_right: bool,
}

impl CameraState {
    pub fn new() -> CameraState {
        CameraState {
            aspect_ratio: 1024.0 / 768.0,
            pos: Vec3(0.0, 0.0, 0.0),
            dir: Vec3(0.0, 0.0, -1.0),
            .. Default::default()
        }
    }

    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
    }

    pub fn get_pos(&self) -> Vec3 {
        self.pos
    }

    pub fn set_dir(&mut self, dir: Vec3) {
        self.dir = dir;
    }

    pub fn get_perspective(&self) -> [[f32; 4]; 4] {
        let fov: f32 = 3.141592 / 2.0;
        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        // note: remember that this is column-major, so the lines of code are actually columns
        [
            [f / self.aspect_ratio,    0.0,              0.0              ,   0.0],
            [         0.0         ,     f ,              0.0              ,   0.0],
            [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
            [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
        ]
    }

    pub fn get_view(&self) -> [[f32; 4]; 4] {
        let f = norm(&self.dir);

        let up = Vec3(0.0, 1.0, 0.0);

        let s = Vec3(f.1 * up.2 - f.2 * up.1,
                     f.2 * up.0 - f.0 * up.2,
                     f.0 * up.1 - f.1 * up.0);
        let sn = norm(&s);

        let u = (sn.1 * f.2 - sn.2 * f.1,
                 sn.2 * f.0 - sn.0 * f.2,
                 sn.0 * f.1 - sn.1 * f.0);

        let p = (-self.pos.0 * s.0 - self.pos.1 * s.1 - self.pos.2 * s.2,
                 -self.pos.0 * u.0 - self.pos.1 * u.1 - self.pos.2 * u.2,
                 -self.pos.0 * f.0 - self.pos.1 * f.1 - self.pos.2 * f.2);

        // note: remember that this is column-major, so the lines of code are actually columns
        [
            [sn.0, u.0, f.0, 0.0],
            [sn.1, u.1, f.1, 0.0],
            [sn.2, u.2, f.2, 0.0],
            [p.0,  p.1, p.2, 1.0],
        ]
    }

    pub fn update(&mut self) {
        let f = norm(&self.dir);

        let up = Vec3(0.0, 1.0, 0.0);

        let s = Vec3(f.1 * up.2 - f.2 * up.1,
                     f.2 * up.0 - f.0 * up.2,
                     f.0 * up.1 - f.1 * up.0);

        let s = norm(&s);

        let u = Vec3(s.1 * f.2 - s.2 * f.1,
                     s.2 * f.0 - s.0 * f.2,
                     s.0 * f.1 - s.1 * f.0);

        if self.moving_up {
            self.pos += u * 0.01;
        }
        if self.moving_left {
            self.pos.0 -= s.0 * 0.01;
            self.pos.1 -= s.1 * 0.01;
            self.pos.2 -= s.2 * 0.01;
        }
        if self.moving_down {
            self.pos.0 -= u.0 * 0.01;
            self.pos.1 -= u.1 * 0.01;
            self.pos.2 -= u.2 * 0.01;
        }
        if self.moving_right {
            self.pos += s * 0.01;
        }
        if self.moving_forward {
            self.pos += f * 0.01;
        }
        if self.moving_backward {
            self.pos.0 -= f.0 * 0.01;
            self.pos.1 -= f.1 * 0.01;
            self.pos.2 -= f.2 * 0.01;
        }
        if self.turning_left {
            let a: f32 = 0.05;
            self.dir = Vec3(f.0 * a.cos() + f.2 * a.sin(), f.1, - f.0 * a.sin() + f.2 * a.cos());
        }
        if self.turning_right {
            let a: f32 = -0.05;
            self.dir = Vec3(f.0 * a.cos() + f.2 * a.sin(), f.1, - f.0 * a.sin() + f.2 * a.cos());
        }
        if self.turning_up {
            let a: f32 = -0.05;
            self.dir = Vec3(f.0, f.1 * a.cos() - f.2 * a.sin(), f.1 * a.sin() + f.2 * a.cos());
        }
        if self.turning_down {
            let a: f32 = 0.05;
            self.dir = Vec3(f.0, f.1 * a.cos() - f.2 * a.sin(), f.1 * a.sin() + f.2 * a.cos());
        }
        //println!("camera_pos = {:?}", self.pos);
        //println!("camera_dir = {:?}", self.dir);
    }

    pub fn process_input(&mut self, event: &glutin::Event) {
        match event {
            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::Up)) => {
                self.moving_up = true;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Released, _, Some(glutin::VirtualKeyCode::Up)) => {
                self.moving_up = false;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::Down)) => {
                self.moving_down = true;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Released, _, Some(glutin::VirtualKeyCode::Down)) => {
                self.moving_down = false;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::Left)) => {
                self.moving_left = true;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Released, _, Some(glutin::VirtualKeyCode::Left)) => {
                self.moving_left = false;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::Right)) => {
                self.moving_right = true;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Released, _, Some(glutin::VirtualKeyCode::Right)) => {
                self.moving_right = false;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::A)) => {
                self.turning_left = true;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Released, _, Some(glutin::VirtualKeyCode::A)) => {
                self.turning_left = false;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::D)) => {
                self.turning_right = true;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Released, _, Some(glutin::VirtualKeyCode::D)) => {
                self.turning_right = false;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::W)) => {
                self.moving_forward = true;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Released, _, Some(glutin::VirtualKeyCode::W)) => {
                self.moving_forward = false;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::S)) => {
                self.moving_backward = true;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Released, _, Some(glutin::VirtualKeyCode::S)) => {
                self.moving_backward = false;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::R)) => {
                self.turning_up = true;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Released, _, Some(glutin::VirtualKeyCode::R)) => {
                self.turning_up = false;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::F)) => {
                self.turning_down = true;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Released, _, Some(glutin::VirtualKeyCode::F)) => {
                self.turning_down = false;
            },
            _ => {}
        }
    }
}
