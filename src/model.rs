/* Notice:  Copyright 2016, The Care Connections Initiative c.i.c.
 * Author:  Charlie Fyvie-Gauld (cfg@zunautica.org)
 * License: GPLv3 (http://www.gnu.org/licenses/gpl-3.0.txt)
 */
use std::fmt::format;

use ::protocol::{Ipv4, NodeTypeField};
use ::enums::{DvspNodeState,DvspService,DvspNodeType,Success,Failure};
use ::formats::{str_address_to_ipv4, ipv4_to_str_address};

#[derive(Debug)]
pub struct Node {
	springname: String,
	hostname: String,
	address: Ipv4,
	
	service: DvspService,
	state: DvspNodeState,
	types: NodeTypeField,
}

#[derive(Debug)]
pub struct Url {
	
	gsn: Vec<String>,
	gtn: String,
	glq: String,
	res: String,
	query: String,
}

pub trait Netspace {
	fn gsn_nodes(&self) -> Vec<Node>;
	fn gsn_nodes_by_address(&self, address: Ipv4) -> Vec<Node>;
	fn gsn_nodes_by_type(&self, types: NodeTypeField) -> Vec<Node>;
	fn gsn_nodes_by_state(&self, state: DvspNodeState) -> Vec<Node>;
	
	fn gsn_node_by_springname(&self, name: &str) -> Result<Node,Failure>;
	fn gsn_node_by_hostname(&self, name: &str) -> Result<Node,Failure>;
	
	
	fn gtn_root_nodes(&self) -> Vec<Node>;
	fn gtn_geosubs(&self) -> Vec<String>;
	
	fn gsn_node_register(&self, node: &Node) -> Result<Success,Failure>;
	fn gsn_node_unregister(&self, node: &Node) -> Result<Success,Failure>;

	fn gsn_node_update_state(&self, node: &Node) -> Result<Success,Failure>;
	fn gsn_node_update_service(&self, node: &Node) -> Result<Success,Failure>;
	fn gsn_node_update_types(&self, node: &Node) -> Result<Success,Failure>;
	
	fn gtn_geosub_root_nodes(&self, gsn: &str) -> Vec<Node>;
	fn gtn_geosub_node_by_springname(&self, name: &str, gsn: &str) -> Result<Node,Failure>;
	 
	fn gtn_geosub_register_node(&self, node: &Node, gsn: &str) -> Result<Success,Failure>;
	fn gtn_geosub_unregister_node(&self, node: &Node, gsn: &str) -> Result<Success,Failure>;
	
}

pub trait Metaspace {
	fn gsn_resolve(metadata: String) -> Vec<String>;
}

// --------- Implementations ----------- \\
impl Node {
	pub fn new( spring: String, host: String, address: Ipv4, service: DvspService, state: DvspNodeState, types: NodeTypeField  ) -> Node {
			
		Node {
			springname: spring,
			hostname: host,
			address: address,
			
			service: service,
			state: state,
			types: types,
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
				types: DvspNodeType::Undefined as u8,
			}
		)
	}
	
	pub fn from_springname(springname: &str) -> Result<Node,Failure> {
		Ok(
			Node {
				springname: String::from(springname),
				hostname: String::from("unknown"),
				address: [0,0,0,0],
				
				service: DvspService::Undefined,
				state: DvspNodeState::Unspecified,
				types: DvspNodeType::Undefined as u8,
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
	
	pub fn update_service(&mut self, service: DvspService) {
		self.service = service;
	}

	
	pub fn state(&self) -> DvspNodeState {
		self.state
	}

	pub fn update_state(&mut self, state: DvspNodeState) {
		self.state = state;
	}
	
	pub fn types(&self) -> NodeTypeField {
		self.types
	}
	
	pub fn update_types(&mut self, types: NodeTypeField) {
		self.types = types;
	}

	
	pub fn to_node_string(&self) -> String {
		format(format_args!("{},{},{}", self.springname, self.hostname, ipv4_to_str_address(&self.address) ))
	}
	
	pub fn to_node_register(&self) -> String {
		format(format_args!("{},{}", self.springname, self.hostname))
	}
}

impl Url {
	
	pub fn new(url: &str) -> Result<Url, Failure> {
		
		let initial : Vec<&str> = url.split("://").collect();

		if initial[0] != "spring" || initial.len() < 2 {
			return Err(Failure::InvalidFormat)
		}

		let mut gsn : Vec<String> = Vec::new();
		let mut glq: &str = "";
		let mut res: &str = "";
		let mut query: &str = "";


		let mut atoms : Vec<&str> = initial[1].split('?').collect();
		if atoms.len() > 1 {
			query = atoms[1]
		}

		atoms = atoms[0].split('/').collect();

		if atoms.len() > 1 {
			res = atoms[1]
		}
		
		atoms = atoms[0].split(':').collect();

		if atoms.len() > 1 {
			glq = atoms[1]
		}
		
		let v : Vec<&str> = atoms[0].split('.').collect();
		
		
		let gtn = match v[v.len()-1] {
			"uk" => "uk",
			_ => ""
		};
		
		for s in v {
			gsn.push(String::from(s))
		}
		
		Ok(Url {
			gsn: gsn,
			gtn: String::from(gtn),
			glq: String::from(glq),
			res: String::from(res),
			query: String::from(query),
		})
	}
		
	pub fn route(&self) -> &Vec<String> {
		&self.gsn
	}
	
	pub fn route_mut(&mut self) -> &mut Vec<String> {
		&mut self.gsn
	} 
	
	
	pub fn gtn(&self) -> &str {
		&self.gtn
	}

	pub fn glq(&self) -> &str {
		&self.glq
	}

	pub fn query(&self) -> &str {
		&self.query
	}
	
	pub fn res(&self) -> &str {
		&self.res
	}
	
	pub fn to_string(&self) -> String {
		
		let mut s = "spring://".to_string();
		let last = self.gsn.len()-1;
		
		for i in 0 .. last {
			s.push_str(&self.gsn[i]);
			s.push('.');
		}
		
		s.push_str(&self.gsn[last]);
		
		if self.glq.len() > 0 {
			s.push(':');
			s.push_str(&self.glq);
		}

		if self.res.len() > 0 {
			s.push('/');
			s.push_str(&self.res);
		}

		if self.query.len() > 0 {
			s.push('?');
			s.push_str(&self.query);
		}
		s
	}
	
}

impl Clone for Url {
	fn clone(&self) -> Url {
		Url {
			gsn: (&self).gsn.clone(),
			gtn: (&self).gtn.to_string(),
			glq: (&self).glq.to_string(),
			res: (&self).res.to_string(),
			query: (&self).query.to_string()
		}
	}

	fn clone_from(&mut self, source: &Url)  {
			self.gsn = source.route().clone();
			self.gtn = source.gtn().to_string();
			self.glq = source.glq().to_string();
			self.res = source.res().to_string();
			self.query = source.query().to_string();
	}
}

pub fn nodes_from_nodelist(nodelist: &str) -> Vec<Node> {
	let mut v : Vec<Node> = Vec::new();
	let nstr : Vec<&str> = nodelist.split(";").collect();
	
	for n in nstr {
		
		let node = match Node::from_node_string(n) {
			Err(_) => continue,
			Ok(n) => n,
		};
		
		v.push(node);
	}
	v
}