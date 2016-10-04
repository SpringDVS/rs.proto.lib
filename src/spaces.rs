/* Notice:  Copyright 2016, The Care Connections Initiative c.i.c.
 * Author:  Charlie Fyvie-Gauld (cfg@zunautica.org)
 * License: GPLv3 (http://www.gnu.org/licenses/gpl-3.0.txt)
 */

pub use protocol::*;
pub use ::node::Node;
use ::enums::{Success};

#[derive(Debug,PartialEq)]
pub enum NetspaceFailure {
	NodeNotFound,
	DuplicateNode,
	DatabaseError,
}


pub trait Netspace {
	fn gsn_nodes(&self) -> Vec<Node>;
	fn gsn_nodes_by_address(&self, address: &str) -> Vec<Node>;
	fn gsn_nodes_by_type(&self, types: NodeRole) -> Vec<Node>;
	fn gsn_nodes_by_state(&self, state: NodeState) -> Vec<Node>;
	
	fn gsn_node_by_springname(&self, name: &str) -> Result<Node,NetspaceFailure>;
	fn gsn_node_by_hostname(&self, name: &str) -> Result<Node,NetspaceFailure>;
	
	
	fn gtn_root_nodes(&self) -> Vec<Node>;
	fn gtn_geosubs(&self) -> Vec<String>;
	
	fn gsn_node_register(&self, node: &Node) -> Result<Success,NetspaceFailure>;
	fn gsn_node_unregister(&self, node: &Node) -> Result<Success,NetspaceFailure>;

	fn gsn_node_update_state(&self, node: &Node) -> Result<Success,NetspaceFailure>;
	fn gsn_node_update_service(&self, node: &Node) -> Result<Success,NetspaceFailure>;
	fn gsn_node_update_role(&self, node: &Node) -> Result<Success,NetspaceFailure>;
	fn gsn_node_update_hostname(&self, node: &Node) -> Result<Success,NetspaceFailure>;
	fn gsn_node_update_address(&self, node: &Node) -> Result<Success,NetspaceFailure>;
	
	fn gtn_geosub_root_nodes(&self, gsn: &str) -> Vec<Node>;
	fn gtn_geosub_node_by_springname(&self, name: &str, gsn: &str) -> Result<Node,NetspaceFailure>;
	 
	fn gtn_geosub_register_node(&self, node: &Node, gsn: &str) -> Result<Success,NetspaceFailure>;
	fn gtn_geosub_unregister_node(&self, node: &Node, gsn: &str) -> Result<Success,NetspaceFailure>;
	
	fn gsn_check_token(&self, token: &str) -> bool;
	
}

pub trait Metaspace {
	fn gsn_resolve(metadata: String) -> Vec<String>;
}