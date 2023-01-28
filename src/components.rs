use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    x: f32,
    y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Velocity {
    dx: f32,
    dy: f32,
}
