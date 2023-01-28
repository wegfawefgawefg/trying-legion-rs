use crate::prelude::*;

#[system(for_each)]
pub fn render_entities(
    pos: &Position,
    #[resource] rl: &mut RaylibHandle,
    #[resource] thread: &RaylibThread,
) {
    let mut draw = rl.begin_drawing(&thread);
    draw.draw_circle(pos.x as i32, pos.y as i32, 10.0, Color::RED);
}
