/* Notice:  Copyright 2016, The Care Connections Initiative c.i.c.
 * Author:  Charlie Fyvie-Gauld (cfg@zunautica.org)
 * License: GPLv3 (http://www.gnu.org/licenses/gpl-3.0.txt)
 */
use std::fmt;
use std::str::FromStr;
use regex::Regex;
pub use std::net::{IpAddr};

pub use ::enums::{ParseFailure,NodeService,NodeState,NodeRole};


macro_rules! rng {
	($chk:expr, $from:expr, $to:expr) => (
		//($from..$to).contains($chk)
		if $chk >= $from && $chk <= $to { true } else { false } 
	)
}

#[macro_export]
macro_rules! opt_parsefail {
	($opt:expr) => (
		match $opt {
			Some(s) => s,
			None => return Err(ParseFailure::InvalidContentFormat),
		}
	);
	($opt:expr,$fail:expr) => (
		match $opt {
			Some(s) => s,
			None => return Err($fail),
		}
		 
	);
}

#[macro_export]
macro_rules! res_parsefail {
	($opt:expr) => (
		match $opt {
			Ok(s) => s,
			Err(_) => return Err(ParseFailure::InvalidContentFormat),
		}
	);
	($opt:expr,$fail:expr) => (
		match $opt {
			Ok(s) => s,
			Err(_) => return Err($fail),
		}
		 
	);
}


fn valid_springname(s: &str) -> bool {
	if rng!(s.len(),1,63) == false {
		false
	} else {
		let rex = Regex::new(r"^[a-z0-9-s]+$").unwrap();
		rex.is_match(s)
	}
}

#[test]
fn ts_valid_springname_pass() {
	assert!(valid_springname("foo-bar"));
	assert!(valid_springname("foobar"));
	assert!(valid_springname("f"));
	assert!(valid_springname("foo123"));
}

#[test]
fn ts_valid_springname_fail() {
	assert_eq!(valid_springname("foo.bar"),false);
	assert_eq!(valid_springname("foobarfoobarfoobarfoobarfoobarfoobarfoobarfoobarfoobarfoobarfoobarfoobar"),false);
	assert_eq!(valid_springname(""),false);
	assert_eq!(valid_springname("foo.123"),false);
	assert_eq!(valid_springname("foo_123"),false);
	assert_eq!(valid_springname("foo*123"),false);
}

fn valid_hostname(s: &str) -> bool {
	if rng!(s.len(),1,63) == false {
		false
	} else {
		let rex = Regex::new(r"^[a-z0-9-s./]+$").unwrap();
		rex.is_match(s)
	}
}

#[test]
fn ts_valid_hostname_pass() {
	assert!(valid_hostname("foo.bar"));
	assert!(valid_hostname("foo-bar"));
	assert!(valid_hostname("f"));
	assert!(valid_hostname("foo123"));
}

#[test]
fn ts_valid_hostname_fail() {
	assert_eq!(valid_hostname("foo(bar"),false);
	assert_eq!(valid_hostname("foobarfoobarfoobarfoobarfoobarfoobarfoobarfoobarfoobarfoobarfoobarfoobar"),false);
	assert_eq!(valid_hostname(""),false);
	assert_eq!(valid_hostname("foo[123"),false);
	assert_eq!(valid_hostname("foo_123"),false);
	assert_eq!(valid_hostname("foo*123"),false);
}

fn valid_ip(s: &str) -> bool {
	match IpAddr::from_str(s) {
		Ok(_) => true,
		_ => false,
	}
}

#[test]
fn ts_valid_ip_pass() {
	assert!(valid_ip("192.168.1.1"));
	assert!(valid_ip("1.1.1.1"));
	assert!(valid_ip("1.255.0.0"));
}

#[test]
fn ts_valid_ip_fail() {
	assert_eq!(valid_ip("192.168.1.1.3"), false);
	assert_eq!(valid_ip("1.1"), false);
	assert_eq!(valid_ip("1"), false);
}

/// NodeSingle consists of the string Springname
/// 
/// Text Format: spring
#[derive(Clone,Debug, PartialEq)]
pub struct NodeSingleFmt {
	pub spring: String,
}


impl NodeSingleFmt {
	pub fn from_str(sns: &str) -> Result<Self, ParseFailure> {
		let s = sns.to_lowercase();
		
		if valid_springname(&s) == false {
			return Err(ParseFailure::InvalidNaming)
		}

		Ok( NodeSingleFmt { 
				spring: String::from(s), 
			}
		)
	}
	
	pub fn to_string(&self) -> String {
		format!("{}", self)
	}
}

impl fmt::Display for NodeSingleFmt {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.spring)
	}
}	

/// NodeDouble consists of the strings Springname and Hostname
/// 
/// Text Format: spring,host
#[derive(Clone,Debug, PartialEq)]
pub struct NodeDoubleFmt {
	pub spring: String,
	pub host: String,
}

impl NodeDoubleFmt {

	pub fn from_str(snd: &str) -> Result<Self, ParseFailure> {
		let s = snd.to_lowercase();
		let parts : Vec<&str> = s.split(",").collect();
		
		if parts.len() != 2 { 
			return Err(ParseFailure::InvalidContentFormat) 
		}
		
		if valid_springname(parts[0]) == false || valid_hostname(parts[1]) == false {
			return Err(ParseFailure::InvalidNaming)
		}

		Ok( NodeDoubleFmt { 
				spring: String::from(parts[0]),
				host: String::from(parts[1]), 
			}
		)
	}
	
	pub fn to_string(&self) -> String {
		format!("{}", self)
	}
}

