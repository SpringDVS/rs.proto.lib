/* Notice:	Copyright 2016, The Care Connections Initiative c.i.c.
 * Author: 	Charlie Fyvie-Gauld (cfg@zunautica.org)
 * License: GPLv3 (http://www.gnu.org/licenses/gpl-3.0.txt)
 */
//! Module `protocol` 
//!
//! Covers the construction of protocol messages into
//! internal data structure representation and converting 
//! the internal representation into protocol messages
//!
//! The construction of the data structures is a cascade through
//! the variants, resulting in a Message containing the correct
//! nesting of variants.
//!
//! The construction of the protocol message is a format! 
//! cascade through the valid Message object and the nested 
//! content, resulting a syntactally correct string
//! 
//! When a message is constructed -- it must be in a completely
//! valid state; aggresive failure when parsing is necessary.

use std::str;
use std::fmt;
pub use std::net::{Ipv4Addr, Ipv6Addr};

pub use ::enums::{ParseFailure,NodeRole,Response,NodeService,NodeState};


pub use ::formats::{NodeSingleFmt,NodeDoubleFmt,NodeTripleFmt,NodeQuadFmt,NodeInfoFmt};

pub type Ipv4 = [u8;4];
pub type Ipv6 = [u8;6];


macro_rules! utf8_from {
	($bytes:expr) => (
		res_parsefail!(str::from_utf8($bytes), ParseFailure::InvalidContentFormat)
	);
}

#[macro_export]
macro_rules! msg_content {
	($content:ident, $ctype:pat) => (
		match $content {
			$ctype(s) => s,
			_ => return Err(ParseFailure::UnexpectedContent)
		}
	)
}


macro_rules! cascade_none {
	($opt: expr) => (
		match $opt {
			Some(s) => Some(s),
			_ => return None,
		}
	)
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CmdType {
	Register, Unregister,
	State,
	Info, Update,
	Resolve,
	Response,
}

impl CmdType  {
	fn from_str(s: &str) -> Option<CmdType> {
		match s.parse::<usize>() {
			Ok(_) => return Some(CmdType::Response),
			_ => {}
		}
		match s {
			"register" => Some(CmdType::Register),
			"unregister" => Some(CmdType::Unregister),
			"info" => Some(CmdType::Info),
			"update" => Some(CmdType::Update),
			_  => None
		}		
	}
}

impl fmt::Display for CmdType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&CmdType::Register => write!(f, "register"),
			&CmdType::Unregister => write!(f, "unregister"),
			&CmdType::Info => write!(f, "info"),
			_ => write!(f, ""),
		}
	}
}

#[derive(Clone, Debug, PartialEq)]
pub enum NodeProperty {
	All,
	Hostname,
	Address,
	State(Option<NodeState>),
	Service(Option<NodeService>),
	Role(Option<NodeRole>),
}

impl NodeProperty  {
	fn from_str(s: &str) -> Option<NodeProperty> {
		match s {
			"hostname" => Some(NodeProperty::Hostname),
			"address" => Some(NodeProperty::Address),
			"state" => Some(NodeProperty::State(None)),
			"service" => Some(NodeProperty::Service(None)),
			"role" => Some(NodeProperty::Role(None)),
			"all" => Some(NodeProperty::All),
			"" => Some(NodeProperty::All),
			_  => None
		}		
	}
	
	fn from_str_option(s: &str, o: &str) -> Option<NodeProperty> {
		
		// cascade_none is used so if there is an invalid property set
		// the None is sent back through the callstack and invalidates
		// the message 
		match s {
			"hostname" => Some(NodeProperty::Hostname),
			"address" => Some(NodeProperty::Address),
			"state" => Some(NodeProperty::State(cascade_none!(NodeState::from_str(o)))),
			"service" => Some(NodeProperty::Service(cascade_none!(NodeService::from_str(o)))),
			"role" => Some(NodeProperty::Role(cascade_none!(NodeRole::from_str(o)))),
			"all" => Some(NodeProperty::All),
			"" => Some(NodeProperty::All),
			_  => None
		}		
	}
}

impl fmt::Display for NodeProperty {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&NodeProperty::All => write!(f, "all"),
			
			&NodeProperty::Hostname => write!(f, "hostname"),
			&NodeProperty::Address => write!(f, "address"),
			
