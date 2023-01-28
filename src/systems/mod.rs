use crate::prelude::*;

mod components;
mod physics;

pub fn build_scheduler() -> Schedule {
    let mut schedule = Schedule::builder()
        .add_system(update_positions_system())
        // .add_system(render_entities_system())
        .build();
}
