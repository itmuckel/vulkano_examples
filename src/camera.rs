use nalgebra_glm::{Mat4, rotate_x, rotate_y, rotate_z, translate, Vec3, vec3};

pub struct Camera {
    view_matrix: Mat4,
    pub position: Vec3,
    pub rotation: Vec3,
    pub zoom: f32,
}

impl Camera {
    pub fn new(position: Vec3, zoom: f32) -> Self {
        let mut camera = Camera {
            position,
            rotation: vec3(0.0, 0.0, 0.0),
            zoom,
            view_matrix: Mat4::identity(),
        };
        camera.update_view_matrix();

        camera
    }

    pub fn update_view_matrix(&mut self) {
        let mut rot_matrix = Mat4::identity();

        rot_matrix = rotate_x(&rot_matrix, f32::to_radians(self.rotation.x));
        rot_matrix = rotate_y(&rot_matrix, f32::to_radians(self.rotation.y));
        rot_matrix = rotate_z(&rot_matrix, f32::to_radians(self.rotation.z));

        let trans_matrix = translate(&Mat4::identity(), &self.position);

        self.view_matrix = trans_matrix * rot_matrix * Mat4::from_diagonal_element(self.zoom);
    }

    pub fn get_rotation(&self) -> Vec3 {
        self.rotation
    }

    pub fn translate(&mut self, delta: Vec3) {
        self.position += delta;
    }

    pub fn zoom(&mut self, delta: f32) {
        self.zoom += delta;
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        self.view_matrix
    }

    pub fn set_rotation(&mut self, rotation: Vec3) {
        self.rotation = rotation;
    }

    pub fn rotate(&mut self, delta: Vec3) {
        self.rotation += delta;
    }
}