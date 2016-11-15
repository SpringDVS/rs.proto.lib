/* Notice:  Copyright 2016, The Care Connections Initiative c.i.c.
 * Author:  Charlie Fyvie-Gauld (cfg@zunautica.org)
 * License: GPLv3 (http://www.gnu.org/licenses/gpl-3.0.txt)
 */
use std::str;
use std::str::FromStr;
use std::net::{SocketAddr};
use std::i64;

use protocol::{ProtocolObject, Message};
use node::Node;
use enums::{Failure};

use std::io::prelude::*;
use std::net::{TcpStream};


pub struct HttpWrapper;

// ToDo: Make the chunked encoding handler nicer
// ToDo: Handle case where HTTP header is larger than initial buffer 
// ToDo *2:
//   Handle all the standard and de-facto headers here
//   - Forwarded:
//   - X-Real-IP:

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
"POST /spring/ HTTP/1.0\r
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

	/// Takes bytes and wrap in HTTP POST request
	///
	/// # Arguments
	///
	/// * `bytes` - A slice of bytes to be wrapped 
	/// * `host` - The host of the target node
	/// * `post` - The host of the target node
	pub fn wrap_request(bytes: &[u8], host: &str, path: &str) -> Vec<u8> {

		let header : String = format!(
"POST /{} HTTP/1.1\r
Host: {}\r
User-Agent: SpringPrim/0.3\r
Content-Type: text/plain\r
Content-Length: {}\r\n\r\n", path, host, bytes.len()
		);
		
		let mut v = Vec::new();
		v.extend_from_slice(header.as_ref());
		v.extend_from_slice(bytes.as_ref());
		v
	}
	
	pub fn unwrap_response(bytes: &[u8]) -> Option<(Vec<u8>,Vec<u8>)> {
		let s = match str::from_utf8(bytes) {
			Ok(s) => s,
			Err(_) => return None
		};

		match s.find("\r\n\r\n") {
			Some(_) => {
				
				let atoms : Vec<&str> = s.splitn(2, "\r\n\r\n").collect();
				if atoms.len() != 2 { return None }
				//println!("\n*********\n[Debug]Header:\n{}\n%%%%%%%%%%%%%\n", atoms[0]);
				Some( (Vec::from(atoms[0].trim().as_bytes()),Vec::from(atoms[1].trim().as_bytes())) )
			},
			None => None
		}
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
			
			// Todo: *2
			if atoms[0] == "X-Forwarded-For" && atoms.len() > 1 {
				return Some(String::from(atoms[1].trim()));
			}
		} 
		
		None
	}
	
	pub fn extract_header(search: &str, block: &str) -> Option<String> {
		let headers : Vec<&str> = block.split("\n").collect();
		for header in headers {
			let atoms : Vec<&str> = header.split(":").collect();
			
			if atoms[0] == search && atoms.len() > 1 {
				return Some(String::from(atoms[1].trim()));
			}
		} 
		
		None		
	}
	
	pub fn content_len(bytes: &[u8]) -> Option<usize> {
		
		let block = match str::from_utf8(bytes) {
			Ok(s) => s,
			Err(_) => return None,
		};
		
		match HttpWrapper::extract_header("Content-Length", block) {
			Some(v) => match v.parse::<usize>() {
				Ok(u) => Some(u),
				Err(_) => None
			},
			None => None
		}
	}
}

pub struct Outbound;

