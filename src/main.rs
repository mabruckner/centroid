extern crate nalgebra;
extern crate stl;
extern crate clap;
extern crate centroid;

use clap::{
    App,
    Arg
};

use nalgebra::{
    Vector3
};
use std::fs::File;
use std::path::Path;

fn atob(val: [f32; 3]) -> Vector3<f64> {
    Vector3::new(val[0] as f64, val[1] as f64, val[2] as f64)
}

fn main() {
    let matches = App::new("centroid")
                        .about("calculates the center of mass and volume of an stl file.")
                        .arg(Arg::with_name("file")
                             .required(true)
                             .value_name("FILE")
                             .help("The path to the stl file to analyze"))
                        .get_matches();

    let filename = matches.value_of("file").unwrap();
    let mut stlfile = File::open(Path::new(filename)).expect("error while opening stl file");
    let binfile = stl::read_stl(&mut stlfile).unwrap();

    let mut tris = Vec::new();
    for x in binfile.triangles {
        tris.push([x.v1, x.v2, x.v3]);
    }

    let mut thing = tris.iter().map(|x| {
        [atob(x[0]), atob(x[1]), atob(x[2])]
    });

    let (volume, centroid, moment) = centroid::collection_vals(&mut thing);
    println!("Volume: {}", volume);
    println!("Centroid:\n\tx: {}\n\ty: {}\n\tz: {}", centroid.x, centroid.y, centroid.z);
    println!("Moment of Inertia:");
    for r in 0..3 {
        println!("\t[{}\t{}\t{}]", moment[(r,0)], moment[(r,1)], moment[(r,2)]);
    }
}
