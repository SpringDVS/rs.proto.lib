use std::mem::transmute;
use ::serialise::NetSerial;
use ::serialise::push_bytes;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PacketType {
	Undefined,

	GsnRegisterHost,
	GsnUnregisterHost,

}

pub struct Packet {
	pub msg_type: PacketType,
	pub msg_part: u8,
	pub msg_size: u32,
	pub addr_orig: [u8;4],
	pub addr_dest: [u8;4]
}

fn u8_packet_type(byte: u8) -> Option<PacketType> {
	match byte {
		0 => Some(PacketType::Undefined),
		1 => Some(PacketType::GsnRegisterHost),
		2 => Some(PacketType::GsnUnregisterHost),
		_ => None
	}
}

impl Packet {
	pub fn new(t: PacketType) -> Packet {
		Packet {
			msg_type: t,
			msg_part: 0,
			msg_size: 0,
			addr_orig: [0,0,0,0],
			addr_dest: [0,0,0,0]
		}
	} 
}

impl NetSerial for Packet {

	fn serialise(&self) -> Vec<u8> {
		
		let mut v: Vec<u8> = Vec::new();
		let t : u8 =  self.msg_type as u8 ;
		v.push( t as u8 );
		
		v.push( self.msg_part );
		let bytes: [u8;4] = unsafe { transmute(self.msg_size.to_be()) };
		
		push_bytes(&mut v, &bytes);
		push_bytes(&mut v, &self.addr_orig);  
		push_bytes(&mut v, &self.addr_dest);
		
		v
	}

	fn deserialise(bytes: &[u8]) -> Option<Packet> {
		let op = u8_packet_type(bytes[0]);
		let pt = match op {
			None => return None,
			_ => op.unwrap()
		};
		
		let p = Packet::new(pt);
		
		Some(p)
	}
}




// ------- Testing  -------- \\

#[test]
fn ts_protocol_packet_serialise_s() {
	let mut p = Packet::new(PacketType::GsnRegisterHost);
	p.msg_part = 0;
	p.msg_size = 101;
	p.addr_orig = [192,168,1,1];
	p.addr_dest = [192,168,1,2];
	
	let serial = p.serialise();
	
	assert_eq!(1, serial[0]);	// type
	assert_eq!(0, serial[1]);	// part
	
	assert_eq!(0, serial[2]);	// int32
	assert_eq!(0, serial[3]);
	assert_eq!(0, serial[4]);
	assert_eq!(101, serial[5]);
	
	assert_eq!(192, serial[6]); // addr_orig
	assert_eq!(168, serial[7]);
	assert_eq!(1, serial[8]);
	assert_eq!(1, serial[9]);
	
	assert_eq!(192, serial[10]); // addr_dest
	assert_eq!(168, serial[11]);
	assert_eq!(1, serial[12]);
	assert_eq!(2, serial[13]);
	
}

#[test]
fn ts_protocol_packet_deserialise_p() {
	// Test good
	let bytes : [u8;14] = [1,0, 0,0,0,33, 127,0,0,1, 192,168,0,255];
	let op = Packet::deserialise(&bytes);
	
	assert!(op.is_some());
	
	let p = op.unwrap();
	
	assert_eq!(PacketType::GsnRegisterHost, p.msg_type);
}

#[test]
fn ts_protocol_packet_deserialise_f() {
	// Test bad
	let bytes : [u8;14] = [128,0, 0,0,0,33, 127,0,0,1, 192,168,0,255];
	let op = Packet::deserialise(&bytes);
	
	assert!(op.is_none());
}