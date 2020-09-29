use serde::{Serialize,Deserialize};

use crate::asteroid::*;
use crate::ship::*;

#[derive(Deserialize)]
pub struct ShipArgs(pub u8, pub f64, pub f64);

#[derive(Deserialize)]
pub struct AsteroidArgs(pub f64, pub f64, pub f64);

#[derive(Deserialize)]
pub struct SpawnerArgs();

enum ConstructorArgs {

}