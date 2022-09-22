// Code copied from https://github.com/ssnover/netty
use byteorder::{NetworkEndian, ReadBytesExt, WriteBytesExt};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use std::io;
use std::io::{Read, Write};
use std::net::Ipv4Addr;

pub const HEADER_SIZE: usize = 20;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub enum ProtocolType {
    Icmp = 1,
    Tcp = 6,
    Udp = 11,
}

#[derive(Clone, Copy, Debug)]
pub struct Header {
    pub version: u8,
    pub internet_header_len: u8,
    pub type_of_service: u8,
    pub datagram_len: u16,
    pub id: u16,
    pub control_flags: u8,
    pub fragment_offset: u16,
    pub time_to_live: u8,
    pub proto: ProtocolType,
    pub checksum: u16,
    pub src_addr: Ipv4Addr,
    pub dst_addr: Ipv4Addr,
}

impl Header {
    pub fn decode(buf: &[u8]) -> io::Result<(Self, &[u8])> {
        let mut cursor = io::Cursor::new(buf);
        let version_and_ihl = cursor.read_u8()?;
        let version = version_and_ihl & 0xF;
        let internet_header_len = version_and_ihl >> 4;
        let type_of_service = cursor.read_u8()?;
        let datagram_len = cursor.read_u16::<NetworkEndian>()?;
        let id = cursor.read_u16::<NetworkEndian>()?;
        let flags_and_frag_offset = cursor.read_u16::<NetworkEndian>()?;
        let control_flags = (flags_and_frag_offset & 0b111) as u8;
        let fragment_offset = flags_and_frag_offset >> 3;
        let time_to_live = cursor.read_u8()?;
        let proto = match FromPrimitive::from_u8(cursor.read_u8()?) {
            Some(proto) => proto,
            None => return Err(io::ErrorKind::Unsupported.into()),
        };
        let checksum = cursor.read_u16::<NetworkEndian>()?;
        let mut src_addr_octets = [0u8; 4];
        cursor.read_exact(&mut src_addr_octets)?;
        let src_addr = Ipv4Addr::from(src_addr_octets);
        let mut dst_addr_octets = [0u8; 4];
        cursor.read_exact(&mut dst_addr_octets)?;
        let dst_addr = Ipv4Addr::from(dst_addr_octets);

        Ok((
            Header {
                version,
                internet_header_len,
                type_of_service,
                datagram_len,
                id,
                control_flags,
                fragment_offset,
                time_to_live,
                proto,
                checksum,
                src_addr,
                dst_addr,
            },
            &buf[HEADER_SIZE..],
        ))
    }

    pub fn encode(self, buf: &mut [u8]) -> io::Result<usize> {
        let mut cursor = io::Cursor::new(buf);
        cursor.write_u8((self.version & 0xF) | (self.internet_header_len << 4))?;
        cursor.write_u8(self.type_of_service)?;
        cursor.write_u16::<NetworkEndian>(self.datagram_len)?;
        cursor.write_u16::<NetworkEndian>(self.id)?;
        cursor.write_u16::<NetworkEndian>(
            (self.control_flags as u16 & 0b111) | (self.fragment_offset << 3),
        )?;
        cursor.write_u8(self.time_to_live)?;
        cursor.write_u8(self.proto.to_u8().unwrap())?;
        cursor.write_u16::<NetworkEndian>(self.checksum)?;
        cursor.write_all(&self.src_addr.octets())?;
        cursor.write_all(&self.dst_addr.octets())?;
        Ok(HEADER_SIZE)
    }
}
