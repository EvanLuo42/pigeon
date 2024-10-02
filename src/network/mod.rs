pub mod udp;
pub mod tcp;
pub mod kcp;

use std::net::SocketAddr;
use async_trait::async_trait;
use bytes::{Bytes, BytesMut};
use tokio::net::ToSocketAddrs;
use crate::error::PigeonError;

#[async_trait]
pub trait NetworkingProtocol {
    async fn listen<A: ToSocketAddrs + Send>(&mut self, ip_addr: A) -> Result<(), PigeonError>;
    async fn send<A: ToSocketAddrs + Send>(&self, buf: Bytes, addr: A) -> Result<usize, PigeonError>;
    async fn receive(&self, buf: &mut BytesMut) -> Result<SocketAddr, PigeonError>;
}