/* Notice:  Copyright 2016, The Care Connections Initiative c.i.c.
 * Author:  Charlie Fyvie-Gauld (cfg@zunautica.org)
 * License: GPLv3 (http://www.gnu.org/licenses/gpl-3.0.txt)
 */
pub use protocol::*;





#[derive(Debug, Clone)]
pub struct Node {
	springname: String,
	hostname: String,
	address: String,
	
	service: NodeService,
	state: NodeState,
	role: NodeRole,
	
	resource: String,
	key: String
}

pub fn nodevec_quadvec(v: Vec<Node>) -> Vec<NodeQuadFmt> {

	let mut out : Vec<NodeQuadFmt> = Vec::new();
	for n in v {
		match n.to_node_quad() {
			Some(q) => out.push(q),
			None => continue,
		}
	}

	out
}

impl Node {
	pub fn new( spring: &str, host: &str, address: &str, service: NodeService, state: NodeState, role: NodeRole, key: &str ) -> Self {
		
		
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
			key: String::from(key)
		}
			
	}
	
	pub fn from_registration(reg: &ContentRegistration, address: &str) -> Self {
		Node::new(
			&reg.ndouble.spring,
			&reg.ndouble.host,
			&String::from(address),
			reg.service,
			NodeState::Disabled,
			reg.role,
			&reg.key,
		)
	}
	
	pub fn from_str(s: &str) -> Result<Node,ParseFailure> {
		match  s.find(":") {
			Some(_) =>  {
					let t : NodeInfoFmt = try!(NodeInfoFmt::from_str(s));
					return Ok(Node::new(&t.spring, &t.host, &t.address, t.service,t.state, t.role,""))
				},
			_ => { }
		};
		
		let  v : Vec<&str> = s.split(",").collect();
		Ok(
			match v.len() {
				1 => {
					let t : NodeSingleFmt = try!(NodeSingleFmt::from_str(s));
					Node::new(&t.spring, "","0.0.0.0",NodeService::Undefined,NodeState::Unspecified, NodeRole::Undefined,"")
				},
				2 => {
					let t : NodeDoubleFmt = try!(NodeDoubleFmt::from_str(s));
					Node::new(&t.spring, &t.host,"0.0.0.0",NodeService::Undefined,NodeState::Unspecified, NodeRole::Undefined,"")
				},
				3 => {
					let t : NodeTripleFmt = try!(NodeTripleFmt::from_str(s));
					Node::new(&t.spring, &t.host, &t.address,NodeService::Undefined,NodeState::Unspecified, NodeRole::Undefined,"")
				},
				4 => {
					let t : NodeQuadFmt = try!(NodeQuadFmt::from_str(s));
					Node::new(&t.spring, &t.host, &t.address, t.service,NodeState::Unspecified, NodeRole::Undefined,"")
				},
				
				_ => return Err(ParseFailure::ConversionError)
			}
		)
	}
	
	pub fn from_node_single(n: &NodeSingleFmt) -> Self {
		Node::new(&n.spring, "", "", NodeService::Undefined, NodeState::Unspecified, NodeRole::Undefined,"")
	}

	pub fn to_node_single(&self) -> Option<NodeSingleFmt> {
		if self.springname.is_empty() { return None }
		
		Some(NodeSingleFmt { 
			spring: self.springname.clone()
		})
	}

	pub fn from_node_double(n: &NodeDoubleFmt) -> Self {
		Node::new(&n.spring, &n.host, "", NodeService::Undefined, NodeState::Unspecified, NodeRole::Undefined,"")
	}

	pub fn to_node_double(&self) -> Option<NodeDoubleFmt> {
		
		if self.springname.is_empty() { return None }
		if self.hostname.is_empty() { return None }
		
		Some(NodeDoubleFmt { 
			spring: self.springname.clone(),
			host: self.hostname.clone()
		})
	}
	
	pub fn from_node_triple(n: &NodeTripleFmt) -> Self {
		Node::new(&n.spring, &n.host, &n.address, NodeService::Undefined, NodeState::Unspecified, NodeRole::Undefined,"")
	}

	pub fn to_node_triple(&self) -> Option<NodeTripleFmt> {
		if self.springname.is_empty() { return None }
		if self.hostname.is_empty() { return None }
		if self.address.is_empty() { return None }
		
		Some(NodeTripleFmt { 
			spring: self.springname.clone(),
			host: self.hostname.clone(),
			address: self.address.clone(),
		})
	}

	pub fn from_node_quad(n: &NodeQuadFmt) -> Self {
		Node::new(&n.spring, &n.host, &n.address, n.service, NodeState::Unspecified, NodeRole::Undefined,"")
	}

	pub fn to_node_quad(&self) -> Option<NodeQuadFmt> {

		if self.springname.is_empty() { return None }
		if self.hostname.is_empty() { return None }
		if self.address.is_empty() { return None }

		Some(NodeQuadFmt { 
			spring: self.springname.clone(),
			host: self.hostname.clone(),
			address: self.address.clone(),
			service: self.service,
		})
	}
	
	pub fn to_node_info(&self) -> Option<NodeInfoFmt> {
		
		if self.springname.is_empty() { return None }
		if self.hostname.is_empty() { return None }
		if self.address.is_empty() { return None }

		Some(NodeInfoFmt {
			spring: self.springname.clone(),
			host: self.hostname.clone(),
			address: self.address.clone(),
			service: self.service,
			state: self.state,
			role: self.role,
		})
	}
	
	pub fn to_node_info_property(&self, property: NodeProperty) -> NodeInfoFmt {
		
		let mut info = NodeInfoFmt::new();
		
		match property {
			
			NodeProperty::Hostname => { info.host = self.hostname.clone() },
			NodeProperty::Address => { info.address = self.address.clone() },
			NodeProperty::Service(_) => { info.service = self.service },
			NodeProperty::Role(_) => { info.role = self.role },
			NodeProperty::State(_) => { info.state = self.state },
			
			NodeProperty::All => { match self.to_node_info() { 
										Some(o) => info = o, None => {} 
									}
								},
		};
		
		info
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
	
	pub fn key(&self) -> &str {
		self.key.as_ref()
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
	
	pub fn update_key(&mut self, key: &str) {
		self.key = String::from(key)
	}
}