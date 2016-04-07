/* Notice:  Copyright 2016, The Care Connections Initiative c.i.c.
 * Author:  Charlie Fyvie-Gauld (cfg@zunautica.org)
 * License: GPLv3 (http://www.gnu.org/licenses/gpl-3.0.txt)
 */
extern crate spring_dvs;
use spring_dvs::model::*;
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

#[test]
fn ts_model_url_new_basic_p() {
	
	let r = Url::new("spring://cci.esusx.uk");
	
	assert!(r.is_ok());
	
	let url = r.unwrap();
	let gsn = url.gsn();
	assert_eq!(gsn.len(), 3);
	assert_eq!(gsn[0], "cci");
	assert_eq!(gsn[1], "esusx");
	assert_eq!(gsn[2], "uk");
	assert_eq!(url.gtn(), "uk");
}

#[test]
fn ts_model_url_new_basic_no_gtn_p() {
	
	let r = Url::new("spring://cci.esusx");
	
	assert!(r.is_ok());
	
	let url = r.unwrap();
	let gsn = url.gsn();
	assert_eq!(gsn.len(), 2);
	assert_eq!(gsn[0], "cci");
	assert_eq!(gsn[1], "esusx");
	assert_eq!(url.gtn(), "");
}

#[test]
fn ts_model_url_new_basic_glq_p() {
	
	let r = Url::new("spring://cci.esusx.uk:foo=bar");
	
	assert!(r.is_ok());
	
	let url = r.unwrap();
	let gsn = url.gsn();
	assert_eq!(gsn.len(), 3);
	assert_eq!(gsn[0], "cci");
	assert_eq!(gsn[1], "esusx");
	assert_eq!(gsn[2], "uk");
	assert_eq!(url.gtn(), "uk");
	assert_eq!(url.glq(), "foo=bar");
}

#[test]
fn ts_model_url_new_basic_glq_res_p() {
	
	let r = Url::new("spring://cci.esusx.uk:foo=bar/res");
	
	assert!(r.is_ok());
	
	let url = r.unwrap();
	let gsn = url.gsn();
	assert_eq!(gsn.len(), 3);
	assert_eq!(gsn[0], "cci");
	assert_eq!(gsn[1], "esusx");
	assert_eq!(gsn[2], "uk");
	assert_eq!(url.gtn(), "uk");
	assert_eq!(url.glq(), "foo=bar");
	assert_eq!(url.res(), "res");
}

#[test]
fn ts_model_url_new_basic_glq_res_query_p() {
	
	let r = Url::new("spring://cci.esusx.uk:foo=bar/res?query:test");
	
	assert!(r.is_ok());
	
	let url = r.unwrap();
	let gsn = url.gsn();
	assert_eq!(gsn.len(), 3);
	assert_eq!(gsn[0], "cci");
	assert_eq!(gsn[1], "esusx");
	assert_eq!(gsn[2], "uk");
	assert_eq!(url.gtn(), "uk");
	assert_eq!(url.glq(), "foo=bar");
	assert_eq!(url.res(), "res");
	assert_eq!(url.query(), "query:test");
}

#[test]
fn ts_model_url_new_f() {
	
	let r = Url::new("cci.esusx.uk:foo=bar/res?query:test");
	
	assert!(r.is_err());
	assert_eq!(Failure::InvalidFormat, r.unwrap_err());
}

#[test]
fn ts_model_url_to_string_basic_p() {
	let s = "spring://cci.esusx.uk";
	let r = Url::new(s);
	
	assert!(r.is_ok());
	let url = r.unwrap();
	
	assert_eq!(s, url.to_string());
}

#[test]
fn ts_model_url_to_string_basic_glq_p() {
	let s = "spring://cci.esusx.uk:foo=bar";
	let r = Url::new(s);
	
	assert!(r.is_ok());
	let url = r.unwrap();
	
	assert_eq!(s, url.to_string());
}

#[test]
fn ts_model_url_to_string_basic_glq_res_p() {
	let s = "spring://cci.esusx.uk:foo=bar/res";
	let r = Url::new(s);
	
	assert!(r.is_ok());
	let url = r.unwrap();
	
	assert_eq!(s, url.to_string());
}

#[test]
fn ts_model_url_to_string_basic_glq_res_query_p() {
	let s = "spring://cci.esusx.uk:foo=bar/res?query:test";
	let r = Url::new(s);
	
	assert!(r.is_ok());
	let url = r.unwrap();
	
	assert_eq!(s, url.to_string());
}

#[test]
fn ts_model_url_clone_p() {
	let s = "spring://cci.esusx.uk:foo=bar/res?query:test";
	let r = Url::new(s);
	
	assert!(r.is_ok());
	let url = r.unwrap();
	
	let cpy = url.clone();
	
	assert_eq!(s, cpy.to_string());
}

#[test]
fn ts_model_url_clone_from_p() {
	let s = "spring://cci.esusx.uk:foo=bar/res?query:test";
	let r = Url::new(s);
	
	assert!(r.is_ok());
	let url = r.unwrap();
	
	let mut cpy: Url = Url::new("spring://").unwrap();
	cpy.clone_from(&url);
	
	assert_eq!(s, cpy.to_string());
}

#[test]
fn ts_nodes_from_node_string_p() {
	let nodelist = "s1,h1,192.168.0.1;s2,h2,192.168.0.2;s3,h3,192.168.0.3;";
	let v = nodes_from_nodelist(nodelist);
	
	assert_eq!(3, v.len());
	
	assert_eq!("s1", v[0].springname());
	assert_eq!("h2", v[1].hostname());
	assert_eq!([192,168,0,3], v[2].address());
}