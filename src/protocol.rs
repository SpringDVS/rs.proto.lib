/* Notice:	Copyright 2016, The Care Connections Initiative c.i.c.
 * Author: 	Charlie Fyvie-Gauld (cfg@zunautica.org)
 * License: GPLv3 (http://www.gnu.org/licenses/gpl-3.0.txt)
 */

use std::str;

use ::serialise::*;
use ::enums::*;

pub type Ipv4 = [u8;4];
pub type Ipv6 = [u8;6];
pub type NodeTypeField = u8;

// ----- Helper Functions ----- \\

pub fn u8_packet_type(byte: u8) -> Option<DvspMsgType> {
	match byte {
		0 => Some(DvspMsgType::Undefined),
		1 => Some(DvspMsgType::GsnRegistration),
		8 => Some(DvspMsgType::GsnResponse),
		_ => None
	}
}

pub fn u32_rcode_type(bytes: u32) -> Option<DvspRcode> {
	match bytes {
		101 => Some(DvspRcode::NetspaceError),
		102 => Some(DvspRcode::NetspaceDuplication),
		103 => Some(DvspRcode::NetworkError),
		104 => Some(DvspRcode::MalformedContent),

		200 => Some(DvspRcode::Ok),

		_ => None
	}
}

pub fn u8_service_type(byte: u8) -> Option<DvspService> {
	match byte {
		0 => Some(DvspService::Undefined),
		1 => Some(DvspService::Dvsp),
		2 => Some(DvspService::Http),
		_ => None
	}
}

pub fn u8_status_type(byte: u8) -> Option<DvspNodeState> {
	match byte {
		0 => Some(DvspNodeState::Disabled),
		1 => Some(DvspNodeState::Enabled),
		2 => Some(DvspNodeState::Unresponsive),
		3 => Some(DvspNodeState::Unspecified),
		_ => None
	}
}

pub fn u8_valid_nodetype(field: u8) -> bool {
	if field > Bounds::MaxNodeType as u8 {
		return false
	}
	
	return true
}

fn bytes_slice_to_ipv4(bytes: &[u8]) -> Option<Ipv4> {
	if bytes.len() < 4 {
		return None;
	}
	
	let mut addr : Ipv4 = [0;4];
	
	for b in 0 .. 4 {
		addr[b] = bytes[b]
	}
	
	Some(addr)
}

// ----- Data Structures ----- \\
#[derive(Debug)]
pub struct PacketHeader {
	pub msg_type: DvspMsgType,
	pub msg_part: bool,
	pub msg_size: u32,
	pub addr_orig: [u8;4],
	pub addr_dest: [u8;4]
	
}

#[derive(Debug)]
pub struct Packet {
	header: PacketHeader,
	content: Vec<u8>,
}

#[derive(Debug)]
pub struct FrameResponse {	// Response
	pub code: DvspRcode,
}

#[derive(Debug)]
pub struct FrameNodeStatus {	// Response
	pub code: DvspRcode,
	pub status: DvspNodeState
}

#[derive(Debug)]
pub struct FrameNetwork { // Response
	pub list: Vec<u8>,
}

#[derive(Debug)]
pub struct FrameNodeInfo { // Response
	pub code: DvspRcode,
	pub ntype: NodeTypeField,
	pub service: DvspService,
	pub address: Ipv4,
	pub name: String,
}

#[derive(Debug)]
pub struct FrameRegister { 	// Request
	pub register: bool,
	pub ntype: u8,
	pub len: u8,
	pub service: DvspService,
	pub nodereg: String,
}

#[derive(Debug)]
pub struct FrameStateUpdate { 	// Request
	pub status: DvspNodeState,
	pub springname: String,
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
	
	pub fn write_content(&mut self, bytes: &[u8]) -> Result<Success, Failure> {
		
		if bytes.len() > Bounds::PacketContentSize as usize {
			return Err(Failure::OutOfBounds);
		} else {
			push_bytes(&mut self.content, bytes);
			self.header.msg_size = bytes.len() as u32;	
			Ok(Success::Ok)
		}
	}
	
	pub fn content_raw(&self) -> &Vec<u8> {
		&self.content
	}
	
	pub fn content_as<T: NetSerial>(&self) -> Result<T,Failure> {
		
		if self.content.len() < T::lower_bound() {
			return Err(Failure::OutOfBounds);
		}
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
		let bytes : &[u8] = self.content.as_ref();
		v.extend(bytes);
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
		
		p.content = Vec::from(&bytes[Packet::lower_bound()..]);
		Ok(p)
	}
	
	fn lower_bound() -> usize {
		14
	}
}



// ----- FrameResponse ------

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

	fn deserialise(bytes: &[u8]) -> Result<FrameResponse,Failure> {

		let rcode = match u32_rcode_type( u32_transmute_le_arr(&bytes[0..4]) ) {
			None => return Err(Failure::InvalidBytes),
			Some(op) => op
		};
		
		Ok(FrameResponse::new(rcode))
	}
	
	fn lower_bound() -> usize {
		4
	}
}



