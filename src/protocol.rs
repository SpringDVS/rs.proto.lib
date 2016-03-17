use std::mem::transmute;
use ::serialise::NetSerial;

#[derive(Copy, Clone)]
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

fn push_bytes(v: &mut Vec<u8>, bytes: &[u8]) {
	for b in bytes {
		v.push(*b)
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

	//fn deserialise(bytes: [u8]) -> Packet {
	//	let r = bytes[0]	;
	//}
}




#[test]
fn ts_protocol_packet_serialise() {
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
