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