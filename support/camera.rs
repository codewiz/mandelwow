extern crate glutin;

use cgmath::{Matrix4, Vector4};
use cgmath::conv::array4x4;
use glutin::ElementState::{Pressed, Released};
use glutin::Event::{KeyboardInput, MouseMoved};
use glutin::VirtualKeyCode;
use std::f32::consts::PI;
use support::vec3::Vec3;
use support::vec3::norm;

use std::f32;

//use glutin::Event;
//use VirtualKeyCode;

#[derive(Default)]
pub struct CameraState {
    aspect: f32,
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

    mouse_x: i32,
    mouse_y: i32,
    rel_x: i32,
    rel_y: i32,
}

impl CameraState {
    pub fn new() -> CameraState {
        CameraState {
            aspect: 1280.0 / 720.0,
            pos: Vec3(0.0, 0.0, 0.0),
            dir: Vec3(0.0, 0.0, -1.0),
            mouse_x: -1,
            mouse_y: -1,
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

    pub fn get_persp_mat(&self) -> Matrix4<f32> {
        let fov: f32 = PI / 2.0;
        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        // note: remember that this is column-major, so the lines of code are actually columns
        Matrix4 {
            x: Vector4{ x: f / self.aspect, y: 0.0, z:  0.0,                           w: 0.0 },
            y: Vector4{ x: 0.0,             y: f,   z:  0.0,                           w: 0.0 },
            z: Vector4{ x: 0.0,             y: 0.0, z:  (zfar+znear)/(zfar-znear),     w: 1.0 },
            w: Vector4{ x: 0.0,             y: 0.0, z: -(2.0*zfar*znear)/(zfar-znear), w: 0.0 },
        }
    }

    pub fn get_view_mat(&self) -> Matrix4<f32> {
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
        Matrix4{
            x: Vector4{ x: sn.0, y: u.0, z: f.0, w: 0.0 },
            y: Vector4{ x: sn.1, y: u.1, z: f.1, w: 0.0 },
            z: Vector4{ x: sn.2, y: u.2, z: f.2, w: 0.0 },
            w: Vector4{ x:  p.0, y: p.1, z: p.2, w: 1.0 },
        }
    }

    pub fn get_perspview(&self) -> [[f32; 4]; 4] {
        array4x4(self.get_persp_mat() * self.get_view_mat())
    }

    pub fn get_perspective(&self) -> [[f32; 4]; 4] {
        array4x4(self.get_persp_mat())
    }

    pub fn get_view(&self) -> [[f32; 4]; 4] {
        array4x4(self.get_view_mat())
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

        let walk_speed = 0.01;
        let strife_speed = 0.02;
        let pan_speed = 0.001;

        if self.moving_up {
            self.pos += u * strife_speed;
        }
        if self.moving_down {
            self.pos -= u * strife_speed;
        }
        if self.moving_left {
            self.pos -= s * strife_speed;
        }
        if self.moving_right {
            self.pos += s * strife_speed;
        }
        if self.moving_forward {
            self.pos += f * walk_speed;
        }
        if self.moving_backward {
            self.pos -= f * walk_speed;
        }

        if self.turning_left { self.rel_x -= 8; }
        if self.turning_right { self.rel_x += 8; }
        if self.turning_up { self.rel_y -= 2; }
        if self.turning_down { self.rel_y += 2; }
        let vx = -pan_speed * self.rel_x as f32;
        let vy = -pan_speed * self.rel_y as f32;
        self.dir = Vec3(f.0 * vx.cos() + f.2 * vx.sin(),
                        f.1 * vy.cos() - f.2 * vy.sin(),
                        f.1 * vy.sin() - f.0 * vx.sin() + f.2 * vx.cos() * vy.cos());
        self.rel_x = 0;
        self.rel_y = 0;

        //println!("camera_pos = {:?}", self.pos);
        //println!("camera_dir = {:?}", self.dir);
    }

    pub fn process_input(&mut self, event: &glutin::Event) {
        match event {
            &MouseMoved(x, y) => {
                if self.mouse_x == -1 {
                    // Set initial absolute position.
                    self.mouse_x = x;
                    self.mouse_y = y;
                }
                self.rel_x += x - self.mouse_x;
                self.rel_y += y - self.mouse_y;
                self.mouse_x = x;
                self.mouse_y = y;
            }
            &KeyboardInput(Pressed, _, Some(VirtualKeyCode::Up)) => {
                self.moving_up = true;
            },
            &KeyboardInput(Released, _, Some(VirtualKeyCode::Up)) => {
                self.moving_up = false;
            },
            &KeyboardInput(Pressed, _, Some(VirtualKeyCode::Down)) => {
                self.moving_down = true;
            },
            &KeyboardInput(Released, _, Some(VirtualKeyCode::Down)) => {
                self.moving_down = false;
            },
            &KeyboardInput(Pressed, _, Some(VirtualKeyCode::Left)) => {
                self.moving_left = true;
            },
            &KeyboardInput(Released, _, Some(VirtualKeyCode::Left)) => {
                self.moving_left = false;
            },
            &KeyboardInput(Pressed, _, Some(VirtualKeyCode::Right)) => {
                self.moving_right = true;
            },
            &KeyboardInput(Released, _, Some(VirtualKeyCode::Right)) => {
                self.moving_right = false;
            },
            &KeyboardInput(Pressed, _, Some(VirtualKeyCode::A)) => {
                self.turning_left = true;
            },
            &KeyboardInput(Released, _, Some(VirtualKeyCode::A)) => {
                self.turning_left = false;
            },
            &KeyboardInput(Pressed, _, Some(VirtualKeyCode::D)) => {
                self.turning_right = true;
            },
            &KeyboardInput(Released, _, Some(VirtualKeyCode::D)) => {
                self.turning_right = false;
            },
            &KeyboardInput(Pressed, _, Some(VirtualKeyCode::W)) => {
                self.moving_forward = true;
            },
            &KeyboardInput(Released, _, Some(VirtualKeyCode::W)) => {
                self.moving_forward = false;
            },
            &KeyboardInput(Pressed, _, Some(VirtualKeyCode::S)) => {
                self.moving_backward = true;
            },
            &KeyboardInput(Released, _, Some(VirtualKeyCode::S)) => {
                self.moving_backward = false;
            },
            &KeyboardInput(Pressed, _, Some(VirtualKeyCode::R)) => {
                self.turning_up = true;
            },
            &KeyboardInput(Released, _, Some(VirtualKeyCode::R)) => {
                self.turning_up = false;
            },
            &KeyboardInput(Pressed, _, Some(VirtualKeyCode::F)) => {
                self.turning_down = true;
            },
            &KeyboardInput(Released, _, Some(VirtualKeyCode::F)) => {
                self.turning_down = false;
            },
            _ => {}
        }
    }
}
