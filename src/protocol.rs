use std::str;

use ::serialise::*;
use ::enums::*;

// ----- Helper Functions ----- \\

fn u8_packet_type(byte: u8) -> Option<DvspMsgType> {
	match byte {
		0 => Some(DvspMsgType::Undefined),
		1 => Some(DvspMsgType::GsnRegistration),
		8 => Some(DvspMsgType::GsnResponse),
		_ => None
	}
}

fn u8_rcode_type(byte: u8) -> Option<DvspRcode> {
	match byte {
		101 => Some(DvspRcode::NetspaceError),
		102 => Some(DvspRcode::NetspaceDuplication),
		103 => Some(DvspRcode::NetworkError),
		104 => Some(DvspRcode::MalformedContent),

		200 => Some(DvspRcode::Ok),

		_ => None
	}
}

fn u8_service_type(byte: u8) -> Option<DvspService> {
	match byte {
		0 => Some(DvspService::Undefined),
		1 => Some(DvspService::Dvsp),
		2 => Some(DvspService::Http),
		_ => None
	}
}

fn u8_valid_nodetype(field: u8) -> bool {
	if field > Bounds::MaxNodeType as u8 {
		return false
	}
	
	return true
}

// ----- Data Structures ----- \\

pub struct PacketHeader {
	pub msg_type: DvspMsgType,
	pub msg_part: bool,
	pub msg_size: u32,
	pub addr_orig: [u8;4],
	pub addr_dest: [u8;4]
	
}

pub struct Packet {
	header: PacketHeader,
	content: Vec<u8>,
}

pub struct FrameResponse {
	pub code: DvspRcode,
}

pub struct FrameRegister {
	pub register: bool,
	pub ntype: u8,
	pub len: u8,
	pub service: DvspService,
	pub nodereg: String,
}

// ----- Implementations ----- \\

impl Packet {
	pub fn new(t: DvspMsgType) -> Packet {
		Packet {
			header: PacketHeader {
				msg_type: t,
				msg_part: false,
				msg_size: 0,
				addr_orig: [0,0,0,0],
				addr_dest: [0,0,0,0],
			},
			content: Vec::new(),
		}
	}
	
	pub fn header(&self) -> &PacketHeader {
		&self.header
	}
	
	pub fn mut_header(&mut self) -> &mut PacketHeader {
		&mut self.header
	}
	
	pub fn write_content(&mut self, bytes: &[u8]) {
		push_bytes(&mut self.content, bytes)
	}
	
	pub fn content_raw(&self) -> &Vec<u8> {
		&self.content
	}
	
	pub fn content_as<T: NetSerial>(&self) -> Option<T> {
		T::deserialise(self.content.as_slice())
	}
}

impl NetSerial for Packet {

	fn serialise(&self) -> Vec<u8> {
		
		let mut v: Vec<u8> = Vec::new();
		let t : u8 =  self.header.msg_type as u8 ;
		v.push( t as u8 );
		
		v.push( self.header.msg_part as u8 );
		let bytes = array_transmute_le_u32(self.header.msg_size);
		
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
			h.msg_part =  deserialise_bool(bytes[1]);
			
			h.msg_size = u32_transmute_le_arr(&bytes[2..6]);
			h.addr_orig = byte_slice_4array(&bytes[6..10]);
			h.addr_dest = byte_slice_4array(&bytes[10..14]);
		}
		Some(p)
	}
}

impl FrameResponse {

	pub fn new(c: DvspRcode) -> FrameResponse {
		FrameResponse { code: c }
	}

}

impl NetSerial for FrameResponse {
	
	fn serialise(&self) -> Vec<u8> {
		
		let mut v: Vec<u8> = Vec::new();
		push_bytes(&mut v, & array_transmute_le_u32(self.code as u32));		
		v
	}

	fn deserialise(bytes: &[u8]) -> Option<FrameResponse> {
		let op = u8_rcode_type(bytes[0]);
		let rc = match op {
			None => return None,
			_ => op.unwrap()
		};
		
		Some(FrameResponse::new(rc))
	}
}

impl FrameRegister {
	pub fn new(register: bool, ntype: u8, service: DvspService, nodereg: String) -> FrameRegister {
		FrameRegister {
			register: register,
			ntype: ntype,
			len: nodereg.len() as u8,
			service: service,
			nodereg: nodereg,
		}
	} 
}

impl NetSerial for FrameRegister {
	fn serialise(&self) -> Vec<u8> {
		
		let mut v: Vec<u8> = Vec::new();
		v.push(self.register as u8);
		v.push(self.ntype);
		v.push(self.len);
		v.push(self.service as u8);
		push_bytes(&mut v, self.nodereg.as_bytes());		
		v
	}

