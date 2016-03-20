/* Notice:	Copyright 2016, The Care Connections Initiative c.i.c.
 * Author: 	Charlie Fyvie-Gauld (cfg@zunautica.org)
 * License: GPLv3 (http://www.gnu.org/licenses/gpl-3.0.txt)
 */

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

fn u8_status_type(byte: u8) -> Option<DvspNodeState> {
	match byte {
		0 => Some(DvspNodeState::Disabled),
		1 => Some(DvspNodeState::Enabled),
		2 => Some(DvspNodeState::Unresponsive),
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

pub struct FrameResponse {	// Response
	pub code: DvspRcode,
}

pub struct FrameNodeStatus {	// Response
	pub code: DvspRcode,
	pub status: DvspNodeState
}

pub struct FrameNetwork { // Response
	pub list: Vec<u8>,
}

pub struct FrameRegister { 	// Request
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
	
	pub fn write_content(&mut self, bytes: &[u8]) -> Result<Success,Failure> {
		
		if bytes.len() > Bounds::PacketContentSize as usize {
			return Err(Failure::OutOfBounds);
		} else {
			push_bytes(&mut self.content, bytes);	
			Ok(Success::Ok)
		}
		
	}
	
	pub fn content_raw(&self) -> &Vec<u8> {
		&self.content
	}
	
	pub fn content_as<T: NetSerial>(&self) -> Result<T,Failure> {
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

	fn deserialise(bytes: &[u8]) -> Result<Packet, Failure> {
		
		let pt = match u8_packet_type(bytes[0]) {
			None => return Err(Failure::InvalidBytes),
			Some(op) => op
		};
		
		let mut p = Packet::new(pt);
		{
			let h = p.mut_header();
			h.msg_part =  deserialise_bool(bytes[1]);
			
			h.msg_size = u32_transmute_le_arr(&bytes[2..6]);
			h.addr_orig = byte_slice_4array(&bytes[6..10]);
			h.addr_dest = byte_slice_4array(&bytes[10..14]);
		}
		Ok(p)
	}
}

impl FrameResponse {

	pub fn new(c: DvspRcode) -> FrameResponse {
		FrameResponse { code: c }
	}

}

impl FrameNodeStatus {
	pub fn new(status: DvspNodeState) -> FrameNodeStatus {
		FrameNodeStatus {
			code: DvspRcode::Ok,
			status: status
		}
	}
}

impl NetSerial for FrameResponse {
	
	fn serialise(&self) -> Vec<u8> {
		
		let mut v: Vec<u8> = Vec::new();
		push_bytes(&mut v, & array_transmute_le_u32(self.code as u32));		
		v
	}

	fn deserialise(bytes: &[u8]) -> Result<FrameResponse,Failure> {

		let rc = match u8_rcode_type(bytes[0]) {
			None => return Err(Failure::InvalidBytes),
			Some(op) => op
		};
		
		Ok(FrameResponse::new(rc))
	}
}


impl NetSerial for FrameNodeStatus {
	
	fn serialise(&self) -> Vec<u8> {
		
		let mut v: Vec<u8> = Vec::new();
		push_bytes(&mut v, & array_transmute_le_u32(self.code as u32));
		v.push(self.status as u8);
		v
	}

	fn deserialise(bytes: &[u8]) -> Result<FrameNodeStatus, Failure> {

		let rc = match u8_rcode_type(bytes[0]) {
			None => return Err(Failure::InvalidBytes),
			Some(op) => op
		};

		let status = match u8_status_type(bytes[4]) {
			None => return Err(Failure::InvalidBytes),
			Some(op) => op			
		};

		Ok(FrameNodeStatus {
				code: rc,
				status: status
		})
	}
}

impl FrameNetwork {
	pub fn new(list: &str) -> FrameNetwork {

		let mut v: Vec<u8> = Vec::new();
		v.extend_from_slice(list.as_bytes());

		FrameNetwork {
			list: v 
		}
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

	fn deserialise(bytes: &[u8]) -> Result<FrameRegister, Failure> {

		let service = match u8_service_type(bytes[3]) {
			None => return Err(Failure::InvalidBytes),
			Some(op) => op
		};
		
		if u8_valid_nodetype(bytes[1]) == false 
		|| bytes[2] > Bounds::FrameRegisterLen as u8 {
			return Err(Failure::OutOfBounds)
		};

		Ok(FrameRegister {
				register: deserialise_bool(bytes[0]),
				ntype: bytes[1],
				len: bytes[2],
				service: service,
				nodereg: String::from(str::from_utf8(&bytes[4..]).unwrap()) // unwrap Dangerzone
		})
	}	
}