impl Outbound {
	pub fn request(bytes: &[u8], address: &str, host: &str, path: &str) -> Option<Vec<u8>> {
		
		let addr = format!("{}:{}", address, 80);
		let msg = HttpWrapper::wrap_request(bytes, host, path);
		
		let mut stream = match TcpStream::connect(addr.as_str()) {
			Ok(s) => s,
			Err(_) => return None
		};
		
		

		stream.write(msg.as_slice()).unwrap();

		let mut buf = [0;4096];
		let size = match stream.read(&mut buf) {
					Ok(s) => s,
					Err(_) => 0
		};

		

		if size == 0 { return None }
		let (hdrbuf, msgbuf) = match HttpWrapper::unwrap_response(&buf[0..size]) {
			Some(r) => r,
			None => return None
		};
		
		let hdrstr = String::from_utf8(hdrbuf.clone()).unwrap();
		
		match HttpWrapper::extract_header("Content-Length", &hdrstr) {
			Some(_) => Outbound::transfer_single(&hdrbuf, msgbuf, &mut stream),
			None => {
				match HttpWrapper::extract_header("Transfer-Encoding", &hdrstr) {
					Some(_) => Outbound::transfer_chunked(msgbuf, &mut stream),
					None => None
				}
			}
		}
			

	}
	
	fn transfer_single(hdrbuf: &Vec<u8>, mut msgbuf: Vec<u8>, stream: &mut TcpStream) -> Option<Vec<u8>> {
		
		match HttpWrapper::content_len(hdrbuf.as_slice()) {
			Some(conlen) => {
				let metalen = hdrbuf.len() + 4; // 4 bytes = \r\n\r\n
				if (metalen + conlen) > 4096 {
					let diff = conlen - (4096-metalen);
					let mut vbuf = Vec::new();
					vbuf.resize(diff, 0);
					match stream.read(&mut vbuf.as_mut_slice()) {
						Ok(s) => s,
						Err(_) =>  0
					};
					msgbuf.append(&mut vbuf);
				}
				
				Some(msgbuf)
			}
			_ => { Some(msgbuf) }
		}
		
					
	}
	
	fn transfer_chunked(msgbuf: Vec<u8>, stream: &mut TcpStream) -> Option<Vec<u8>> {
		
		let mut buflen = msgbuf.len();
		let mut index = 0;
		let mut response = str::from_utf8(msgbuf.as_slice()).unwrap().to_string();
		
		
		let mut aggregate = String::new();
		loop {
			
			let r = response.clone();
			let buf : Vec<&str> = r.splitn(2, "\r\n").collect();
			
			if buf[0].trim() == "0" {
					break
			}

			index += buf[0].len() + 2; // include \r\n bytes
			
			
			let chunk_size = i64::from_str_radix(buf[0], 16).unwrap() as usize;
			let buflen_req = index + chunk_size;
			
			let chunkbuf = if buflen_req > buflen + 2 {
				
				// We haven't pulled the whole chunk from the socket
				// so read the chunk and read to next chunk size
				let diff = (buflen_req - buflen)+2;
				
				let mut v : Vec<u8> = Vec::new();
				v.resize(diff, b'\0');
				let _ = stream.read_exact(&mut v.as_mut());
				let mut bufstr = String::from(buf[1]);
				 
				bufstr.push_str(str::from_utf8(v.as_slice()).unwrap());
				
				buflen += diff;
				
				v.clear();
				
				
				let mut b = [b'\0'];
				loop {
					let _ =stream.read_exact(&mut b);
					v.push(b[0]);
					
					if b[0] == b'\r' {
						let _ = stream.read_exact(&mut b);
						v.push(b[0]);
						if b[0] == b'\n' { break; }
					}
				}
				
				bufstr.push_str(str::from_utf8(v.as_slice()).unwrap());
				buflen += v.len();
				bufstr.clone()
				
			} else {
				String::from(buf[1])
			};

			let (value,rest) = chunkbuf.split_at(chunk_size);
			
			response = rest.trim_left().to_string();
			index += chunk_size+2;
			
			aggregate.push_str(value);
		}
		
		 
		Some(Vec::from(aggregate.as_bytes()))
	}
	pub fn request_node(message: &Message, node: &Node) -> Option<Message> {
		let response : Vec<u8> = match Outbound::request(message.to_bytes().as_slice(), node.address(), node.hostname(), "spring") {
			Some(r) => r,
			None => return None
		};
		
		match Message::from_bytes(response.as_slice()) {
			Ok(m) => Some(m),
			Err(_) => None
		}
	}	
}