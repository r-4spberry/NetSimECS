mod render;
mod sim;

use hecs;
use macroquad::prelude::*;

#[macroquad::main("Networking")]
async fn main() {
    let mut world = hecs::World::new();
    let pc_a = sim::spawn_spammer(
        &mut world,
        "PC_1".to_string(),
        8,
        vec![1, 2, 3],
        20,
        30.,
        30.,
    );
    let pc_b = sim::spawn_sink(&mut world, "PC_2".to_string(), 8, 500., 30.);
    let hub = sim::spawn_hub(&mut world, "Hub_1".to_string(), 4, 250., 400.);
    sim::spawn_link(&mut world, pc_a, 0, hub, 0, 50);
    sim::spawn_link(&mut world, hub, 1, pc_b, 0, 50);
    let mut tick = 0;
    loop {
        println!("Tick {}", tick);
        sim::tick(&mut world);
        render::render_system(&world);
        next_frame().await;
        tick += 1;
    }
}
