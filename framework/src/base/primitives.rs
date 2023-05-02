use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Copy, Clone, PartialEq)]
/// A two-dimensional point in space
pub struct Point(pub [f64; 2]);

#[derive(Serialize, Deserialize, JsonSchema, Debug, PartialEq, Copy, Clone)]
/// Dimensions in x and y direction
pub struct Dimensions(pub [f64; 2]);