			&NodeProperty::State(None) => write!(f, "state"),
			&NodeProperty::State(Some(ref s)) => write!(f, "state {}",s),
			
			&NodeProperty::Service(None) => write!(f, "service"),
			&NodeProperty::Service(Some(ref s)) => write!(f, "service {}",s),
			
			&NodeProperty::Role(None) => write!(f, "role"),
			&NodeProperty::Role(Some(ref s)) => write!(f, "role {}",s),
			
		}
	}
}

/// Variant defining first level content of the message
#[derive(Clone, Debug, PartialEq)]
pub enum MessageContent {
	/// There is no body of content
	Empty,
	
	/// Request for Registration
	Registration(ContentRegistration),
	
	/// Request for information
	Info(ContentInfoRequest),
	
	/// Request an Update
	Update(ContentNodeProperty),
	
	/// Contains a NodeSingle
	NodeSingle(ContentNodeSingle),

	/// Contains a response
	Response(ContentResponse),
}

impl fmt::Display for MessageContent {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&MessageContent::Empty => write!(f, ""),
			&MessageContent::Info(ref s) => write!(f, "{}",s),
			&MessageContent::Response(ref s) => write!(f, "{}",s),
			&MessageContent::NodeSingle(ref s) => write!(f, "{}",s),
			&MessageContent::Update(ref s) => write!(f, "update {}",s),
			&MessageContent::Registration(ref s) => write!(f, "{}",s)
		}
	}
}

/// Variant defining second level info content
#[derive(Clone, Debug, PartialEq)]
pub enum InfoContent {
	Node(ContentNodeProperty),
	Network
}

impl fmt::Display for InfoContent {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&InfoContent::Network => write!(f, "network"),
			&InfoContent::Node(ref s) => write!(f,"node {}",s)
		}
	}
}

/// Variant defining second level response content
#[derive(Clone, Debug, PartialEq)]
pub enum ResponseContent {
	/// There is no body of content
	Empty,
	
	/// Contains a Node Single
	NodeSingle(ContentNodeSingle),
	
	/// Contains a Network
	Network(ContentNetwork),
	
	/// Containes node info
	NodeInfo(ContentNodeInfo),
}

impl fmt::Display for ResponseContent {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&ResponseContent::Empty => write!(f, ""),
			&ResponseContent::NodeSingle(ref s) => write!(f, "{}", s),
			&ResponseContent::Network(ref s) => write!(f, "{}", s),
			&ResponseContent::NodeInfo(ref s) => write!(f, "{}", s),
			
		}
	}
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
	
	pub fn next(bytes: &[u8]) -> Result<(usize, &str), ParseFailure> {
		
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
	
	fn parse_content(bytes: &[u8], mtype: CmdType) -> Result<MessageContent, ParseFailure> {
		
		match mtype {
			CmdType::Register => Ok(MessageContent::Registration(try!(ContentRegistration::from_bytes(&bytes)))),
			CmdType::Unregister => Ok(MessageContent::NodeSingle(try!(ContentNodeSingle::from_bytes(&bytes)))),
			CmdType::Response => Ok(MessageContent::Response(try!(ContentResponse::from_bytes(&bytes)))),
			CmdType::Info => Ok(MessageContent::Info(try!(ContentInfoRequest::from_bytes(&bytes)))),
			CmdType::Update => Ok(MessageContent::Update(try!(ContentNodeProperty::from_bytes(&bytes)))),
			_ => return Err(ParseFailure::InvalidCommand),
		}
		
	}
}

impl ProtocolObject for Message {
	fn from_bytes(bytes: &[u8]) -> Result<Self, ParseFailure> {

		let (index, cmd) = try!(Message::next(bytes));
		let mtype = match CmdType::from_str(cmd) {
			Some(c) => c, None => return Err(ParseFailure::InvalidCommand) 
		};
		
		let content = match mtype {
			CmdType::Response => try!(Message::parse_content(&bytes, mtype)),
			_ => try!(Message::parse_content(&bytes[index..], mtype)),
		};

		Ok(Message{
				cmd: mtype,
				content: content
			})
	}

	fn to_bytes(&self) -> Vec<u8> {
		let mut v : Vec<String> = Vec::new();
		
		let s : String = format!("{}", self.cmd);
		if s.is_empty() == false {
			v.push(s)
		}
		
		v.push(format!("{}", self.content));
		let full : String = v.join(" ");	
		Vec::from(full.as_str())
	}
	
}