// ----- FrameNodeStatus ------

impl FrameNodeStatus {
	pub fn new(status: DvspNodeState) -> FrameNodeStatus {
		FrameNodeStatus {
			code: DvspRcode::Ok,
			status: status
		}
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

		let rc = match u32_rcode_type(u32_transmute_le_arr(&bytes[0..4])) {
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
	
	fn lower_bound() -> usize {
		5
	}
}




// ----- FrameNetwork ------

impl FrameNetwork {
	pub fn new(list: &str) -> FrameNetwork {
		let mut v: Vec<u8> = Vec::new();
		v.extend_from_slice(list.as_bytes());

		FrameNetwork {
			list: v 
		}
	}
}

impl NetSerial for FrameNetwork {
	
	fn serialise(&self) -> Vec<u8> {
		self.list.clone()
	}

	fn deserialise(bytes: &[u8]) -> Result<FrameNetwork,Failure> {
		let mut v: Vec<u8> = Vec::new();
		v.extend_from_slice(bytes);
		Ok(FrameNetwork {
				list: v
		})
	}
	
	fn lower_bound() -> usize {
		0
	}
}




// ----- FrameNodeInfo ------

impl FrameNodeInfo {
	pub fn new(ntype: NodeTypeField, service: DvspService, address: Ipv4, name: &str ) -> FrameNodeInfo {
		
		FrameNodeInfo {
			code: DvspRcode::Ok,
			ntype: ntype,
			service: service,
			address: address,
			name: String::from(name),
		}
	}
}

impl NetSerial for FrameNodeInfo {
	
	fn serialise(&self) -> Vec<u8> {
		let mut v: Vec<u8> = Vec::new();
		
		 v.extend_from_slice( &array_transmute_le_u32(self.code as u32) );
		 v.push(self.ntype as u8);
		 v.push(self.service as u8);
		 v.extend_from_slice(&self.address);
		 v.extend_from_slice(&self.name.as_bytes());
		 v
	}

	fn deserialise(bytes: &[u8]) -> Result<FrameNodeInfo,Failure> {
		
		let code = match u32_rcode_type(u32_transmute_le_arr(&bytes[0..4])) {
			None => return Err(Failure::InvalidBytes),
			Some(op) => op	
		};
		
		if u8_valid_nodetype(bytes[4]) == false {
			return Err(Failure::InvalidBytes);
		}
		
		let service = match u8_service_type(bytes[5]) {
			None => return Err(Failure::InvalidBytes),
			Some(op) => op 
		};

		let addr = match bytes_slice_to_ipv4(&bytes[6..10]) {
			None => return return Err(Failure::OutOfBounds),
			Some(op) => op
 		};
	
		let mut name = String::new();
		if bytes.len() > 9 {
			name = String::from( str::from_utf8(&bytes[10..]).unwrap() ) // DANGERZONE: unwrap()
		}	
		
		Ok(FrameNodeInfo {
			code: code,
			ntype: bytes[4],
			service: service,
			address: addr,
			name: name,
		})
	}
	
	fn lower_bound() -> usize {
		10
	}
}




// ----- FrameRegister ------

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
		
		if u8_valid_nodetype(bytes[1]) == false {
			return Err(Failure::InvalidBytes)
		} 
		
		if bytes[2] > Bounds::FrameRegisterLen as u8 {
			return Err(Failure::OutOfBounds)
		}

		
		let mut reg = String::new();
		if bytes.len() > 4 {
			reg = String::from( str::from_utf8(&bytes[4..]).unwrap() ) // DANGERZONE: unwrap()
		}
		
		Ok(FrameRegister {
			register: deserialise_bool(bytes[0]),
			ntype: bytes[1],
			len: bytes[2],
			service: service,
			nodereg: reg 
		})
	}
	
	fn lower_bound() -> usize {
		4
	}
}




// ------- FrameStateUpdate ------- \\

impl FrameStateUpdate {

	pub fn new(state: DvspNodeState, name: &str) -> FrameStateUpdate {
		FrameStateUpdate { 
			status: state,
			springname: String::from(name) 
		}
	}

}

impl NetSerial for FrameStateUpdate {
	
	fn serialise(&self) -> Vec<u8> {
		
		let mut v: Vec<u8> = Vec::new();
		v.push(self.status as u8);	
		v.extend_from_slice(self.springname.as_bytes());
		v
	}

	fn deserialise(bytes: &[u8]) -> Result<FrameStateUpdate,Failure> {

		let status = match u8_status_type(bytes[0]) {
			None => return Err(Failure::InvalidBytes),
			Some(op) => op
		};
		
		let mut name = String::new();
		if bytes.len() > 1 {
			name = String::from( str::from_utf8(&bytes[1..]).unwrap() )
		}
		
		Ok(FrameStateUpdate::new(status, &name))
	}
	
	fn lower_bound() -> usize {
		1
	}
}
