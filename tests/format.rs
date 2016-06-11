extern crate spring_dvs;

use spring_dvs::formats::*;

#[test]
fn ts_format_node_single_fmt_pass() {
	let o = NodeSingleFmt::from_str("foo");
	assert!(o.is_ok());
	let ns : NodeSingleFmt = o.unwrap();
	assert_eq!(ns.spring, "foo");
}

#[test]
fn ts_format_node_single_fmt_pass_to_string() {
	let o = NodeSingleFmt::from_str("foo");
	assert!(o.is_ok());
	let nd : NodeSingleFmt = o.unwrap();
	
	assert_eq!("foo", nd.to_string());
}

#[test]
fn ts_format_node_single_fmt_fail_zero() {
	let o = NodeSingleFmt::from_str("");
	assert!(o.is_err());
}

#[test]
fn ts_format_node_single_fmt_fail_malformed() {

	let o = NodeSingleFmt::from_str("foo*");
	assert!(o.is_err());

	let o = NodeSingleFmt::from_str("foo.bar");
	assert!(o.is_err());
}


#[test]
fn ts_format_node_double_fmt_pass() {
	let o = NodeDoubleFmt::from_str("foo,bar");
	assert!(o.is_ok());
	let nd : NodeDoubleFmt = o.unwrap();
	assert_eq!(nd.spring, "foo");
	assert_eq!(nd.host, "bar");
}

#[test]
fn ts_format_node_double_fmt_pass_to_string() {
	let o = NodeDoubleFmt::from_str("foo,bar");
	assert!(o.is_ok());
	let nd : NodeDoubleFmt = o.unwrap();
	
	assert_eq!("foo,bar", nd.to_string());
}

#[test]
fn ts_format_node_double_fmt_fail_zero() {
	let o = NodeDoubleFmt::from_str("");
	assert!(o.is_err());
}

#[test]
fn ts_format_node_double_fmt_fail_malformed() {

	let o = NodeDoubleFmt::from_str("foo,");
	assert!(o.is_err());

	let o = NodeDoubleFmt::from_str(",foo");
	assert!(o.is_err());
}

#[test]
fn ts_format_node_double_fmt_fail_bad_names() {
	let o = NodeTripleFmt::from_str("foo.bar,foo");
	assert!(o.is_err());
	
	let o = NodeTripleFmt::from_str("foo,foo.bar");
	assert!(o.is_err());
}


#[test]
fn ts_format_node_triple_fmt_pass() {
	let o = NodeTripleFmt::from_str("foo,bar,192.168.1.2");
	assert!(o.is_ok());
	let nd : NodeTripleFmt = o.unwrap();
	assert_eq!(nd.spring, "foo");
	assert_eq!(nd.host, "bar");
	assert_eq!(nd.address, "192.168.1.2");
}

#[test]
fn ts_format_node_triple_fmt_pass_to_string() {
	let o = NodeTripleFmt::from_str("foo,bar,192.168.1.2");
	assert!(o.is_ok());
	let nd : NodeTripleFmt = o.unwrap();
	
	assert_eq!("foo,bar,192.168.1.2", nd.to_string());
}

#[test]
fn ts_format_node_triple_fmt_fail_zero() {
	let o = NodeDoubleFmt::from_str("");
	assert!(o.is_err());
}

#[test]
fn ts_format_node_triple_fmt_fail_bad_names() {
	let o = NodeTripleFmt::from_str("foo.bar,foo,192.168.1.2");
	assert!(o.is_err());
	
	let o = NodeTripleFmt::from_str("foo,foo.bar,192.168.1.2");
	assert!(o.is_err());
}

#[test]
fn ts_format_node_triple_fmt_fail_malformed() {



	let o = NodeTripleFmt::from_str(",foo,1.0.");
	assert!(o.is_err());
	
	let o = NodeTripleFmt::from_str("foo,");
	assert!(o.is_err());
	
	let o = NodeTripleFmt::from_str("foo,,");
	assert!(o.is_err());
	let o = NodeTripleFmt::from_str("foo,bar,");
	assert!(o.is_err());
	
	let o = NodeTripleFmt::from_str(",foo,");
	assert!(o.is_err());
	let o = NodeTripleFmt::from_str(",foo,bar");
	assert!(o.is_err());
	
	let o = NodeTripleFmt::from_str("foo,,bar");
	assert!(o.is_err());
	
}
