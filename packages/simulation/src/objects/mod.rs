/*
    These files define how the calculation of the magnetic field is done with 
*/
// use crate::particle::Vec3;
pub mod coil;
use crate::particle::Vec3;

pub trait MangeticObject {
    fn get_B_field(&self, position: Vec3, velocity: Vec3) -> Vec3;
}