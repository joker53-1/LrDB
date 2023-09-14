use std::ptr::copy_nonoverlapping;

use bytes::{BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

use super::auth::ServerHandshakeCodec;
use crate::{err::ProtocolError, mysql_const::*, server::err::MySQLError, util::get_length};

/// The `CommonPacket` trait contains generic methods for handling the mysql protocol
pub trait CommonPacket {
    fn make_packet_header(&mut self, length: usize, data: &mut [u8], offset: usize);
    fn reset_seq(&mut self);
    fn get_session(&mut self) -> &mut ServerHandshakeCodec;
}

/// Used to reading packet from client side to parseing mysql protocol
pub struct PacketCodec {
    session: ServerHandshakeCodec,
    buf: BytesMut,
    // Whether the payload is greater than MAX_PAYLOAD_LEN
    is_max: bool,
    seq: u8,
}

impl PacketCodec {
    pub fn new(session: ServerHandshakeCodec, init_size: usize) -> Self {
        Self {
            session,
            buf: BytesMut::with_capacity(init_size),
            is_max: false,
            seq: 0,
        }
    }

    #[inline]
    fn set_seq_id(&mut self, buf: &mut [u8], offset: usize) {
        unsafe {
            let data_ptr = buf.as_mut_ptr().add(offset);
            *data_ptr.add(3) = self.seq;
        }

        self.seq = self.seq.wrapping_add(1)
    }

    #[inline]
    fn encode_packet(&mut self, item: &[u8], dst: &mut BytesMut) {
        self.encode_packet_offset(item, dst, 0);
    }

    #[inline]
    fn encode_packet_offset(&mut self, item: &[u8], dst: &mut BytesMut, offset: usize) {
        let length = item.len();
        dst.reserve(length);

        let num = length / MAX_PAYLOAD_LEN;
        let remain = length % MAX_PAYLOAD_LEN;
        let mut offset = offset;

        for i in 0..num {
            dst.put_bytes(0, 4);
            dst.extend_from_slice(&item[i * MAX_PAYLOAD_LEN..(i + 1) * MAX_PAYLOAD_LEN]);
            self.make_packet_header(MAX_PAYLOAD_LEN, dst, offset);

            offset += MAX_PAYLOAD_LEN + 4;
        }

        dst.put_bytes(0, 4);
        dst.extend_from_slice(&item[num * MAX_PAYLOAD_LEN..]);

        self.make_packet_header(remain, dst, offset);
    }
}

impl CommonPacket for PacketCodec {
    #[inline]
    fn make_packet_header(&mut self, length: usize, data: &mut [u8], offset: usize) {
        // we have ensured length is 3bytes, so we can use unsafe block
        unsafe {
            let bytes = *(&(length as u64).to_le() as *const u64 as *const [u8; 8]);
            let data_ptr = data.as_mut_ptr().add(offset);
            copy_nonoverlapping(bytes.as_ptr(), data_ptr, 3);
        }

        self.set_seq_id(data, offset)
    }

    #[inline]
    fn reset_seq(&mut self) {
        self.seq = 0
    }

    fn get_session(&mut self) -> &mut ServerHandshakeCodec {
        &mut self.session
    }
}

impl Decoder for PacketCodec {
    type Item = BytesMut;
    type Error = ProtocolError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.is_empty() || src.len() < 3 {
            return Ok(None);
        }

        let length = get_length(&*src) as usize;

        if 4 + length > src.len() {
            return Ok(None);
        }

        if length == MAX_PAYLOAD_LEN {
            self.is_max = true
        }

        self.seq = self.seq.wrapping_add(1);
        let _ = src.split_to(4);
        self.buf.extend_from_slice(&src.split_to(length));

        if length < MAX_PAYLOAD_LEN {
            return Ok(Some(self.buf.split()));
        }

        self.decode(src)
    }
}

pub enum PacketSend<T> {
    Origin(T),
    Encode(T),
    EncodeOffset(T, usize),
}

impl<T: AsRef<[u8]>> Encoder<PacketSend<T>> for PacketCodec {
    type Error = ProtocolError;

    fn encode(&mut self, item: PacketSend<T>, dst: &mut BytesMut) -> Result<(), Self::Error> {
        match item {
            PacketSend::Origin(data) => dst.extend_from_slice(data.as_ref()),
            PacketSend::Encode(data) => self.encode_packet(data.as_ref(), dst),
            PacketSend::EncodeOffset(data, offset) => {
                self.encode_packet_offset(data.as_ref(), dst, offset)
            }
        };

        Ok(())
    }
}

#[inline]
pub fn make_eof_packet() -> [u8; 9] {
    [5, 0, 0, 0, 0xfe, 0, 0, 2, 0]
}

#[inline]
pub fn ok_packet() -> [u8; 11] {
    [7, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0]
}

#[inline]
pub fn make_err_packet(err: MySQLError) -> Vec<u8> {
    let mut data = BytesMut::with_capacity(128);
    data.extend_from_slice(&[0; 4]);
    data.put_u8(0xff);
    data.extend_from_slice(&[err.code as u8, (err.code >> 8) as u8]);
    data.put_u8(b'#');
    data.extend_from_slice(&err.state);
    data.put_u8(b' ');
    data.extend_from_slice(err.msg.as_bytes());

    data.to_vec()
}
