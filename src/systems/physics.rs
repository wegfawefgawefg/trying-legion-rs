use crate::prelude::*;

#[system(for_each)]
pub fn update_positions(pos: &mut Position, vel: &Velocity) {
    pos.x += vel.dx;
    pos.y += vel.dy;
}
