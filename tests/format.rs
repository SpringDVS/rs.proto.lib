extern crate spring_dvs;

use spring_dvs::formats::*;

macro_rules! assert_match {
	
	($chk:ident, $pass:pat) => (
		assert!(match $chk {
					$pass => true,
					_ => false
			})
	)
}

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
	assert_match!(o, Err(ParseFailure::InvalidNaming));

	let o = NodeSingleFmt::from_str("foo.bar");
	assert!(o.is_err());
	assert_match!(o, Err(ParseFailure::InvalidNaming));
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
	assert_match!(o, Err(ParseFailure::InvalidContentFormat));
}

#[test]
fn ts_format_node_double_fmt_fail_malformed() {

	let o = NodeDoubleFmt::from_str("foo,");
	assert!(o.is_err());
	assert_match!(o, Err(ParseFailure::InvalidNaming));

	let o = NodeDoubleFmt::from_str(",foo");
	assert!(o.is_err());
	assert_match!(o, Err(ParseFailure::InvalidNaming));
}

#[test]
fn ts_format_node_double_fmt_fail_bad_names() {
	let o = NodeDoubleFmt::from_str("foo.bar,foo");
	assert!(o.is_err());
	assert_match!(o, Err(ParseFailure::InvalidNaming));
	
	let o = NodeDoubleFmt::from_str("foo,foo.bar");
	assert!(o.is_err());
	assert_match!(o, Err(ParseFailure::InvalidNaming));
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
	assert_match!(o, Err(ParseFailure::InvalidContentFormat));
}

#[test]
fn ts_format_node_triple_fmt_fail_bad_names() {
	let o = NodeTripleFmt::from_str("foo.bar,foo,192.168.1.2");
	assert!(o.is_err());
	assert_match!(o, Err(ParseFailure::InvalidNaming));
		
	let o = NodeTripleFmt::from_str("foo,foo.bar,192.168.1.2");
	assert!(o.is_err());
	assert_match!(o, Err(ParseFailure::InvalidNaming));
}

#[test]
fn ts_format_node_triple_fmt_fail_malformed() {



	let o = NodeTripleFmt::from_str("bar,foo,1.0.");
	assert!(o.is_err());
	assert_match!(o, Err(ParseFailure::InvalidAddress));
	
	let o = NodeTripleFmt::from_str("foo,");
	assert!(o.is_err());
	assert_match!(o, Err(ParseFailure::InvalidContentFormat));
	
	let o = NodeTripleFmt::from_str("foo,,");
	assert!(o.is_err());
	assert_match!(o, Err(ParseFailure::InvalidAddress));
	
		let o = NodeTripleFmt::from_str("foo,,127.0.0.1");
	assert!(o.is_err());
	assert_match!(o, Err(ParseFailure::InvalidNaming));
	
	let o = NodeTripleFmt::from_str("foo,bar,");
	assert!(o.is_err());
	assert_match!(o, Err(ParseFailure::InvalidAddress));
	
	let o = NodeTripleFmt::from_str(",foo,127.0.0.1");
	assert!(o.is_err());
	assert_match!(o, Err(ParseFailure::InvalidNaming));
	
	let o = NodeTripleFmt::from_str(",foo,bar,192.168.1.1");
	assert!(o.is_err());
	assert_match!(o, Err(ParseFailure::InvalidContentFormat));
	
	let o = NodeTripleFmt::from_str("foo,,bar");
	assert_match!(o, Err(ParseFailure::InvalidAddress));
	assert!(o.is_err());
	
}

#[test]
fn ts_format_node_quad_fmt_pass() {
	let o = NodeQuadFmt::from_str("foo,bar,127.1.4.3,http");
	assert!(o.is_ok());
	let nq : NodeQuadFmt = o.unwrap();
	
	assert_eq!(nq.spring, "foo");
	assert_eq!(nq.host, "bar");
	assert_eq!(nq.address, "127.1.4.3");
	assert_eq!(nq.service, NodeService::Http);

}

#[test]
fn ts_format_node_quad_fmt_pass_to_string() {
	let o = NodeQuadFmt::from_str("foo,bar,127.1.4.3,dvsp");
	assert!(o.is_ok());
	let nq : NodeQuadFmt = o.unwrap();
	
	assert_eq!("foo,bar,127.1.4.3,dvsp", nq.to_string());
}

#[test]
fn ts_format_node_quad_fmt_fail_invalid_service() {
	let o = NodeQuadFmt::from_str("foo,bar,127.1.4.3,dvspd");
	assert!(o.is_err());
	assert!(match o {
			Err(ParseFailure::InvalidService) => true,
			_ => false	
		});
}
#[test]
fn ts_format_node_quad_fmt_fail_malformed() {

	let o = NodeQuadFmt::from_str("foo,bar,127.1.4,dvsp");
	assert!(o.is_err());
	assert_match!(o, Err(ParseFailure::InvalidAddress));
	
	let o = NodeQuadFmt::from_str("foo,bar,,dvsp");
	assert!(o.is_err());
	assert_match!(o, Err(ParseFailure::InvalidAddress));
	
	let o = NodeQuadFmt::from_str("foo,127.1.4,dvsp");
	assert!(o.is_err());
	assert_match!(o, Err(ParseFailure::InvalidContentFormat));
}

#[test]
fn ts_format_node_info_fmt_from_bytes_pass() {
	let o = NodeInfoFmt::from_str("spring:foo,host:bar,address:127.1.4.3,service:http");
	assert!(o.is_ok());
	
	let nf :  NodeInfoFmt = o.unwrap();
	assert_eq!(nf.spring, "foo");
	assert_eq!(nf.host, "bar");
	assert_eq!(nf.address, "127.1.4.3");
	assert_eq!(nf.service, NodeService::Http);
	assert_eq!(nf.state, NodeState::Unspecified);
	assert_eq!(nf.role, NodeRole::Undefined);

}

#[test]
fn ts_format_node_info_fmt_display_pass() {
	let o = NodeInfoFmt::from_str("spring:foo,host:bar,address:127.1.4.3,service:http,role:hybrid");
	assert!(o.is_ok());
	let nf :  NodeInfoFmt = o.unwrap();
	
	assert_eq!(format!("{}", nf), "spring:foo,host:bar,address:127.1.4.3,service:http,role:hybrid");
}

#[test]
fn ts_format_node_info_fmt_from_str_fail() {
	let o = NodeInfoFmt::from_str("spring:foo,host:bar,address:127.1.4.3,service:http,role:hy");
	assert!(o.is_err());
	assert_match!(o, Err(ParseFailure::InvalidRole));
	
	let o = NodeInfoFmt::from_str("spring:foo,host:bar,address:127.1.4.3,service:ftp,role:hybrid");
	assert!(o.is_err());
	assert_match!(o, Err(ParseFailure::InvalidService));

	let o = NodeInfoFmt::from_str("spring:foo,host:bar,address:127.1.4.3,service:http,role:hybrid,state:jacked");
	assert!(o.is_err());
	assert_match!(o, Err(ParseFailure::InvalidState));
}