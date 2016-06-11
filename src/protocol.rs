/* Notice:	Copyright 2016, The Care Connections Initiative c.i.c.
 * Author: 	Charlie Fyvie-Gauld (cfg@zunautica.org)
 * License: GPLv3 (http://www.gnu.org/licenses/gpl-3.0.txt)
 */

use std::str;
pub use std::net::{Ipv4Addr, Ipv6Addr};
pub use ::enums::{ParseFailure, NodeRole};

pub use ::formats::{NodeSingleFmt,NodeDoubleFmt,NodeTripleFmt};

pub type Ipv4 = [u8;4];
pub type Ipv6 = [u8;6];

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CmdType {
	Register, Unregister,
	State,
	Info,
	Resolve,
}

/// Variant defining the content of the message
pub enum MessageContent {
	/// There is no body of content
	Empty,
	
	/// Request for Registration
	Registration(ContentRegistration),
	
	/// Contains a Node Single
	NodeSingle(ContentNodeSingle),
}

/// Empty content type
pub struct Empty;

/// Trait for anything that is processed as part of the protocol
pub trait ProtocolObject : Sized {
	fn from_bytes(bytes: &[u8]) -> Result<Self, ParseFailure>;
	fn to_bytes(&self) -> Vec<u8>;
	
}

/// Representing a single message within the protocol
pub struct Message {
	/// The command held in the message
	pub cmd: CmdType,
	
	/// Empty or a content data structure
	pub content: MessageContent,
}

impl Message {
	
	fn next(bytes: &[u8]) -> Result<(usize, &str), ParseFailure> {
		
		for i in 0 .. bytes.len() { 
			match bytes[i] {
				b' ' =>  return match str::from_utf8(&bytes[0..i]) {
							Err(_) => Err(ParseFailure::ConversionError),
							Ok(s) => Ok((i+1, s))
						},
				_ => { }
			}
		}

		match str::from_utf8(&bytes) {
			Err(_) => Err(ParseFailure::ConversionError),
			Ok(s) => Ok((bytes.len(), s))
		}	
	}
	
	fn parse_cmd(cmd: &str) -> Result<CmdType, ParseFailure> {
		match cmd {
			"reg" => Ok(CmdType::Register),
			"ureg" => Ok(CmdType::Unregister),
			"info" => Ok(CmdType::Info),
			_  => Err(ParseFailure::InvalidCommand)
		}
	}
	
	fn parse_content(bytes: &[u8], mtype: CmdType) -> Result<MessageContent, ParseFailure> {
		
		match mtype {
			CmdType::Register => Ok(MessageContent::Registration(try!(ContentRegistration::from_bytes(&bytes)))),
			CmdType::Unregister => Ok(MessageContent::NodeSingle(try!(ContentNodeSingle::from_bytes(&bytes)))),
			_ => return Err(ParseFailure::InvalidCommand),
		}
		
	}
}

impl ProtocolObject for Message {
	fn from_bytes(bytes: &[u8]) -> Result<Self, ParseFailure> {

		let (index, cmd) = try!(Message::next(bytes));
		let mtype = try!(Message::parse_cmd(cmd));
		let content = try!(Message::parse_content(&bytes[index..], mtype));
		Ok(Message{
				cmd: mtype,
				content: content
			})
	}

	fn to_bytes(&self) -> Vec<u8> {
		Vec::new()
	}
	
}


pub struct ContentRegistration {
	pub ndouble: NodeDoubleFmt,
	pub role: NodeRole
}

impl ContentRegistration {
	pub fn to_string(&self) -> String {
		format!("{}", self.ndouble)
	}
}

impl ProtocolObject for ContentRegistration {
	fn from_bytes(bytes: &[u8]) -> Result<Self, ParseFailure> {
		
		if bytes.len() == 0 { return Err(ParseFailure::InvalidContentFormat) }
		let s = match str::from_utf8(bytes) {
			Ok(s) => s,
			Err(_) => return Err(ParseFailure::ConversionError)
		};
		
		let parts: Vec<&str> = s.split(";").collect();
		
		if parts.len() < 3 || parts[0].len() == 0 || parts[1].len() == 0 || parts[2].len() == 0 { 
			return Err(ParseFailure::InvalidContentFormat) 
		}
		
		let role = match NodeRole::from_str(parts[1]) {
			Some(r) => r,
			None => return Err(ParseFailure::InvalidRole)
		};
		
		Ok(
			ContentRegistration {
				ndouble: try!(NodeDoubleFmt::from_str(parts[0])),
				role: role,
			}
		)
	}

	fn to_bytes(&self) -> Vec<u8> {
		Vec::from(self.to_string().as_bytes())
	}	
}

pub struct ContentNodeTriple {
	pub ntriple: NodeTripleFmt
}

impl ContentNodeTriple {
	pub fn to_string(&self) -> String {
		self.ntriple.to_string()
	}
}

impl ProtocolObject for ContentNodeTriple {
	fn from_bytes(bytes: &[u8]) -> Result<Self, ParseFailure> {
		
		if bytes.len() == 0 { return Err(ParseFailure::InvalidContentFormat) }
		
		let s = match str::from_utf8(bytes) {
			Ok(s) => s,
			Err(_) => return Err(ParseFailure::ConversionError)
		};	
		
		Ok( ContentNodeTriple { 
			ntriple: try!(NodeTripleFmt::from_str(s))	 
			} )
	}

	fn to_bytes(&self) -> Vec<u8> {
		Vec::from(self.to_string().as_bytes())
	}	
}


pub struct ContentNodeSingle {
	pub nsingle: NodeSingleFmt
}

impl ContentNodeSingle {
	pub fn to_string(&self) -> String {
		self.nsingle.to_string()
	}
}

impl ProtocolObject for ContentNodeSingle {
	fn from_bytes(bytes: &[u8]) -> Result<Self, ParseFailure> {
		
		if bytes.len() == 0 { return Err(ParseFailure::InvalidContentFormat) }
		
		let s = match str::from_utf8(bytes) {
			Ok(s) => s,
			Err(_) => return Err(ParseFailure::ConversionError)
		};	
		
		Ok( ContentNodeSingle { 
			nsingle: try!(NodeSingleFmt::from_str(s))	 
			} )
	}

	fn to_bytes(&self) -> Vec<u8> {
		Vec::from(self.to_string().as_bytes())
	}	
}