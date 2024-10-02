use std::net::{SocketAddr};
use bytes::Bytes;
use tokio::sync::mpsc::Sender;
use crate::proto::ClientProtocol;

pub trait Service<CP: ClientProtocol> {
    type Body;

    fn start();
    fn handle(&self, body: Self::Body, context: Context);
    fn end();
}

pub struct Context {
    addr: SocketAddr,
    sender: Sender<(Bytes, SocketAddr)>
}

impl Context {
    pub(crate) fn new(addr: SocketAddr, sender: Sender<(Bytes, SocketAddr)>) -> Context {
        Context {
            addr,
            sender,
        }
    }

    pub async fn send(&self, bytes: Bytes, addr: SocketAddr) {
        self.sender.send((bytes, addr)).await.unwrap();
    }

    pub fn addr(&self) -> SocketAddr {
        self.addr
    }
}

pub trait ServiceHandler<CP: ClientProtocol> {
    fn handle(&self, header: CP::Header, context: Context);
    fn register_service<S: Service<CP>>(&self, service: S);
}