/* Notice:  Copyright 2016, The Care Connections Initiative c.i.c.
 * Author:  Charlie Fyvie-Gauld (cfg@zunautica.org)
 * License: GPLv3 (http://www.gnu.org/licenses/gpl-3.0.txt)
 */
use std::fmt::format;

pub use ::protocol::*;
//pub use ::enums::{ParseFailure,NodeRole,Response,NodeService,NodeState};



#[derive(Debug, Clone)]
pub struct Node {
	springname: String,
	hostname: String,
	address: String,
	
	service: NodeService,
	state: NodeState,
	role: NodeRole,
	
	resource: String,
}

impl Node {
	pub fn new( spring: &str, host: &str, address: &str, service: NodeService, state: NodeState, role: NodeRole  ) -> Node {
		
		
		let (hostname,res) = match host.find("/") {
			None => (host, "/"),
			Some(p) => host.split_at(p)
		};
		
		Node {
			springname: String::from(spring),
			hostname: String::from(hostname),
			address: String::from(address),
			
			service: service,
			state: state,
			role: role,
			resource: String::from(&res[1..]),
		}
			
	}
	
	pub fn from_str(s: &str) -> Result<Node,ParseFailure> {
		let  v : Vec<&str> = s.split(",").collect();
		Ok(
			match v.len() {
				1 => {
					let t : NodeSingleFmt = try!(NodeSingleFmt::from_str(s));
					Node::new(&t.spring, "","0.0.0.0",NodeService::Undefined,NodeState::Unspecified, NodeRole::Undefined)
				},
				2 => {
					let t : NodeDoubleFmt = try!(NodeDoubleFmt::from_str(s));
					Node::new(&t.spring, &t.host,"0.0.0.0",NodeService::Undefined,NodeState::Unspecified, NodeRole::Undefined)
				},
				3 => {
					let t : NodeTripleFmt = try!(NodeTripleFmt::from_str(s));
					Node::new(&t.spring, &t.host, &t.address,NodeService::Undefined,NodeState::Unspecified, NodeRole::Undefined)
				},
				4 => {
					let t : NodeQuadFmt = try!(NodeQuadFmt::from_str(s));
					Node::new(&t.spring, &t.host, &t.address, t.service,NodeState::Unspecified, NodeRole::Undefined)
				}
				
				_ => return Err(ParseFailure::ConversionError)
			}
		)
	}
	
	pub fn springname(&self) -> &str {
		self.springname.as_ref()
	}
	
	pub fn hostname(&self) -> &str {
		self.hostname.as_ref()
	}
	
	pub fn address(&self) -> &str {
		self.address.as_ref()
	}
	
	pub fn service(&self) -> NodeService {
		self.service
	}
	
	pub fn update_service(&mut self, service: NodeService) {
		self.service = service;
	}

	
	pub fn state(&self) -> NodeState {
		self.state
	}
	
	pub fn resource(&self) -> &str {
		&self.resource
	}

	pub fn update_state(&mut self, state: NodeState) {
		self.state = state;
	}
	
	pub fn role(&self) -> NodeRole {
		self.role
	}
	
	pub fn update_role(&mut self, role: NodeRole) {
		self.role = role;
	}
}