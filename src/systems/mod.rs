use crate::prelude::*;

mod physics;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(physics::update_positions_system())
        // .add_system(render_entities_system())
        .build()
}