#[derive(Clone,Debug, PartialEq)]
pub struct ContentRegistration {
	pub ndouble: NodeDoubleFmt,
	pub role: NodeRole,
	pub service: NodeService,
}

impl ContentRegistration {
	pub fn to_string(&self) -> String {
		format!("{}", self)
	}
}

impl ProtocolObject for ContentRegistration {
	fn from_bytes(bytes: &[u8]) -> Result<Self, ParseFailure> {
		
		if bytes.len() == 0 { return Err(ParseFailure::InvalidContentFormat) }
		
		let s = utf8_from!(bytes);
		
		let parts: Vec<&str> = s.split(";").collect();
		
		if parts.len() < 3 || parts[0].len() == 0 || parts[1].len() == 0 || parts[2].len() == 0 { 
			return Err(ParseFailure::InvalidContentFormat) 
		}
		
		let role = opt_parsefail!(NodeRole::from_str(parts[1]),ParseFailure::InvalidRole);
		let service = opt_parsefail!(NodeService::from_str(parts[2]), ParseFailure::InvalidService);
		
		Ok(
			ContentRegistration {
				ndouble: try!(NodeDoubleFmt::from_str(parts[0])),
				role: role,
				service: service,
			}
		)
	}

	fn to_bytes(&self) -> Vec<u8> {
		Vec::from(self.to_string().as_bytes())
	}	
}

impl fmt::Display for ContentRegistration {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,"{};{};{}",self.ndouble,self.role,self.service)
	}
}

#[derive(Clone,Debug, PartialEq)]
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
		
		let s = utf8_from!(bytes);
		
		Ok( ContentNodeTriple { 
			ntriple: try!(NodeTripleFmt::from_str(s))	 
			} )
	}

	fn to_bytes(&self) -> Vec<u8> {
		Vec::from(self.to_string().as_bytes())
	}	
}

#[derive(Clone,Debug, PartialEq)]
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


impl fmt::Display for ContentNodeSingle {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,"{}",self.nsingle)
	}
}

#[derive(Clone,Debug, PartialEq)]
pub struct ContentNetwork {
	pub network: Vec<NodeQuadFmt>
}

impl ContentNetwork {
	pub fn to_string(&self) -> String {
		format!("{}", self)
	}
}

impl ProtocolObject for ContentNetwork {
	fn from_bytes(bytes: &[u8]) -> Result<Self, ParseFailure> {
		
		if bytes.len() == 0 { return Err(ParseFailure::InvalidContentFormat) }
		
		let s = utf8_from!(bytes);
		
		let parts : Vec<&str> = s.split(";").collect();
		
		let mut v: Vec<NodeQuadFmt> = Vec::new();
		for sq in parts {
			if sq.len() == 0 { continue }
			v.push(try!(NodeQuadFmt::from_str(sq)))
		}
		
		Ok(ContentNetwork {
			network: v		
		})
	}

	fn to_bytes(&self) -> Vec<u8> {
		Vec::from(self.to_string().as_bytes())
	}
}

impl fmt::Display for ContentNetwork {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut s = String::new();
		
		for n in &self.network {
			s.push_str(&format!("{};", n));
		}
		
		write!(f, "{}", s)
	}
}

#[derive(Clone, Debug, PartialEq)]
pub struct ContentNodeInfo {
	pub info: NodeInfoFmt,
	
}
impl ProtocolObject for ContentNodeInfo {
	fn from_bytes(bytes: &[u8]) -> Result<Self, ParseFailure> {
		
		if bytes.len() == 0 { return Err(ParseFailure::InvalidContentFormat) }
		
		let s = utf8_from!(bytes);
		
		Ok(ContentNodeInfo {
			info: try!(NodeInfoFmt::from_str(s))
		})
	}

	fn to_bytes(&self) -> Vec<u8> {
		Vec::from(self.to_string().as_bytes())
	}
}

impl fmt::Display for ContentNodeInfo {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.info)
	}
}

#[derive(Clone, Debug, PartialEq)]
pub struct ContentResponse {
	pub code: Response,
	pub content: ResponseContent,
}

impl ContentResponse {
	pub fn to_string(&self) -> String {
		format!("{}", self)
	}	
}

