use crate::error::PigeonError;
use crate::network::NetworkingProtocol;
use async_trait::async_trait;
use bytes::{Buf, Bytes, BytesMut};
use std::net::{SocketAddr};
use std::sync::Arc;
use tokio::net::{ToSocketAddrs, UdpSocket};

#[derive(Clone)]
pub struct UdpProtocol {
    socket: Option<Arc<UdpSocket>>
}

impl UdpProtocol {
    pub fn new() -> UdpProtocol {
        UdpProtocol {
            socket: None
        }
    }
}

#[async_trait]
impl NetworkingProtocol for UdpProtocol {
    async fn listen<A: ToSocketAddrs + Send>(&mut self, ip_addr: A) -> Result<(), PigeonError> {
        self.socket = Some(Arc::new(UdpSocket::bind(ip_addr).await?));
        Ok(())
    }

    async fn send<A: ToSocketAddrs + Send>(&self, buf: Bytes, addr: A) -> Result<usize, PigeonError> {
        Ok(self.socket.clone().unwrap().send_to(buf.chunk(), addr).await?)
    }

    async fn receive(&self, buf: &mut BytesMut) -> Result<SocketAddr, PigeonError> {
        Ok(self.socket.clone().unwrap().recv_from(buf).await?.1)
    }
}