	fn deserialise(bytes: &[u8]) -> Option<FrameRegister> {

		let service = match u8_service_type(bytes[3]) {
			None => return None,
			Some(op) => op
		};
		
		if u8_valid_nodetype(bytes[1]) == false 
		|| bytes[2] > Bounds::FrameRegisterLen as u8 {
			return None
		}

		Some(FrameRegister {
				register: deserialise_bool(bytes[0]),
				ntype: bytes[1],
				len: bytes[2],
				service: service,
				nodereg: String::from(str::from_utf8(&bytes[4..]).unwrap()) // unwrap Dangerzone
		})
	}	
}



// ------- Testing  -------- \\

#[test]
fn ts_protocol_packet_serialise_s() {
	let mut p: Packet = Packet::new(DvspMsgType::GsnRegistration);
	
	{
		let mut h = p.mut_header();
		h.msg_part = true;
		h.msg_size = 101;
		h.addr_orig = [192,168,1,1];
		h.addr_dest = [192,168,1,2];
	}
	
	let serial = p.serialise();
	
	assert_eq!(1, serial[0]);	// type
	assert_eq!(1, serial[1]);	// part
	
	assert_eq!(101, serial[2]);	// uint32
	assert_eq!(0, serial[3]);
	assert_eq!(0, serial[4]);
	assert_eq!(0, serial[5]);
	
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
	
	assert_eq!(DvspMsgType::GsnRegistration, p.header().msg_type);
	assert_eq!(false, p.header().msg_part);
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

#[test]
fn ts_protocol_frame_response_serialise_p() {
	// Test pass
	let fr = FrameResponse::new(DvspRcode::Ok);
	let bytes = fr.serialise();
	
	assert_eq!([200,0,0,0], byte_slice_4array(&bytes));
}

#[test]
fn ts_protocol_frame_response_deserialis_p() {
	// Test pass
	let bytes = [200,0,0,0];
	let op = FrameResponse::deserialise(&bytes);
	
	assert!(op.is_some());
	
	let frame = op.unwrap();
	
	assert_eq!(DvspRcode::Ok, frame.code);
}

#[test]
fn ts_protocol_frame_response_deserialis_f() {
	// Test fail
	let bytes = [0,200,0,0];
	let op = FrameResponse::deserialise(&bytes);
	
	assert!(op.is_none());
}

#[test]
fn ts_protocol_frame_register_serialise_p() {
	// Test pass
	let fr = FrameRegister::new(
		true,
		DvspNodeType::Org as u8, 
		DvspService::Http, 
		String::from("abc")
	);
	
	let bytes = fr.serialise();
	
	assert_eq!(1, bytes[0]); // register
	assert_eq!(2, bytes[1]); // type
	assert_eq!(3, bytes[2]); // len
	assert_eq!(2, bytes[3]); // service
	
	assert_eq!('a' as u8, bytes[4]);
	assert_eq!('b' as u8, bytes[5]);
	assert_eq!('c' as u8, bytes[6]);
}

#[test]
fn ts_protocol_frame_register_deserialise_p() {
	// Test pass
	let bytes : [u8;7] = [1,2,3,1, 'a' as u8,'b' as u8,'c' as u8];
	let op = FrameRegister::deserialise(&bytes);
	
	assert!(op.is_some());
	
	let frame = op.unwrap();
	
	assert_eq!(true, frame.register);
	assert_eq!(2, frame.ntype);
	assert_eq!(3, frame.len);
	assert_eq!(DvspService::Dvsp, frame.service);
	assert_eq!(String::from("abc"), frame.nodereg);
}

#[test]
fn ts_protocol_frame_register_deserialise_f() {
	// Test fail
	
	// Invalid node type
	let mut bytes : [u8;7] = [1, Bounds::MaxNodeType as u8 + 1 ,3,1, 'a' as u8,'b' as u8,'c' as u8];
	let op1 = FrameRegister::deserialise(&bytes);
	assert!(op1.is_none());
	
	// Invalid node service
	bytes[1] = 2;
	bytes[3] = 100;
	let op2 = FrameRegister::deserialise(&bytes);
	assert!(op2.is_none());
	
	
	// Invalid nodereg len
	bytes[1] = 2;
	bytes[3] = 1;
	bytes[2] = Bounds::FrameRegisterLen as u8 + 1;
	let op3 = FrameRegister::deserialise(&bytes);
	assert!(op3.is_none());
}

#[test]
fn ts_protocol_packet_content_as() {
	let mut p = Packet::new(DvspMsgType::Undefined);
	let fr = FrameResponse::new(DvspRcode::Ok);
	
	p.write_content(fr.serialise().as_slice());
	
	let op = p.content_as::<FrameResponse>();
	
	assert!(op.is_some());
	
	let frame = op.unwrap();
	
	assert_eq!(DvspRcode::Ok, frame.code);
}