impl ProtocolObject for ContentResponse {
	fn from_bytes(bytes: &[u8]) -> Result<Self, ParseFailure> {
		
		if bytes.len() == 0 { return Err(ParseFailure::InvalidContentFormat) }
		
		let s = utf8_from!(bytes);
		

		let code = opt_parsefail!(Response::from_str(&s[0..3]));
		
		let mut content = ResponseContent::Empty;
		if s.len() > 3 {
			let st = String::from(&s[4..]);
			let index = opt_parsefail!(st.find(" "));
			
			let (t,r) = st.split_at(index);

			content = match t {
				"network" => ResponseContent::Network(try!(ContentNetwork::from_bytes(&r[1..].as_bytes()))),
				"node" => ResponseContent::NodeInfo(try!(ContentNodeInfo::from_bytes(&r[1..].as_bytes()))),
				_ => return Err(ParseFailure::InvalidContentFormat),
			}
		}
		
		
		Ok(ContentResponse {
			code: code,
			content: content
		})
	}

	fn to_bytes(&self) -> Vec<u8> {
		Vec::from(self.to_string().as_bytes())
	}
}

impl fmt::Display for ContentResponse {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let d = self.content.clone();
		match d {
			ResponseContent::Network(s) => write!(f, "{} network {}", self.code, s),
			ResponseContent::NodeInfo(s) => write!(f, "{} node {}", self.code, s),
			_ =>  write!(f, "{}", self.code),
			 
		}
		
	}
}

#[derive(Clone, Debug, PartialEq)]
pub struct ContentInfoRequest {
	pub info: InfoContent,
}

impl ContentInfoRequest {
	pub fn to_string(&self) -> String {
		format!("{}", self)
	}	
}

impl ProtocolObject for ContentInfoRequest {
	fn from_bytes(bytes: &[u8]) -> Result<Self, ParseFailure> {
		
		if bytes.len() == 0 { return Err(ParseFailure::InvalidContentFormat) }
		
		let s = utf8_from!(bytes);
		if s.len() >= 4 {
			
			let st = String::from(s);

			let (t,r) =  match st.find(" ") {
				Some(i) => st.split_at(i),
				None => (st.as_str(), "")
			};
		
			let nx = if r.len() > 0 { &r[1..] }  else { "" };
			
			let info = match t {
				"network" => InfoContent::Network,
				"node" => InfoContent::Node(try!(ContentNodeProperty::from_bytes(nx.as_bytes()))),
				_ => return Err(ParseFailure::InvalidContentFormat)
			};
			Ok(
				ContentInfoRequest{
					info: info,
				}
			)
			
		} else {
			Err(ParseFailure::InvalidContentFormat)
		}
	}

	fn to_bytes(&self) -> Vec<u8> {
		Vec::from(self.to_string().as_bytes())
	}
}

impl fmt::Display for ContentInfoRequest {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.info)
		
	}
}

#[derive(Clone, Debug, PartialEq)]
pub struct ContentNodeProperty {
	pub spring: String,
	pub property: NodeProperty,
}

impl ContentNodeProperty {
	pub fn to_string(&self) -> String {
		format!("{}", self)
	}	
}

impl ProtocolObject for ContentNodeProperty {
	fn from_bytes(bytes: &[u8]) -> Result<Self, ParseFailure> {
		
		if bytes.len() == 0 { return Err(ParseFailure::InvalidContentFormat) }
		
		let s : &str = utf8_from!(bytes);

		let (spring, property, value) : (&str,&str,&str) = {
			let parts : Vec<&str> = s.split(" ").collect();
			match parts.len() {
				1 => (parts[0],"",""),
				2 => (parts[0],parts[1],""),
				3 => (parts[0],parts[1],parts[2]),
				_ => return Err(ParseFailure::InvalidContentFormat)
			}
		};
		
		if value.len() == 0 {
			Ok(ContentNodeProperty {
				spring : String::from(spring),	
				property: opt_parsefail!(NodeProperty::from_str(property))
			})
		} else {
			Ok(ContentNodeProperty {
				spring : String::from(spring),	
				property: opt_parsefail!(NodeProperty::from_str_option(property, value))
			})
		}
	}

	fn to_bytes(&self) -> Vec<u8> {
		Vec::from(self.to_string().as_bytes())
	}
}

impl fmt::Display for ContentNodeProperty {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} {}", self.spring, self.property)
		
	}
}
