use crate::error::PigeonError;
use crate::network::NetworkingProtocol;
use crate::proto::ClientProtocol;
use crate::service::{Context, Service, ServiceHandler};
use bytes::{Bytes, BytesMut};
use std::net::{SocketAddr};
use tokio::net::ToSocketAddrs;
use tokio::sync::mpsc::{channel, Sender};

pub mod network;
pub mod proto;
pub mod service;
pub mod error;

pub struct Pigeon<NP, CP, SH> {
    networking_protocol: NP,
    client_protocol: CP,
    service_handler: SH,
    channel_sender: Option<Sender<(Bytes, SocketAddr)>>,
}

impl<NP, CP, SH> Pigeon<NP, CP, SH>
where
    NP: NetworkingProtocol + Send + Sync + 'static,
    CP: ClientProtocol,
    SH: ServiceHandler<CP>
{
    pub fn new(networking_protocol: NP, client_protocol: CP, service_handler: SH) -> Pigeon<NP, CP, SH> {
        Pigeon {
            networking_protocol,
            client_protocol,
            service_handler,
            channel_sender: None,
        }
    }

    pub async fn start<A: ToSocketAddrs + Send>(&mut self, ip_addr: A) -> Result<(), PigeonError> {
        let (tx, mut rx) = channel(100);
        self.channel_sender = Some(tx);
        self.networking_protocol.listen(ip_addr).await?;
        let np = self.networking_protocol.clone();
        tokio::spawn(async move {
            loop {
                while let Some((bytes, addr)) = rx.recv().await {
                    np.send(bytes, addr).await.unwrap();
                }
            }
        });

        loop {
            let mut buf = BytesMut::with_capacity(512);
            let addr = self.networking_protocol.receive(&mut buf).await?;

            let header = self.client_protocol.parse_header(&mut buf);
            let context = Context::new(addr, self.channel_sender.clone().unwrap());
            self.service_handler.handle(header, context);
        }
    }

    pub fn register_service<S: Service<CP>>(&self, service: S) {
        self.service_handler.register_service(service);
    }
}