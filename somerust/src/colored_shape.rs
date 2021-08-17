use crate::vec3;
use std::f32::consts::PI;

pub struct ColoredShape {
  pub vertices: Vec<vec3::Vec3>,
  pub colors: Vec<vec3::Vec3>,
}

pub fn equilateral_triangle(radius: f32) -> ColoredShape {
  let top_angle: f32 = PI / 2.0;
  let left_angle: f32 = top_angle + (2.0 * PI) / 3.0;
  let right_angle: f32 = left_angle + (2.0 * PI) / 3.0;

  let vertices = vec![
    [0.0, radius, 0.0],
    [right_angle.cos() * radius, right_angle.sin() * radius, 0.0],
    [left_angle.cos() * radius, left_angle.sin() * radius, 0.0]
  ];

  let colors = vec![
    [1.0, 0.0, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 0.0, 1.0],
  ];

  return ColoredShape {
    vertices: vertices,
    colors: colors
  }
}

pub fn square(radius: f32, color: &vec3::Vec3) -> ColoredShape {
  let vertices = vec![
    [-radius, -radius, 0.0],
    [-radius, radius, 0.0],
    [radius, radius, 0.0],
    
    [radius, radius, 0.0],
    [radius, -radius, 0.0],
    [-radius, -radius, 0.0],
  ];

  let colors = vec![
    color.clone(),
    color.clone(),
    color.clone(),
    
    color.clone(),
    color.clone(),
    color.clone(),
  ];

  return ColoredShape {
    vertices: vertices,
    colors: colors
  }
}