pub type Mat4 = [f32; 16];

pub fn new_mat() -> Mat4 {
  return [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
}


pub fn identity() -> Mat4 {
  return [
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 1.0, 0.0,
    0.0, 0.0, 0.0, 1.0
  ];
}

pub fn translation(x: f32, y: f32, z: f32) -> Mat4 {
  return [
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 1.0, 0.0,
    x, y, z, 1.0
  ];
}

pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Mat4 {
  return [
    2.0 / (right - left), 0.0, 0.0, -(right + left) / (right - left),
    0.0, 2.0 / (top - bottom), 0.0, -(top + bottom) / (top - bottom),
    0.0, 0.0, -2.0 / (far - near), -(far + near) / (far - near),
    0.0, 0.0, 0.0, 1.0
  ];
}


// pub fn frustum(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Mat4 {
//   return [2.0 * near / (right - left), 0.0, 0.0, 0.0,
//           0.0, 2.0 * near / (top - bottom), 0.0, 0.0,
//           (right + left) / (right - left), (top + bottom) / (top - bottom), (near + far) / (near - far), -1.0,
//           0.0, 0.0, 2.0 * near * far / (near - far), 0.0];
// }

pub fn col_mul(m1: Mat4, m2: Mat4) -> Mat4 {
  let mut new_mat = new_mat();
  for y in 0..4 {
    for x in 0..4 {
      let mut val: f32 = 0.0;
      for i in 0..4 {
        val += m1[i * 4 + y] * m2[x * 4 + i];
      }
      new_mat[x * 4 + y] = val;
    }
  }
  return new_mat;
}