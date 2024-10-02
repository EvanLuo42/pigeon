use crate::network::NetworkingProtocol;
use crate::proto::ClientProtocol;
use crate::service::{Service, ServiceHandler};
use bytes::BytesMut;
use tokio::net::ToSocketAddrs;
use crate::error::PigeonError;

pub mod network;
pub mod proto;
pub mod service;
pub mod error;

pub struct Pigeon<NP, CP, SH>
where
    NP: NetworkingProtocol,
    CP: ClientProtocol,
    SH: ServiceHandler<CP>
{
    networking_protocol: NP,
    client_protocol: CP,
    service_handler: SH
}

impl<NP, CP, SH> Pigeon<NP, CP, SH>
where
    NP: NetworkingProtocol,
    CP: ClientProtocol,
    SH: ServiceHandler<CP>
{
    pub fn new(networking_protocol: NP, client_protocol: CP, service_handler: SH) -> Pigeon<NP, CP, SH> {
        Pigeon {
            networking_protocol,
            client_protocol,
            service_handler
        }
    }

    pub async fn start<A: ToSocketAddrs + Send>(&mut self, ip_addr: A) -> Result<(), PigeonError> {
        self.networking_protocol.listen(ip_addr).await?;

        loop {
            let mut buf = BytesMut::with_capacity(512);
            self.networking_protocol.receive(&mut buf).await?;

            let header = self.client_protocol.parse_header(&mut buf);
            self.service_handler.handle(header);
        }
    }

    pub fn register_service<S: Service<CP>>(&self, service: S) {
        self.service_handler.register_service(service);
    }
}