impl fmt::Display for NodeDoubleFmt {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{},{}", self.spring, self.host)
	}
}

/// NodeTriple consists of the strings Springname, Hostname and IP Address
/// 
/// Text Format: spring,host,###.###.###.###
#[derive(Clone,Debug, PartialEq)]
pub struct NodeTripleFmt {
	pub spring: String,
	pub host: String,
	pub address: String,
}

impl NodeTripleFmt {

	pub fn from_str(snt: &str) -> Result<Self, ParseFailure> {
		let s = snt.to_lowercase();
		let parts : Vec<&str> = s.split(",").collect();
		
		if parts.len() != 3 { 
			return Err(ParseFailure::InvalidContentFormat) 
		}
		
		if valid_ip(parts[2]) == false {
			return Err(ParseFailure::InvalidAddress)
		}

		if valid_springname(parts[0]) == false || valid_hostname(parts[1]) == false {
			return Err(ParseFailure::InvalidNaming)
		}

		Ok( NodeTripleFmt { 
				spring: String::from(parts[0]),
				host: String::from(parts[1]),
				address: String::from(parts[2]) 
			}
		)
	}
	
	pub fn to_string(&self) -> String {
		format!("{}", self)
	}
}

impl fmt::Display for NodeTripleFmt {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{},{},{}", self.spring, self.host, self.address)
	}
}

/// NodeQuad consists of the strings Springname, Hostname, IP Address and service
/// 
/// Text Format: spring,host,###.###.###.###,service
#[derive(Clone,Debug, PartialEq)]
pub struct NodeQuadFmt {
	pub spring: String,
	pub host: String,
	pub address: String,
	pub service: NodeService,
}

impl NodeQuadFmt {

	pub fn from_str(snt: &str) -> Result<Self, ParseFailure> {
		let s = snt.to_lowercase();
		let parts : Vec<&str> = s.split(",").collect();
		
		if parts.len() != 4 { 
			return Err(ParseFailure::InvalidContentFormat) 
		}

		if valid_springname(parts[0]) == false || valid_hostname(parts[1]) == false {
			return Err(ParseFailure::InvalidNaming)
		}
		
		if valid_ip(parts[2]) == false {
			return Err(ParseFailure::InvalidAddress)
		}
				
		let service = opt_parsefail!(NodeService::from_str(parts[3]), ParseFailure::InvalidService);

		Ok( NodeQuadFmt { 
				spring: String::from(parts[0]),
				host: String::from(parts[1]),
				address: String::from(parts[2]),
				service: service,
				 
			}
		)
	}
	
	pub fn to_string(&self) -> String {
		format!("{}", self)
	}
}

impl fmt::Display for NodeQuadFmt {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{},{},{},{}", self.spring, self.host, self.address, self.service)
	}
}


/// Node Info is similar to Node but is used fo sendin the
/// information.
///
/// It will display any fields that are correctly set and
/// ignore fields that are `unset`
#[derive(Clone,Debug,PartialEq)]
pub struct NodeInfoFmt {
	pub spring: String,
	pub host: String,
	pub address: String,
	
	pub service: NodeService,
	pub state: NodeState,
	pub role: NodeRole,
}

impl NodeInfoFmt {
	pub fn new() -> NodeInfoFmt {
		NodeInfoFmt {
			spring: String::new(),
			host: String::new(),
			address: String::new(),
			
			service: NodeService::Undefined,
			state: NodeState::Unspecified,
			role: NodeRole::Undefined,
		}		
	}
	pub fn from_str(s: &str) -> Result<Self, ParseFailure> {
		
		if s.len() == 0 { return Err(ParseFailure::InvalidContentFormat) }
		let mut ni = NodeInfoFmt {
			spring: String::new(),
			host: String::new(),
			address: String::new(),
			
			service: NodeService::Undefined,
			state: NodeState::Unspecified,
			role: NodeRole::Undefined,
		};
		
		let parts : Vec<&str> = s.split(",").collect();
		
		for p in parts {
			if p.len() == 0 { continue }
			
			let st = String::from(p);
			let (key,value) = st.split_at(
									 match st.find(':') {
									 	Some(i) => i,
									 	None => return Err(ParseFailure::InvalidContentFormat)
									 } 
								);
			match key.trim() {
				"spring"  => ni.spring = String::from( value[1..].trim() ),
				"host"    => ni.host = String::from( value[1..].trim() ),
				"address" => ni.address = String::from( value[1..].trim() ),
				"service" => ni.service = opt_parsefail!(NodeService::from_str(value[1..].trim()), ParseFailure::InvalidService),
				"state"   => ni.state = opt_parsefail!(NodeState::from_str(value[1..].trim()), ParseFailure::InvalidState),
				"role"    => ni.role = opt_parsefail!(NodeRole::from_str(value[1..].trim()), ParseFailure::InvalidRole),
				_ => { return Err(ParseFailure::InvalidProperty) }
			}
			
		}
		
		Ok(ni)
	}
}

impl fmt::Display for NodeInfoFmt {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut v : Vec<String> = Vec::new();
		
		if self.spring  != "" { v.push(format!("spring:{}", self.spring))   }
		if self.host    != "" { v.push(format!("host:{}", self.host))       }
		if self.address != "" { v.push(format!("address:{}", self.address)) }
		
		if self.service != NodeService::Undefined   { v.push(format!("service:{}", self.service)) }
		if self.state   != NodeState::Unspecified   { v.push(format!("state:{}", self.state))     }
		if self.role    != NodeRole::Undefined      { v.push(format!("role:{}", self.role))       }
		
		write!(f, "{}", v.join(","))
	}
}