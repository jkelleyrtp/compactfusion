extern crate dimensioned as dim;
pub use particle::Vec3;
use em::*;
// use crate::objects::Coil;
// use crate::objects::coil::Coil;
mod objects;
mod particle;
pub use objects::coil::Coil;


use dim::si::{Meter, M, S};

// #[gpu_use]
fn main() {

    let coil_1 = Coil {
        current: 1.0 * dim::si::A,
        radius: 1.0 * dim::si::M,
        position: Vec3::default() * dim::si::M,
        orientation: Vec3::default() * dim::si::M
    };


    let mut data = vec![40.0; 1000000];



    gpu_do!(load(data));
    gpu_do!(launch());
    for i in 0..1000000 {
        data[i] = data[i] * 10.0;
    }
    gpu_do!(read(data));

    println!("{:?}", data.len());
}

// fn main() {
// 	// i = get_group_id
// 	// j = get_local_id

// 	for (i, j) in x.enumerate().map(|(i, _)| (i / 10, i % 10)) {

// 	}

// 	for (i, chunk) in x.chunks(10).enumerate() {
// 		let mut scratch = vec![0.0; 10];
// 		for (j, _) in chunk.enumerate() {
			
// 		}
// 	}


// }