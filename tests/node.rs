extern crate spring_dvs;

use spring_dvs::node::*;

#[test]
fn ts_node_from_str_format_node_single_pass() {
	let o = Node::from_str("foobar");
	assert!(o.is_ok());
	
	let n : Node = o.unwrap();
	assert_eq!(n.springname(), "foobar");
}

#[test]
fn ts_node_from_str_format_node_double_pass() {
	let o = Node::from_str("foobar,barfoo");
	assert!(o.is_ok());
	
	let n : Node = o.unwrap();
	assert_eq!(n.springname(), "foobar");
	assert_eq!(n.hostname(), "barfoo");
}

#[test]
fn ts_node_from_str_format_node_triple_pass() {
	let o = Node::from_str("foobar,barfoo,127.3.4.5");
	assert!(o.is_ok());
	
	let n : Node = o.unwrap();
	assert_eq!(n.springname(), "foobar");
	assert_eq!(n.hostname(), "barfoo");
	assert_eq!(n.address(), "127.3.4.5");
}

#[test]
fn ts_node_from_str_format_node_quad_pass() {
	let o = Node::from_str("foobar,barfoo,127.3.4.5,http");
	assert!(o.is_ok());
	
	let n : Node = o.unwrap();
	assert_eq!(n.springname(), "foobar");
	assert_eq!(n.hostname(), "barfoo");
	assert_eq!(n.address(), "127.3.4.5");
	assert_eq!(n.service(), NodeService::Http);
}