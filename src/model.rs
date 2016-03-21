/* Notice:  Copyright 2016, The Care Connections Initiative c.i.c.
 * Author:  Charlie Fyvie-Gauld (cfg@zunautica.org)
 * License: GPLv3 (http://www.gnu.org/licenses/gpl-3.0.txt)
 */
use std::fmt::format;

use ::protocol::{Ipv4};
use ::enums::{DvspNodeState,DvspService,Failure};
use ::formats::{str_address_to_ipv4, ipv4_to_str_address};

#[derive(Debug)]
pub struct Node {
	springname: String,
	hostname: String,
	address: Ipv4,
	
	service: DvspService,
	state: DvspNodeState,
}


// --------- Implementations ----------- \\
impl Node {
	pub fn new( spring: String, host: String, address: Ipv4, service: DvspService, state: DvspNodeState ) -> Node {
			
		Node {
			springname: spring,
			hostname: host,
			address: address,
			
			service: service,
			state: state,
		}
			
	}
	
	pub fn from_node_string(nodestr: &str) -> Result<Node,Failure> {
		let atom : Vec<&str> = nodestr.split(',').collect();
		if atom.len() != 3 {
			return Err(Failure::InvalidArgument);
		}
		
		let addr = try!(str_address_to_ipv4(atom[2]));
		
		Ok(
			Node {
				springname: String::from(atom[0]),
				hostname: String::from(atom[1]),
				address: addr,
				
				service: DvspService::Undefined,
				state: DvspNodeState::Unspecified,
			}
		)
	}
	
	pub fn springname(&self) -> &str {
		self.springname.as_ref()
	}
	
	pub fn hostname(&self) -> &str {
		self.hostname.as_ref()
	}
	
	pub fn address(&self) -> Ipv4 {
		self.address
	}
	
	pub fn service(&self) -> DvspService {
		self.service
	}
	
	pub fn state(&self) -> DvspNodeState {
		self.state
	}
	
	pub fn to_node_string(&self) -> String {
		format(format_args!("{},{},{}", self.springname, self.hostname, ipv4_to_str_address(self.address) ))
	}
	
	pub fn to_node_register(&self) -> String {
		format(format_args!("{},{}", self.springname, self.hostname))
	}
}