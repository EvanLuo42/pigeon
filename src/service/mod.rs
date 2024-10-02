use crate::proto::ClientProtocol;

pub trait Service<CP: ClientProtocol> {
    type Body;

    fn start();
    fn handle(&self, body: Self::Body);
    fn end();
}

pub trait ServiceHandler<CP: ClientProtocol> {
    fn handle(&self, header: CP::Header);
    fn register_service<S: Service<CP>>(&self, service: S);
}