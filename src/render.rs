use crate::sim::*;
use hecs::World;
use macroquad::color;
use macroquad::color::LIGHTGRAY;
use macroquad::prelude::{clear_background, draw_line, draw_text, DARKGRAY};
use macroquad::shapes::{draw_circle, draw_rectangle};

const DEVICE_WIDTH: f32 = 200.;
const DEVICE_HEIGHT: f32 = 75.;
fn draw_devices(world: &World) {
    for (_device, (label, trans, ports, color)) in
        world.query::<(&Label, &Transform, &Ports, &Color)>().iter()
    {
        draw_rectangle(trans.0.x, trans.0.y, DEVICE_WIDTH, DEVICE_HEIGHT, color.0);
        draw_text(
            &*label.0.clone(),
            trans.0.x,
            trans.0.y + DEVICE_HEIGHT / 2.2,
            DEVICE_HEIGHT / 1.8,
            color::WHITE,
        );
        let w = DEVICE_WIDTH / (ports.count * 2 + 1) as f32;
        for port_id in 0..ports.count {
            let x_0 = trans.0.x + w * (2 * port_id + 1) as f32;
            let y_0 = trans.0.y + (DEVICE_HEIGHT / 2.0);
            draw_rectangle(x_0, y_0, w, w, DARKGRAY)
        }
    }
}

fn draw_links(world: &World) {
    for (_link, (connects, color)) in world.query::<(&ConnectsPorts, &Color)>().iter() {
        let w_a =
            DEVICE_WIDTH / (world.get::<&Ports>(connects.a.device).unwrap().count * 2 + 1) as f32;
        let w_b =
            DEVICE_WIDTH / (world.get::<&Ports>(connects.b.device).unwrap().count * 2 + 1) as f32;
        let trans_a = world.get::<&Transform>(connects.a.device).unwrap();
        let trans_b = world.get::<&Transform>(connects.b.device).unwrap();
        let id_a = connects.a.index;
        let id_b = connects.b.index;
        let x_a = trans_a.0.x + w_a * ((2 * id_a + 1) as f32 + 0.5);
        let y_a = trans_a.0.y + (DEVICE_HEIGHT / 2.0) + w_a * 0.5;
        let x_b = trans_b.0.x + w_b * ((2 * id_b + 1) as f32 + 0.5);
        let y_b = trans_b.0.y + (DEVICE_HEIGHT / 2.0) + w_b * 0.5;
        draw_line(x_a, y_a, x_b, y_b, 2.0, color.0);

        for (_packet, trans) in world.query::<&Transit>().iter() {
            let dir = if trans.from == connects.b && trans.to == connects.a {
                Some(((x_a, y_a), (x_b, y_b)))
            } else if trans.from == connects.a && trans.to == connects.b {
                Some(((x_b, y_b), (x_a, y_a)))
            } else {
                None
            };
            let r = trans.delay as f32 / trans.delay_full as f32;
            if let Some(((x_1, y_1), (mut x_2, mut y_2))) = dir {
                x_2 = x_1 + r * (x_2 - x_1);
                y_2 = y_1 + r * (y_2 - y_1);
                draw_circle(x_2, y_2, 10., color::RED)
            }
        }
    }
}

pub fn render_system(world: &World) {
    clear_background(LIGHTGRAY);
    draw_devices(world);
    draw_links(world);
}
