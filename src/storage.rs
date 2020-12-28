use serde::{Serialize,Deserialize};

use crate::asteroid::*;
use crate::ship::*;

#[derive(Debug,Deserialize)]
pub struct ShipArgs(pub usize, pub f64, pub f64, #[serde(default)] pub (f64, f64), #[serde(default)] pub f64);

#[derive(Debug,Deserialize)]
pub struct AsteroidArgs(pub f64, pub f64, pub f64);

#[derive(Debug,Deserialize)]
pub struct WallArgs(pub i64, pub i64, pub i64, pub i64, pub (i64, i64));

#[derive(Debug,Deserialize)]
pub struct SpawnerArgs();

enum ConstructorArgs {

}