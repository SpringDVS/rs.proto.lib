/* Notice:	Copyright 2016, The Care Connections Initiative c.i.c.
 * Author: 	Charlie Fyvie-Gauld (cfg@zunautica.org)
 * License: GPLv3 (http://www.gnu.org/licenses/gpl-3.0.txt)
 */

use std::str;

use ::serialise::*;
use ::enums::*;
use std::net::SocketAddr;
use std::str::FromStr;

pub type Ipv4 = [u8;4];
pub type Ipv6 = [u8;6];
pub type NodeTypeField = u8;

// ----- Helper Functions ----- \\

pub fn u8_packet_type(byte: u8) -> Option<DvspMsgType> {
	match byte {
		0 => Some(DvspMsgType::Undefined),
		1 => Some(DvspMsgType::GsnRegistration),
		2 => Some(DvspMsgType::GsnResolution),
		3 => Some(DvspMsgType::GsnArea),
		4 => Some(DvspMsgType::GsnState),
		5 => Some(DvspMsgType::GsnNodeInfo),
		6 => Some(DvspMsgType::GsnNodeStatus),
		7 => Some(DvspMsgType::GsnRequest),
		8 => Some(DvspMsgType::GsnTypeRequest),
		
		22 => Some(DvspMsgType::GtnRegistration),
		23 => Some(DvspMsgType::GtnGeosubNodes),
		
		30 => Some(DvspMsgType::GsnResponse),
		31 => Some(DvspMsgType::GsnResponseNodeInfo),
		32 => Some(DvspMsgType::GsnResponseNetwork),
		33 => Some(DvspMsgType::GsnResponseHigh),
		34 => Some(DvspMsgType::GsnResponseStatus),
		
		101 => Some(DvspMsgType::UnitTest),
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

pub fn u8_unit_test_action(byte: u8) -> Option<UnitTestAction> {
	match byte {
		0 => Some(UnitTestAction::Undefined),
		1 => Some(UnitTestAction::Reset),
		2 => Some(UnitTestAction::UpdateAddress),
		3 => Some(UnitTestAction::AddGeosubRoot),
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


pub fn http_to_bin(src: &[u8]) -> Result<Vec<u8>,Failure> {
	
	let mut v = Vec::new();
	let l = src.len();
	
	if l % 2 > 0 { return Err(Failure::InvalidFormat) }

	
	let mut i : usize = 0;
	while i < l {
		
		match hex_str_to_byte(&src[i..i+2]) {
			Some(b) => v.push(b),
			None => return Err(Failure::InvalidBytes),
		};
		
		i += 2;
	}
	
	Ok(v)
}

pub fn http_from_bin(src: &Vec<u8>) -> String {
	bin_to_hex(src)
}

// ----- Data Structures ----- \\
#[derive(Debug, Copy,Clone)]
pub struct PacketHeader {
	pub msg_type: DvspMsgType,
	pub msg_part: bool,
	pub msg_size: u32,
	pub addr_orig: [u8;4],
	pub addr_dest: [u8;4]
	
}

#[derive(Debug,Clone)]
pub struct Packet {
	header: PacketHeader,
	content: Vec<u8>,
	tcp: bool,
}

pub struct HttpWrapper;


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
pub struct FrameRegisterGtn { 	// Request
	pub register: bool,
	pub service: DvspService,
	pub len: u8,
	pub nodereg: String,
}


#[derive(Debug)]
pub struct FrameStateUpdate { 	// Request
	pub status: DvspNodeState,
	pub springname: String,
}

#[derive(Debug)]
pub struct FrameNodeRequest { 	// Request
	pub shi: Vec<u8>
}

#[derive(Debug)]
pub struct FrameTypeRequest { 	// Request
	pub ntype: NodeTypeField
}

#[derive(Debug)]
pub struct FrameResolution { 	// Request
	pub url: String
}

#[derive(Debug)]
pub struct FrameGeosub { 	// Request
	pub gsn: String
}

#[derive(Debug)]
pub struct FrameUnitTest {
	pub action: UnitTestAction,
	pub extra: String,
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
			tcp: false,
		}
	}
	
	pub fn from_serialisable<T: NetSerial>(msgtype: DvspMsgType, content: &T) -> Result<Packet, Failure> {
		let mut p = Packet::new(msgtype);
		match p.write_content(content.serialise().as_slice()) {
			Ok(_) => Ok(p),
			Err(f) => Err(f)
		}
	}
	
	pub fn tcp_flag(&mut self, flag: bool) {
		self.tcp = flag;
	}
	
	pub fn header(&self) -> &PacketHeader {
		&self.header
	}
	
	pub fn mut_header(&mut self) -> &mut PacketHeader {
		&mut self.header
	}
	
	pub fn write_content(&mut self, bytes: &[u8]) -> Result<Success, Failure> {
		
		if self.tcp == false && bytes.len() > Bounds::PacketContentSize as usize {
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
		if bytes.len() < Packet::lower_bound() { return Err(Failure::InvalidArgument) }
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


impl HttpWrapper {

	/// Takes a Packet, encodes it in an hexadecimal string, wraps it
	/// in an HTTP request and serialises it all into a vector of bytes
	///
	/// # Arguments
	///
	/// * `packet` - The Packet to serialise for HTTP service layer
	/// * `host` - The host of the target node
	/// * `resource` - The resource to push to on the target node
	///
	/// # Example
	///
	/// 
	/// // Send the request to `spring.example.tld/node/`
	/// let bytes = HttpWrapper::serialise(&Packet::from_serialisable(
	///											DvspMsgType::Response,
	///											&FrameResponse::new(DvspRcode::Ok)
	///										), "spring.example.tld", "/node/");
	///
	pub fn serialise_request(packet: &Packet, host: &str, resource: &str) -> Vec<u8> {
		let serial = http_from_bin(&packet.serialise()).into_bytes();
		let header : String = format!(
"POST /{} HTTP/1.1\r
Host: {}\r
User-Agent: SpringDVS\r
Content-Type: application/octet-stream\r
Content-Length: {}\r\n\r\n", resource, host, serial.len()
		);
		
		let mut v = Vec::new();
		v.extend_from_slice(header.as_ref());
		v.extend_from_slice(serial.as_ref());
		v
	}

	/// Takes bytes of a serialised packet and encodes it in an HTTP Reqest
	///
	/// # Arguments
	///
	/// * `vytes` - Thebytes of an already serialised Packet
	/// * `host` - The host of the target node
	/// * `resource` - The resource to push to on the target node
	///
	/// # Example
	///
	/// 
	/// // Send the request to `spring.example.tld/node/`
	/// let bytes = HttpWrapper::serialise(&Packet::from_serialisable(
	///											DvspMsgType::Response,
	///											&FrameResponse::new(DvspRcode::Ok)
	///										).serialise(), "spring.example.tld", "/node/");
	///
	pub fn serialise_bytes_request(bytes: &Vec<u8>, host: &str, resource: &str) -> Vec<u8> {
		let serial = http_from_bin(&bytes).into_bytes();
		let header : String = format!(
"POST /{} HTTP/1.1\r
Host: {}\r
User-Agent: SpringDVS\r
Content-Type: application/octet-stream\r
Content-Length: {}\r\n\r\n", resource, host, serial.len()
		);
		
		let mut v = Vec::new();
		v.extend_from_slice(header.as_ref());
		v.extend_from_slice(serial.as_ref());
		v
	}

	/// Takes a Packet, encodes it in an hexadecimal string wraps it in
	/// an HTTP response and returns a vector of bytes
	///
	/// # Arguments
	///
	/// * `packet` - The Packet to serialise for HTTP service layer	
	pub fn serialise_response(packet: &Packet) -> Vec<u8> {
		let serial = http_from_bin(&packet.serialise()).into_bytes();
		let header : String = format!(
"HTTP/1.1 200 OK\r
Server: SpringDVS/0.1\r
Content-Type: application/octet-stream\r
Connection: Closed\r
Content-Length: {}\r\n\r\n", serial.len()
		);
		
		let mut v = Vec::new();
		v.extend_from_slice(header.as_ref());
		v.extend_from_slice(serial.as_ref());
		v
	}
	
	/// Takes the bytes of a packet, encodes it in an hexadecimal string
	/// wrapped in an HTTP response and returns a vector for bytes
	///
	/// # Arguments
	///
	/// * `packet` - The Packet to serialise for HTTP service layer	
	pub fn serialise_response_bytes(bytes: &Vec<u8>) -> Vec<u8> {
		let serial = http_from_bin(&bytes).into_bytes();
		let header : String = format!(
"HTTP/1.1 200 OK\r
Server: SpringDVS/0.1\r
Content-Type: application/octet-stream\r
Connection: Closed\r
Content-Length: {}\r\n\r\n", serial.len()
		);
		
		let mut v = Vec::new();
		v.extend_from_slice(header.as_ref());
		v.extend_from_slice(serial.as_ref());
		v
	}
	
	/// Takes an HTTP service layer request, including HTTP Headers,
	/// and returns the bytes of a packet that is encoded within
	///
	/// # Arguments
	///
	/// * `bytes` - A Vector of u8 bytes consisting of the entire request	
	pub fn deserialise_request(bytes: Vec<u8>, address: &mut SocketAddr) -> Result<Vec<u8>,Failure> {
		
		let s = match String::from_utf8(bytes) {
			Ok(s) => s,
			Err(_) => return Err(Failure::InvalidBytes)
		};
		
		let atoms : Vec<&str> = s.split("\r\n\r\n").collect();
		
		if atoms.len() != 2 { return Err(Failure::InvalidFormat) }
		// rewrite address incase of proxy forwarding
		
		match HttpWrapper::extract_forwarded(atoms[0]) {
			Some(addr) => *address = SocketAddr::from_str(&format!("{}:80", addr)).unwrap(),
			_ => { }
		}
		
		http_to_bin(atoms[1].as_bytes())
		
	}
	
	pub fn deserialise_response(bytes: Vec<u8>) -> Result<Vec<u8>,Failure> {
		let s : String = match String::from_utf8(bytes) {
			Ok(s) => s,
			Err(_) => return Err(Failure::InvalidBytes)
		};
		//let mut content = String::new();

		let content = match s.find("\r\n\r\n") {
			Some(_) => {
				let atoms : Vec<&str> = s.split("\r\n\r\n").collect();
				if atoms.len() != 2 { return Err(Failure::InvalidFormat) }
				String::from(atoms[1])
			} 
			None => s
		};

		http_to_bin( content.as_bytes() )
	}
	
	fn extract_forwarded(block: &str) -> Option<String> {
		let headers : Vec<&str> = block.split("\n").collect();
		for header in headers {
			let atoms : Vec<&str> = header.split(":").collect();
			
			// Todo:
			// Handle all the standard and de-facto headers here
			// - Forwarded:
			// - X-Real-IP:
			if atoms[0] == "X-Forwarded-For" && atoms.len() > 1 {
				return Some(String::from(atoms[1].trim()));
			}
		} 
		
		None
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
			reg =  match  str::from_utf8(&bytes[4..]) {
				Ok(s) => String::from(s),
				_ => return Err(Failure::InvalidBytes),
			}
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
			
			name = match str::from_utf8(&bytes[1..]) {
				Ok(s) => String::from(s),
				_ => return Err(Failure::InvalidBytes)
			};
		}
		
		Ok(FrameStateUpdate::new(status, &name))
	}
	
	fn lower_bound() -> usize {
		1
	}
}





// ----- FrameNodeRequest ------

impl FrameNodeRequest {
	pub fn new(shi: &str) -> FrameNodeRequest {
		let mut v: Vec<u8> = Vec::new();
		v.extend_from_slice(shi.as_bytes());

		FrameNodeRequest {
			shi: v 
		}
	}
}

impl NetSerial for FrameNodeRequest {
	
	fn serialise(&self) -> Vec<u8> {
		self.shi.clone()
	}

	fn deserialise(bytes: &[u8]) -> Result<FrameNodeRequest,Failure> {
		let mut v: Vec<u8> = Vec::new();
		v.extend_from_slice(bytes);
		Ok(FrameNodeRequest {
				shi: v
		})
	}
	
	fn lower_bound() -> usize {
		0
	}
}





// ----- FrameNodeRequest ------

impl FrameTypeRequest {
	pub fn new(ntype: NodeTypeField) -> FrameTypeRequest {
		FrameTypeRequest {
			ntype: ntype
		}
	}
}

impl NetSerial for FrameTypeRequest {
	
	fn serialise(&self) -> Vec<u8> {
		let mut v: Vec<u8> = Vec::new();
		v.push(self.ntype as u8);
		v	
	}

	fn deserialise(bytes: &[u8]) -> Result<FrameTypeRequest,Failure> {

		if u8_valid_nodetype(bytes[0]) == false {
			return Err(Failure::InvalidBytes)
		} 

		Ok(FrameTypeRequest {
				ntype: bytes[0]
		})
	}
	
	fn lower_bound() -> usize {
		1
	}
}



// ----- FrameResolution ------

impl FrameResolution {
	pub fn new(url: &str) -> FrameResolution {
		FrameResolution {
			url: String::from(url) 
		}
	}
}

impl NetSerial for FrameResolution {
	
	fn serialise(&self) -> Vec<u8> {
		let mut v : Vec<u8> = Vec::new();
		
		v.extend_from_slice(self.url.as_bytes());
		v
	}

	fn deserialise(bytes: &[u8]) -> Result<FrameResolution,Failure> {

		let url = match str::from_utf8(bytes) {
			Ok(s) => String::from(s),
			_ => return Err(Failure::InvalidBytes)
		};
		
		Ok(FrameResolution {
				url: url
		})
	}
	
	fn lower_bound() -> usize {
		0
	}
}



// ----- FrameResolution ------

impl FrameGeosub {
	pub fn new(gsn: &str) -> FrameGeosub {
		FrameGeosub {
			gsn: String::from(gsn) 
		}
	}
}

impl NetSerial for FrameGeosub {
	
	fn serialise(&self) -> Vec<u8> {
		let mut v : Vec<u8> = Vec::new();
		
		v.extend_from_slice(self.gsn.as_bytes());
		v
	}

	fn deserialise(bytes: &[u8]) -> Result<FrameGeosub,Failure> {

		let gsn = match str::from_utf8(bytes) {
			Ok(s) => String::from(s),
			_ => return Err(Failure::InvalidBytes)
		};
		
		Ok(FrameGeosub {
				gsn: gsn
		})
	}
	
	fn lower_bound() -> usize {
		0
	}
}



// ----- FrameUnitTest ------

impl FrameUnitTest {
	pub fn new(action: UnitTestAction, extra: &str) -> FrameUnitTest {
		FrameUnitTest {
			action: action,
			extra: String::from(extra),
		}
	}
}

impl NetSerial for FrameUnitTest {
	
	fn serialise(&self) -> Vec<u8> {
		let mut v: Vec<u8> = Vec::new();
		v.push(self.action as u8);
		v.extend_from_slice(self.extra.as_bytes());
		v	
	}

	fn deserialise(bytes: &[u8]) -> Result<FrameUnitTest,Failure> {

		
		let action = match u8_unit_test_action(bytes[0]) {
			None => return Err(Failure::InvalidBytes),
			Some(op) => op
		};
		let mut extra = String::new();
		if bytes.len() > 1 {
			
			extra = match str::from_utf8(&bytes[1..]) {
				Ok(s) => String::from(s),
				_ => return Err(Failure::InvalidBytes)
			};
		}
		Ok(FrameUnitTest {
				action: action,
				extra: extra, 
		})
	}
	
	fn lower_bound() -> usize {
		1
	}
}


// ----- FrameRegisterGtn ------

impl FrameRegisterGtn {
	pub fn new(register: bool, service: DvspService, nodereg: String) -> FrameRegisterGtn {
		FrameRegisterGtn {
			register: register,
			service: service,
			len: nodereg.len() as u8,
			nodereg: nodereg,
		}
	} 
}

impl NetSerial for FrameRegisterGtn {
	fn serialise(&self) -> Vec<u8> {
		
		let mut v: Vec<u8> = Vec::new();
		v.push(self.register as u8);
		v.push(self.service as u8);
		v.push(self.len);
		push_bytes(&mut v, self.nodereg.as_bytes());		
		v
	}

	fn deserialise(bytes: &[u8]) -> Result<FrameRegisterGtn, Failure> {

		let service = match u8_service_type(bytes[1]) {
			None => return Err(Failure::InvalidBytes),
			Some(op) => op
		};
		
		if bytes[2] > Bounds::FrameRegisterLen as u8 {
			return Err(Failure::OutOfBounds)
		}

		
		let mut reg = String::new();
		if bytes.len() > 3 {
			reg =  match  str::from_utf8(&bytes[3..]) {
				Ok(s) => String::from(s),
				_ => return Err(Failure::InvalidBytes),
			}
		}
		
		Ok(FrameRegisterGtn {
			register: deserialise_bool(bytes[0]),
			service: service,
			len: bytes[2],
			nodereg: reg 
		})
	}
	
	fn lower_bound() -> usize {
		3
	}
}



