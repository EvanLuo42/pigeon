#[cfg(feature = "json")]
pub mod json;

pub mod protobuf;

use bytes::{BufMut};

pub trait ClientProtocol {
    type Header;

    fn parse_header<B: BufMut>(&self, data: &mut B) -> Self::Header;
    fn parse_body<B>(&self, header: Self::Header) -> B;
}