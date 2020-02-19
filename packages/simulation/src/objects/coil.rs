use crate::Vec3;

use dimensioned as dim;
use dim::si::{Meter, Ampere};
pub struct Coil {
    /// Current flowing through a Coil
    pub radius: Meter<f64>,

    /// Radius of the coil 
    pub current: Ampere<f64>,

    /// Position of the center of the coil
    pub position: Meter<Vec3>,

    /// Orientation of the coil 
    /// 
    /// The coil is in the orientation of the xy plane with the axis of rotation being the z axis
    pub orientation: Meter<Vec3>,
}

