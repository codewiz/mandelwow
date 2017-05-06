use std::f32;

#[derive(Copy, Clone, Debug)]
pub struct Cube {
    pub xmin: f32,
    pub ymin: f32,
    pub zmin: f32,
    pub xmax: f32,
    pub ymax: f32,
    pub zmax: f32,
}

// Returns a unit Cube centered in the origin.
impl Default for Cube {
    fn default() -> Cube {
        Cube { xmin: -0.5, ymin: -0.5, zmin: -0.5, xmax: 0.5, ymax: 0.5, zmax: 0.5f32 }
    }
}
