use crate::error::PigeonError;
use crate::network::NetworkingProtocol;
use bytes::{Buf, Bytes, BytesMut};
use std::net::SocketAddr;
use std::sync::{Arc};
use async_trait::async_trait;
use tokio::net::{ToSocketAddrs, UdpSocket};
use tokio::sync::Mutex;

pub struct UdpProtocol {
    socket: Option<Arc<Mutex<UdpSocket>>>
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
        self.socket = Some(Arc::new(Mutex::new(UdpSocket::bind(ip_addr).await?)));
        Ok(())
    }

    async fn send<A: ToSocketAddrs + Send>(&self, buf: Bytes, addr: A) -> Result<usize, PigeonError> {
        Ok(self.socket.clone().unwrap().lock().await.send_to(buf.chunk(), addr).await?)
    }

    async fn receive(&self, buf: &mut BytesMut) -> Result<SocketAddr, PigeonError> {
        Ok(self.socket.clone().unwrap().lock().await.recv_from(buf).await?.1)
    }
}