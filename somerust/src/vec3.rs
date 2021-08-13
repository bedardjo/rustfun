pub type Vec3 = [f32; 3];

// trait Vec3Utilities {
//   fn x(&self)->f32;
// }

// impl Vec3Utilities for Vec3 {
//   fn x(&self) {
//     self[0];
//   }
// }

pub fn flatten(vs : &Vec<Vec3>) -> Vec<f32> {
  let mut values = Vec::new();
  for v in vs {
    values.push(v[0]);
    values.push(v[1]);
    values.push(v[2]);
  }
  return values;
}