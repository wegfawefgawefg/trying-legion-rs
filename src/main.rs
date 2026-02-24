/*
    lets just make some entities bounce around the screen without collisions
*/

mod components;
mod systems;

mod prelude {
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub use raylib::prelude::*;
    pub const SCREEN_DIMS: raylib::math::Vector2 = raylib::math::Vector2 { x: 640.0, y: 480.0 };
    pub use crate::components::*;
    pub use crate::systems::*;
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    systems: Schedule,
}

pub fn init(ecs: &mut World) {
    let _entities: &[Entity] = ecs.extend(vec![
        (
            Position { x: 320.0, y: 240.0 },
            Velocity { dx: 1.0, dy: 0.5 },
        ),
        // (Position { x: 0.0, y: 0.0 }, Velocity { dx: 0.01, dy: 0.01 }),
        // (Position { x: 1.0, y: 1.0 }, Velocity { dx: 0.01, dy: 0.01 }),
        // (Position { x: 2.0, y: 2.0 }, Velocity { dx: 0.01, dy: 0.01 }),
    ]);
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        // let mut rng = RandomNumberGenerator::new();
        // spawn_player(&mut ecs, map_builder.player_start);
        init(&mut ecs);

        // resources.insert(map_builder.map);
        // resources.insert(Camera::new(map_builder.player_start));
        Self {
            ecs,
            resources,
            systems: build_scheduler(),
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, ecs").build();
    rl.set_target_fps(60);
    let mut state = State::new();

    while !rl.window_should_close() {
        state.systems.execute(&mut state.ecs, &mut state.resources);

        {
            let mut draw = rl.begin_drawing(&thread);
            draw.clear_background(Color::WHITE);
            draw.draw_text("Hello, ecs!", 12, 12, 20, Color::BLACK);
            for pos in <&Position>::query().iter(&state.ecs) {
                draw.draw_circle(pos.x as i32, pos.y as i32, 10.0, Color::RED);
            }
        }
    }
}
