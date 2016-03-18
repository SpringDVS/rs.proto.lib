use std::mem::transmute;
use ::serialise::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PacketType {
	Undefined,

	GsnRegisterHost,
	GsnUnregisterHost,

}

fn u8_packet_type(byte: u8) -> Option<PacketType> {
	match byte {
		0 => Some(PacketType::Undefined),
		1 => Some(PacketType::GsnRegisterHost),
		2 => Some(PacketType::GsnUnregisterHost),
		_ => None
	}
}

pub struct PacketHeader {
	pub msg_type: PacketType,
	pub msg_part: u8,
	pub msg_size: u32,
	pub addr_orig: [u8;4],
	pub addr_dest: [u8;4]
	
}

pub struct Packet {
	header: PacketHeader,
}

impl Packet {
	pub fn new(t: PacketType) -> Packet {
		Packet {
			header: PacketHeader {
				msg_type: t,
				msg_part: 0,
				msg_size: 0,
				addr_orig: [0,0,0,0],
				addr_dest: [0,0,0,0],
			}
		}
	}
	
	pub fn header(&self) -> &PacketHeader {
		&self.header
	}
	
	pub fn mut_header(&mut self) -> &mut PacketHeader {
		&mut self.header
	}
}

impl NetSerial for Packet {

	fn serialise(&self) -> Vec<u8> {
		
		let mut v: Vec<u8> = Vec::new();
		let t : u8 =  self.header.msg_type as u8 ;
		v.push( t as u8 );
		
		v.push( self.header.msg_part );
		let bytes: [u8;4] = unsafe { transmute(self.header.msg_size.to_be()) };
		
		push_bytes(&mut v, &bytes);
		push_bytes(&mut v, &self.header.addr_orig);  
		push_bytes(&mut v, &self.header.addr_dest);
		
		v
	}

	fn deserialise(bytes: &[u8]) -> Option<Packet> {
		let op = u8_packet_type(bytes[0]);
		let pt = match op {
			None => return None,
			_ => op.unwrap()
		};
		
		let mut p = Packet::new(pt);
		{
			let h = p.mut_header();
			
			h.msg_size = u32_transmute_be_arr(&bytes[2..6]);
			h.addr_orig = byte_slice_4array(&bytes[6..10]);
			h.addr_dest = byte_slice_4array(&bytes[10..14]);
		}
		Some(p)
	}
}




// ------- Testing  -------- \\

#[test]
fn ts_protocol_packet_serialise_s() {
	let mut p = Packet::new(PacketType::GsnRegisterHost);
	
	{
		let mut h = p.mut_header();
		h.msg_part = 0;
		h.msg_size = 101;
		h.addr_orig = [192,168,1,1];
		h.addr_dest = [192,168,1,2];
	}
	
	let serial = p.serialise();
	
	assert_eq!(1, serial[0]);	// type
	assert_eq!(0, serial[1]);	// part
	
	assert_eq!(0, serial[2]);	// uint32
	assert_eq!(0, serial[3]);
	assert_eq!(0, serial[4]);
	assert_eq!(101, serial[5]);
	
	assert_eq!([192,168,1,1], byte_slice_4array(&serial[6..10]));

	assert_eq!([192,168,1,2], byte_slice_4array(&serial[10..14]));
	
}

#[test]
fn ts_protocol_packet_deserialise_p() {
	// Test good
	let bytes : [u8;14] = [1,0, 0,0,0,33, 127,0,0,1, 192,168,0,255];
	let op = Packet::deserialise(&bytes);
	
	assert!(op.is_some());
	
	let p = op.unwrap();
	
	assert_eq!(PacketType::GsnRegisterHost, p.header().msg_type);
	assert_eq!(33, p.header().msg_size);
	assert_eq!([192,168,0,255], p.header().addr_dest);
}

#[test]
fn ts_protocol_packet_deserialise_f() {
	// Test bad
	let bytes : [u8;14] = [128,0, 0,0,0,33, 127,0,0,1, 192,168,0,255];
	let op = Packet::deserialise(&bytes);
	
	assert!(op.is_none());
}