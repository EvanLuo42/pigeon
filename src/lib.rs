use bevy_ecs::prelude::*;
use bytes::{Buf, BytesMut};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use bevy_ecs::event::event_update_system;
use tokio::io::{split, WriteHalf};
use tokio::net::{TcpListener, TcpStream};
use tokio_stream::StreamExt;
use tokio_util::codec::LengthDelimitedCodec;

pub struct Server {
    world: World,
    schedule: Schedule
}

impl Server {
    pub fn new(addr: String) -> Server {
        let mut world = World::new();
        let mut schedule = Schedule::default();
        world.insert_resource(Events::<ConnectionEvent>::default());
        world.insert_resource(Events::<TlvParseEvent>::default());
        schedule
            .add_systems(event_update_system)
            .add_systems(on_connection)
            .add_systems(on_parse_tlv);
        tokio::spawn(async move {
            let listener = TcpListener::bind(addr).await.unwrap();
            while let Ok((stream, addr)) = listener.accept().await {
                let (reader, writer) = split(stream);
                let mut connection_events = Events::<ConnectionEvent>::default();
                connection_events.send(ConnectionEvent {
                    writer: Arc::new(Mutex::new(writer)),
                    addr
                });
                tokio::spawn(async move {
                    let mut frame_read = LengthDelimitedCodec::builder()
                        .length_field_type::<u16>()
                        .length_field_offset(2)
                        .length_adjustment(4)
                        .num_skip(0)
                        .new_read(reader);
                    while let Some(Ok(message)) = frame_read.next().await {
                        let mut tlv_parse_events = Events::<TlvParseEvent>::default();
                        tlv_parse_events.send(TlvParseEvent(message.try_into().unwrap()));
                    }
                });
            }
        });
        Server {
            world,
            schedule
        }
    }

    pub fn run(&mut self) {
        loop {
            self.schedule.run(&mut self.world);
        }
    }
}

fn on_connection(mut commands: Commands, mut connection_events: EventReader<ConnectionEvent>) {
    for ConnectionEvent { addr, writer } in connection_events.read() {
        commands.spawn(Connection {
            addr: *addr,
            writer: writer.clone()
        });
    }
}

fn on_parse_tlv(mut receive_tlv_events: EventReader<TlvParseEvent>) {
    for TlvParseEvent(message) in receive_tlv_events.read() {
        println!("{:?}", message)
    }
}

#[derive(Event)]
struct ConnectionEvent {
    addr: SocketAddr,
    writer: Arc<Mutex<WriteHalf<TcpStream>>>
}

#[derive(Event)]
struct TlvParseEvent(TlvMessage);

#[derive(Component)]
struct Connection {
    addr: SocketAddr,
    writer: Arc<Mutex<WriteHalf<TcpStream>>>
}

#[derive(Debug)]
struct TlvMessage {
    typ: u16,
    length: u16,
    value: BytesMut
}

impl TryFrom<BytesMut> for TlvMessage {
    type Error = ();

    fn try_from(value: BytesMut) -> Result<Self, Self::Error> {
        let mut message = value;
        if message.len() < 4 {
            return Err(())
        }
        let typ = message.get_u16();
        let length = message.get_u16();
        if message.len() != length as usize {
            return Err(())
        }
        let value = message;
        Ok(TlvMessage {
            typ,
            length,
            value
        })
    }
}