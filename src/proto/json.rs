use binrw::{binrw, BinRead, BinResult, BinWrite, NullString};
use binrw::io::BufReader;
use bytes::{Buf, BufMut, BytesMut};
use crate::error::PigeonError;
use crate::proto::ClientProtocol;

pub struct JsonProtocol;

#[binrw]
#[brw(big, magic = b"JHEAD")]
pub struct JsonProtocolHeader {
    op_code: u32,
    data: NullString
}

impl ClientProtocol for JsonProtocol {
    type Header = JsonProtocolHeader;

    fn parse_header<B: BufMut>(&self, data: &mut B) -> Result<Self::Header, PigeonError> {
        Ok(JsonProtocolHeader::read(data)?)
    }

    fn parse_body<B>(&self, header: Self::Header) -> Result<B, PigeonError> {
        Ok(serde_json::from_str(header.data.into())?)
    }
}