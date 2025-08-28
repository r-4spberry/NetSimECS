use ahash;
use hecs::{CommandBuffer, Entity, World};
use macroquad::color;
use macroquad::math::Vec2;

pub struct Label(pub String);
pub struct Transform(pub Vec2);
pub struct Color(pub color::Color);

pub struct Ports {
    pub count: usize,
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PortRef {
    pub device: Entity,
    pub index: usize,
}
pub struct HubLogic;

pub struct Emitter {
    pub payload: Vec<u8>,
    pub port: usize,
    pub interval_t: usize,
    pub timer_t: usize,
}
pub struct ConsoleSink;

pub struct ConnectsPorts {
    pub a: PortRef,
    pub b: PortRef,
}
pub struct LatencyT(usize);

pub struct Payload(Vec<u8>);
#[derive(PartialEq, Eq)]
pub enum PortState {
    JustArrived,
    ReadyToSend,
}

pub struct AtPort {
    pub port: PortRef,
    pub state: PortState,
}
pub struct Transit {
    pub from: PortRef,
    pub to: PortRef,
    pub delay: usize,
    pub delay_full: usize,
}

pub fn spawn_spammer(
    world: &mut World,
    label: String,
    num_ports: usize,
    payload: Vec<u8>,
    interval_t: usize,
    x: f32,
    y: f32,
) -> Entity {
    if num_ports == 0 {
        panic!("num_ports must be > 0");
    }
    world.spawn((
        Label(label),
        Transform(Vec2::new(x, y)),
        Color(color::GRAY),
        Ports { count: num_ports },
        Emitter {
            payload,
            port: 0,
            interval_t,
            timer_t: 0,
        },
    ))
}

pub fn spawn_sink(world: &mut World, label: String, n_ports: usize, x: f32, y: f32) -> Entity {
    if n_ports == 0 {
        panic!("n_ports must be > 0");
    }
    world.spawn((
        Label(label),
        Transform(Vec2::new(x, y)),
        Color(color::GRAY),
        Ports { count: n_ports },
        ConsoleSink,
    ))
}

pub fn spawn_link(
    world: &mut World,
    a_device: Entity,
    a_port: usize,
    b_device: Entity,
    b_port: usize,
    latency_t: usize,
) -> Entity {
    world.spawn((
        Transform,
        Color(color::GREEN),
        ConnectsPorts {
            a: PortRef {
                device: a_device,
                index: a_port,
            },
            b: PortRef {
                device: b_device,
                index: b_port,
            },
        },
        LatencyT(latency_t),
    ))
}

pub fn spawn_hub(world: &mut World, label: String, n_ports: usize, x: f32, y: f32) -> Entity {
    if n_ports == 0 {
        panic!("n_ports must be > 0");
    }
    world.spawn((
        Label(label),
        Transform(Vec2::new(x, y)),
        Color(color::GRAY),
        Ports { count: n_ports },
        HubLogic,
    ))
}

pub fn spawn_packet(cb: &mut CommandBuffer, payload: Vec<u8>, machine: Entity, port: usize) {
    cb.spawn((
        Transform,
        Payload(payload),
        AtPort {
            port: PortRef {
                device: machine,
                index: port,
            },
            state: PortState::ReadyToSend,
        },
    ))
}

fn emitter_system(world: &mut World, cb: &mut CommandBuffer) {
    for (entity, (emitter, _ports)) in world.query::<(&mut Emitter, &Ports)>().iter() {
        emitter.timer_t += 1;
        if emitter.timer_t >= emitter.interval_t {
            emitter.timer_t = 0;
            spawn_packet(cb, emitter.payload.clone(), entity, emitter.port);
            println!("Spawned a packet");
        }
    }
}
fn link_depart_system(world: &mut World, cb: &mut CommandBuffer) {
    for (_link, (conn, lat)) in world.query::<(&ConnectsPorts, &LatencyT)>().iter() {
        for (pkt, at) in world.query::<&AtPort>().iter() {
            if at.state == PortState::JustArrived {
                continue;
            }

            let dir = if at.port == conn.a {
                Some((conn.a, conn.b))
            } else if at.port == conn.b {
                Some((conn.b, conn.a))
            } else {
                None
            };

            if let Some((from, to)) = dir {
                println!("Starting to move a packet");
                cb.remove_one::<AtPort>(pkt);
                cb.insert_one(
                    pkt,
                    Transit {
                        from,
                        to,
                        delay: lat.0,
                        delay_full: lat.0,
                    },
                );
            }
        }
    }
}

fn link_move_system(world: &mut World, cb: &mut CommandBuffer) {
    for (trans_packet, transit) in world.query::<&mut Transit>().iter() {
        if transit.delay > 0 {
            println!("Moving a packet");
            transit.delay -= 1;
        } else {
            println!("Packet arrived");
            cb.remove_one::<Transit>(trans_packet);
            cb.insert_one(
                trans_packet,
                AtPort {
                    port: transit.to,
                    state: PortState::JustArrived,
                },
            );
        }
    }
}

fn hub_propagate_system(world: &mut World, cb: &mut CommandBuffer) {
    for (packet, (at, payload)) in world.query::<(&AtPort, &Payload)>().iter() {
        let device = at.port.device;
        if world.get::<&HubLogic>(at.port.device).is_ok() {
            let hub = device;
            let ports = world.get::<&Ports>(at.port.device).unwrap();
            if at.state == PortState::ReadyToSend || at.port.device != hub {
                continue;
            }
            println!("Propagating packets");
            for i in 0..ports.count {
                if i == at.port.index {
                    continue;
                }
                spawn_packet(cb, payload.0.clone(), hub, i)
            }
            cb.despawn(packet)
        }
    }
}
fn sink_consume_system(world: &mut World, cb: &mut CommandBuffer) {
    let mut sinks = ahash::AHashSet::new();
    for (e, _) in world.query::<&ConsoleSink>().iter() {
        sinks.insert(e);
    }

    for (pkt, (at, payload)) in world.query::<(&AtPort, &Payload)>().iter() {
        if sinks.contains(&at.port.device) && at.state == PortState::JustArrived {
            println!("{:?}", payload.0);
            cb.despawn(pkt);
        }
    }
}
pub fn tick(world: &mut World) {
    let mut cb = CommandBuffer::new();

    emitter_system(world, &mut cb);
    hub_propagate_system(world, &mut cb);
    link_depart_system(world, &mut cb);
    link_move_system(world, &mut cb);
    sink_consume_system(world, &mut cb);

    cb.run_on(world);
}
