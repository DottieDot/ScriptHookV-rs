use std::f32::consts::PI;

use super::Vector3;

#[derive(Clone, Copy)]
pub struct Matrix {
  pub forward:  Vector3,
  pub right:    Vector3,
  pub up:       Vector3,
  pub position: Vector3
}

impl Matrix {
  pub fn new_transformation_matrix(position: Vector3, rotation: Vector3) -> Self {
    let radians = rotation * (PI / 180f32);

    let abs_x = radians.x.cos().abs();
    let abs_y = radians.y.cos().abs();

    let forward = Vector3::new(
      -radians.z.sin() * abs_x,
      radians.z.cos() * abs_x,
      radians.x.sin()
    );

    let right = Vector3::new(
      radians.z.cos() * abs_y,
      radians.z.sin() * abs_y,
      -radians.y.sin()
    );

    let up = right.cross(forward);

    Self {
      forward,
      right,
      up,
      position
    }
  }
}
