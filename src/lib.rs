extern crate nalgebra;

use nalgebra::{
    dot,
    cross,
    Vector3
};

fn tri_vals(tri: &[Vector3<f64>; 3]) -> (f64, Vector3<f64>) {
    let face_center = (tri[0] + tri[1] + tri[2])/3.0;
    let total_volume = dot(&tri[0], &cross(&tri[1], &tri[2])) / 6.0;
    let total_center = (3.0/4.0)*face_center;
    (total_volume, total_center)
}

pub fn collection_vals<I: Iterator<Item=[Vector3<f64>; 3]>>(tris: &mut I) -> (f64, Vector3<f64>) {
    let mut vacc = 0.0;
    let mut cacc = Vector3::new(0.0, 0.0, 0.0);

    for val in tris {
        let (a,b) = tri_vals(&val);
        vacc += a;
        cacc = cacc + a*b;
    }
    (vacc, cacc/vacc)
}

