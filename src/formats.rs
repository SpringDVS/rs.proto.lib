/* Notice:  Copyright 2016, The Care Connections Initiative c.i.c.
 * Author:  Charlie Fyvie-Gauld (cfg@zunautica.org)
 * License: GPLv3 (http://www.gnu.org/licenses/gpl-3.0.txt)
 */


use std::fmt;
use regex::Regex;

use ::protocol::{Ipv4};
pub use ::enums::ParseFailure;


macro_rules! rng {
	($chk:expr, $from:expr, $to:expr) => (
		//($from..$to).contains($chk)
		if $chk >= $from && $chk <= $to { true } else { false } 
	)
}

pub struct NodeDoubleFmt {
	pub spring: String,
	pub host: String,
}


fn valid_name(s: &str) -> bool {
	if rng!(s.len(),1,63) == false {
		false
	} else {
		let rex = Regex::new(r"^[a-z0-9-s]+$").unwrap();
		rex.is_match(s)
	}
}

#[test]
fn ts_valid_name_pass() {
	assert!(valid_name("foo-bar"));
	assert!(valid_name("foobar"));
	assert!(valid_name("f"));
	assert!(valid_name("foo123"));
}

#[test]
fn ts_valid_name_fail() {
	assert_eq!(valid_name("foo.bar"),false);
	assert_eq!(valid_name("foobarfoobarfoobarfoobarfoobarfoobarfoobarfoobarfoobarfoobarfoobarfoobar"),false);
	assert_eq!(valid_name(""),false);
	assert_eq!(valid_name("foo.123"),false);
	assert_eq!(valid_name("foo_123"),false);
	assert_eq!(valid_name("foo*123"),false);
}


impl NodeDoubleFmt {

	pub fn from_str(snd: &str) -> Result<Self, ParseFailure> {
		let s = snd.to_lowercase();
		let parts : Vec<&str> = s.split(",").collect();
		
		if parts.len() != 2 { 
			return Err(ParseFailure::InvalidContentFormat) 
		}
		
		if valid_name(parts[0]) == false || valid_name(parts[1]) == false {
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

pub struct NodeTripleFmt {
	pub spring: String,
	pub host: String,
	pub address: String,
}

impl NodeTripleFmt {

	pub fn from_str(snt: &str) -> Result<Self, ParseFailure> {
		let s = snt.to_lowercase();
		let parts : Vec<&str> = s.split(",").collect();
		
		if parts.len() != 3 || parts[2].len() < 7 { 
			return Err(ParseFailure::InvalidContentFormat) 
		}

		if valid_name(parts[0]) == false || valid_name(parts[1]) == false {
			return Err(ParseFailure::InvalidContentFormat)
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



/*
pub fn str_address_to_ipv4(address: &str) -> Result<Ipv4, Failure> {
	let atom: Vec<&str> = address.split('.').collect();
	
	if atom.len() != 4 {
		return Err(Failure::InvalidFormat);
	};
		
	let mut addr: Ipv4 = [0;4];
	
	for i in 0..4 {
		
		addr[i] = match atom[i].parse::<u32>().unwrap() {
			v if v < 0xFF  => v,
			_ => return Err(Failure::InvalidBytes)
		} as u8;
	}

	Ok(addr)
}

pub fn ipv4_to_str_address(address: &Ipv4) -> String {
	format(format_args!("{}.{}.{}.{}", address[0],address[1],address[2],address[3]))
}


pub fn nodes_to_node_list(nodes: &Vec<Node>) -> String {
	let mut s = String::new();
	for n in nodes {
		s.push_str(&format(format_args!("{};", &n.to_node_string())));
	}
	
	s
}



pub fn nodestring_from_node_register(nodereg: &str, address: &Ipv4) -> String {
	//let mut ns : String = nodereg.to_string();
	format(format_args!("{},{}", nodereg, ipv4_to_str_address(address)))
	//ns
}

pub fn geosub_from_node_register_gtn(nodereg: &str) -> Result<String,Failure> {
	let atom : Vec<&str> = nodereg.split(',').collect();
	match atom.len() { 
		4 => Ok(String::from(atom[3])),
		_ => Err(Failure::InvalidFormat), 
	}
	
}
*/