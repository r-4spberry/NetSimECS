mod sim;
use hecs;
use macroquad::prelude::*;

#[macroquad::main("Networking")]
async fn main() {
    let mut world = hecs::World::new();
    let pc_a = sim::spawn_spammer(&mut world, "PC_1".to_string(), 1, vec![1, 2, 3], 20);
    let pc_b = sim::spawn_sink(&mut world, "PC_2".to_string(), 1);
    let hub = sim::spawn_hub(&mut world, "Hub_1".to_string(), 4);
    sim::spawn_link(&mut world, pc_a, 0, hub, 0, 2);
    sim::spawn_link(&mut world, hub, 1, pc_b, 0, 2);
    let mut tick = 0;
    loop {
        println!("Tick {}", tick);
        clear_background(LIGHTGRAY);
        sim::tick(&mut world);
        next_frame().await;
        tick += 1;
    }
}
