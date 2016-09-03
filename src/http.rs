/* Notice:  Copyright 2016, The Care Connections Initiative c.i.c.
 * Author:  Charlie Fyvie-Gauld (cfg@zunautica.org)
 * License: GPLv3 (http://www.gnu.org/licenses/gpl-3.0.txt)
 */
use std::str;
use std::str::FromStr;
use std::net::{SocketAddr};
use protocol::{ProtocolObject, Message};
use enums::{Failure};

pub struct HttpWrapper;


impl HttpWrapper {

	/// Takes a Message and wraps it in an HTTP request and 
	/// serialises it all into a vector of bytes
	///
	/// # Arguments
	///
	/// * `packet` - The Packet to serialise for HTTP service layer
	/// * `host` - The host of the target node
	///
	/// # Example
	///
	/// 
	/// // Send the request to `spring.example.tld/node/`
	/// let bytes = HttpWrapper::serialise(&Packet::from_serialisable(
	///											DvspMsgType::Response,
	///											&FrameResponse::new(DvspRcode::Ok)
	///										), "spring.example.tld");
	///
	pub fn serialise_request(msg: &Message, host: &str) -> Vec<u8> {
		let serial = msg.to_bytes();
		let header : String = format!(
"POST /spring/ HTTP/1.1\r
Host: {}\r
User-Agent: SpringDVS\r
Content-Type: text/plain\r
Content-Length: {}\r\n\r\n", host, serial.len()
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
	pub fn serialise_bytes_request(bytes: &Vec<u8>, host: &str) -> Vec<u8> {

		let header : String = format!(
"POST /spring/ HTTP/1.1\r
Host: {}\r
User-Agent: SpringDVS\r
Content-Type: text/plain\r
Content-Length: {}\r\n\r\n", host, bytes.len()
		);
		
		let mut v = Vec::new();
		v.extend_from_slice(header.as_ref());
		v.extend_from_slice(bytes.as_ref());
		v
	}

	/// Takes a Message, turns it into a string wraps it in
	/// an HTTP response and returns a vector of bytes
	///
	/// # Arguments
	///
	/// * `msg` - The Packet to serialise for HTTP service layer	
	pub fn serialise_response(msg: &Message) -> Vec<u8> {
		let serial = msg.to_bytes();
		let header : String = format!(
"HTTP/1.1 200 OK\r
Server: SpringDVS/0.1\r
Content-Type: text/plain\r
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
		let header : String = format!(
"HTTP/1.1 200 OK\r
Server: SpringDVS/0.1\r
Content-Type: text/plain\r
Connection: Closed\r
Content-Length: {}\r\n\r\n", bytes.len()
		);
		
		let mut v = Vec::new();
		v.extend_from_slice(header.as_ref());
		v.extend_from_slice(bytes.as_ref());
		v
	}
	
	/// Takes an HTTP service layer request, including HTTP Headers,
	/// and returns the String of a message that is encoded within
	///
	/// # Arguments
	///
	/// * `bytes` - A Vector of u8 bytes consisting of the entire request	
	pub fn deserialise_request(bytes: Vec<u8>, address: &mut SocketAddr) -> Result<Message,Failure> {
		
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
		
		let m = match Message::from_bytes(atoms[1].trim().as_bytes()) {
			Ok(m) => m,
			Err(_) => return Err(Failure::InvalidConversion)
		};
		Ok(m)
	}
	
	pub fn deserialise_response(bytes: Vec<u8>) -> Result<(Vec<u8>,usize),Failure> {
		let s : String = match String::from_utf8(bytes) {
			Ok(s) => s,
			Err(_) => return Err(Failure::InvalidBytes)
		};

		match s.find("\r\n\r\n") {
			Some(i) => {
				
				let atoms : Vec<&str> = s.split("\r\n\r\n").collect();
				if atoms.len() != 2 { return Err(Failure::InvalidFormat) }
				
				Ok( (Vec::from(atoms[1].trim().as_bytes()), i+4+1) )
			} 
			None => {
				Err(Failure::InvalidConversion)
			}
		}
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