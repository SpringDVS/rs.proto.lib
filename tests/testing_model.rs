/* Notice:  Copyright 2016, The Care Connections Initiative c.i.c.
 * Author:  Charlie Fyvie-Gauld (cfg@zunautica.org)
 * License: GPLv3 (http://www.gnu.org/licenses/gpl-3.0.txt)
 */
extern crate spring_dvs;
use spring_dvs::model::Node;
use spring_dvs::enums::Failure;

#[test]
fn ts_model_node_from_node_string_p() {
	// Test pass
	let node_string = "spring,host,192.168.1.2";
	
	let r = Node::from_node_string(node_string);
	
	assert!(r.is_ok());
	
	let node = r.unwrap();
	
	assert_eq!("spring", node.springname());
	assert_eq!("host", node.hostname());
	assert_eq!([192,168,1,2], node.address());
}

#[test]
fn ts_model_node_from_node_string_f() {
	// Test Fail
	
	// Invalid address format
	let node_string1 = "spring,host,192.168.1";
	let r1 = Node::from_node_string(node_string1);
	assert!(r1.is_err());
	assert_eq!(Failure::InvalidFormat, r1.unwrap_err());
	
	// Invalid address byte
	let node_string2 = "spring,host,192.168.1.384";
	let r2 = Node::from_node_string(node_string2);
	assert!(r2.is_err());
	assert_eq!(Failure::InvalidBytes, r2.unwrap_err());
	
}

#[test]
fn ts_model_node_to_node_string_f() {
	let node_string = "spring,host,192.168.1.2"; 
	let r = Node::from_node_string(node_string);
	assert!(r.is_ok());
	assert_eq!(node_string, r.unwrap().to_node_string());
}