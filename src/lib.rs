extern crate nalgebra;

use nalgebra::{
    dot,
    cross,
    Vector3,
    Matrix3,
    Norm,
    zero
};

fn imat(tet: &[Vector3<f64>; 4]) -> Matrix3<f64> {
    let mut mat: Matrix3<f64> = zero();
    for r in 0..3 {
        for c in 0..3 {
            if r != c {
                let (p1, p2) = match (r,c) {
                    (1,0) | (0,1) => (0,1),
                    (2,0) | (0,2) => (0,2),
                    (2,1) | (1,2) => (1,2),
                    _ => (r,c)
                };
                for a in 0..4 {
                    mat[(r,c)] -= tet[a][r] * tet[a][c];
                    for b in 0..4 {
                        mat[(r,c)] -= tet[a][r] * tet[b][c];
                    }
                }
            } else {
                let (p1, p2) = match r {
                    0 => (1,2),
                    1 => (0,2),
                    2 => (0,1),
                    _ => panic!("WHAT?")
                };
                for a in 0..4 {
                    mat[(r,c)] +=  tet[a][p1] * tet[a][p1] + tet[a][p2] * tet[a][p2];
                    for b in 0..4 {
                        mat[(r,c)] += tet[a][p1] * tet[b][p1] + tet[a][p2] * tet[b][p2];
                    }
                }
            }
        }
    }
    mat * 0.05
}

fn tri_vals(tri: &[Vector3<f64>; 3]) -> (f64, Vector3<f64>, Matrix3<f64>) {
    let face_center = (tri[0] + tri[1] + tri[2])/3.0;
    let total_volume = dot(&tri[0], &cross(&tri[1], &tri[2])) / 6.0;
    let total_center = (3.0/4.0)*face_center;
    let mut rverts = [-total_center; 4];
    for i in  0..3 {
        rverts[i] = tri[i]-total_center;
    }
    (total_volume, total_center, total_volume*imat(&rverts))
}

pub fn collection_vals<I: Iterator<Item=[Vector3<f64>; 3]>>(tris: &mut I) -> (f64, Vector3<f64>, Matrix3<f64>) {
    let mut vacc = 0.0;
    let mut cacc = Vector3::new(0.0, 0.0, 0.0);
    let mut iacc: Matrix3<f64> = zero();

    let mut vals = Vec::new();

    for val in tris {
        let (v,c,i) = tri_vals(&val);
        vals.push((v,c,i));
        vacc += v;
        cacc = cacc + v*c;
    }
    cacc = cacc/vacc;
    for (v,c,i) in vals {
        let d = c - cacc;
        let mut dmat:Matrix3<f64> = zero();
        for r in 0..3 {
            dmat[(r,r)] += d.norm_squared();
            for c in 0..3 {
                dmat[(r,c)] -= d[r] * d[c];
            }
        }
        iacc = iacc + i + v*dmat;
    }
    (vacc, cacc, iacc)
}

