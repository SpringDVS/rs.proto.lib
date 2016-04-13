/* Notice:  Copyright 2016, The Care Connections Initiative c.i.c.
 * Author:  Charlie Fyvie-Gauld (cfg@zunautica.org)
 * License: GPLv3 (http://www.gnu.org/licenses/gpl-3.0.txt)
 */

use std::fmt::format;

use ::protocol::Ipv4;
use ::enums::Failure;

use ::model::Node;

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