extern crate spring_dvs;

use spring_dvs::protocol::*;

macro_rules! assert_match {
	
	($chk:ident, $pass:pat) => (
		assert!(match $chk {
					$pass => true,
					_ => false
			})
	)
}

#[test]
fn ts_from_bytes_fail_invalid_command() {
	let o = Message::from_bytes(b"void foobar");
	assert!(o.is_err());
	assert!( match o {
			Err(ParseFailure::InvalidCommand) => true,
			_ => false,
		});
}
#[test]
fn ts_from_bytes_fail_invalid_conversion() {
	let o = Message::from_bytes(&[0xc3,0x28]);
	assert!(o.is_err());
	assert!( match o {
			Err(ParseFailure::ConversionError) => true,
			_ => false,
		});
}



#[test]
fn ts_from_bytes_reg_pass() {
	let o = Message::from_bytes(b"reg foobar,hostbar;org;http");
	assert!(o.is_ok());
	let m : Message = o.unwrap();
	assert_eq!(m.cmd, CmdType::Register);
	
	assert!( match m.content {
			MessageContent::Registration(_) => true,
			_ => false,
	});
	
	let c : ContentRegistration = match m.content {
		MessageContent::Registration(s) => s,
		_ => return
	};
	
	assert_eq!(c.ndouble.spring, "foobar");
	assert_eq!(c.ndouble.host, "hostbar");
	assert_eq!(c.role, NodeRole::Org);
	assert_eq!(c.service, NodeService::Http);
}

#[test]
fn ts_from_bytes_reg_fail_zero() {
	let o = Message::from_bytes(b"reg");
	assert!(o.is_err());
	assert!( match o {
			Err(ParseFailure::InvalidContentFormat) => true,
			_ => false,
	});	
}

#[test]
fn ts_from_bytes_reg_fail_malformed() {

	let o = Message::from_bytes(b"reg foobar,bar;orgd;a");
	assert!(o.is_err());
	assert!( match o {
			Err(ParseFailure::InvalidRole) => true,
			_ => false,
	});

	let o = Message::from_bytes(b"reg foobar,bar;org;");
	assert!(o.is_err());
	assert!( match o {
			Err(ParseFailure::InvalidContentFormat) => true,
			_ => false,
	});
	let o = Message::from_bytes(b"reg foobar,bar;");
	assert!(o.is_err());
	assert!( match o {
			Err(ParseFailure::InvalidContentFormat) => true,
			_ => false,
	});

	let o = Message::from_bytes(b"reg bar,foobar;;foo");
	assert!(o.is_err());
	assert!( match o {
			Err(ParseFailure::InvalidContentFormat) => true,
			_ => false,
	});
	let o = Message::from_bytes(b"reg bar,foobar;;");
	assert!(o.is_err());
	assert!( match o {
			Err(ParseFailure::InvalidContentFormat) => true,
			_ => false,
	});
}

#[test]
fn ts_message_to_bytes_reg_pass() {
	let o = Message::from_bytes(b"reg foobar,bar;org;dvsp");
	assert!(o.is_ok());
	
	let m : Message = o.unwrap();
	
	let st = String::from_utf8(m.to_bytes()).unwrap();
	assert_eq!(st, "reg foobar,bar;org;dvsp");
}

#[test]
fn ts_from_bytes_ureg_pass() {
	let o = Message::from_bytes(b"ureg foobar");
	assert!(o.is_ok());
	let m : Message = o.unwrap();
	assert_eq!(m.cmd, CmdType::Unregister);
	assert!( match m.content {
			MessageContent::NodeSingle(_) => true,
			_ => false,
	});
	
	let c = match m.content {
		MessageContent::NodeSingle(s) => s,
		_ => return
	};
	
	assert_eq!(c.nsingle.spring, "foobar");
	
}

#[test]
fn ts_from_bytes_ureg_fail() {
	let o = Message::from_bytes(b"ureg");
	assert!(o.is_err());
	assert!( match o {
			Err(ParseFailure::InvalidContentFormat) => true,
			_ => false,
	});	
}

#[test]
fn ts_from_bytes_ureg_fail_invalid_name() {
	let o = Message::from_bytes(b"ureg foo.bar");
	assert!(o.is_err());
	assert!( match o {
			Err(ParseFailure::InvalidNaming) => true,
			_ => false,
	});	
}

#[test]
fn ts_message_to_bytes_ureg_pass() {
	let o = Message::from_bytes(b"ureg foobar");
	assert!(o.is_ok());
	
	let m : Message = o.unwrap();
	
	let st = String::from_utf8(m.to_bytes()).unwrap();
	assert_eq!(st, "ureg foobar");
}

#[test]
fn ts_from_bytes_content_network_pass() {
	let o = ContentNetwork::from_bytes(b"foo,bar,127.0.0.1,dvsp;bar,foo,127.0.0.2,http;");
	assert!(o.is_ok());
	
	let nw : ContentNetwork = o.unwrap();
	assert_eq!(nw.network.len(), 2);
	assert_eq!(nw.network[0].spring, "foo");
	assert_eq!(nw.network[1].spring, "bar");
}

#[test]
fn ts_from_bytes_content_network_fail_malformed() {
	let o = ContentNetwork::from_bytes(b"foobar,127.0.0.1,dvsp;bar,foo,127.0.0.2,http;");
	assert!(o.is_err());
}


#[test]
fn ts_content_response_from_bytes_pass_empty() {
	let o = ContentResponse::from_bytes(b"200");
	assert!(o.is_ok());
	
	let cr : ContentResponse  = o.unwrap();
	assert_eq!(cr.code, Response::Ok);
	assert_eq!(cr.content, ResponseContent::Empty);
}

#[test]
fn ts_content_response_from_bytes_pass_network_pass () {
	let o = ContentResponse::from_bytes(b"200 network foo,bar,127.0.0.1,dvsp;bar,foo,127.0.0.2,http;");
	assert!(o.is_ok());
	
	let cr : ContentResponse  = o.unwrap();
	assert_eq!(cr.code, Response::Ok);
	assert!(match cr.content {
			ResponseContent::Network(_) => true,
			_ => false,
		});
}

#[test]
fn ts_content_response_from_bytes_pass_node_info_pass () {
	let o = ContentResponse::from_bytes(b"200 node spring:foo,host:bar,state:unresponsive");
	assert!(o.is_ok());
	
	let cr : ContentResponse  = o.unwrap();
	assert_eq!(cr.code, Response::Ok);
	assert!(match cr.content {
			ResponseContent::NodeInfo(_) => true,
			_ => false,
		});
	
	let ni : ContentNodeInfo = match cr.content {
		ResponseContent::NodeInfo(n) => n,
		_ => return,
	};
	
	assert_eq!(ni.info.spring, "foo");
	assert_eq!(ni.info.host, "bar");
	assert_eq!(ni.info.state, NodeState::Unresponsive);
}

#[test]
fn ts_message_content_response_nodeinfo_from_bytes_pass () {
	let o = Message::from_bytes(b"200 node spring:foo,host:bar,state:unresponsive"); 
	assert!(o.is_ok());
	let m : Message = o.unwrap();
	
	let c = m.content;
	
	assert_match!(c, MessageContent::Response(_));
	let r : ContentResponse = match c { MessageContent::Response(r) => r, _ => return };
	assert_eq!(r.code, Response::Ok);
	
	let rc = r.content;
	assert_match!(rc, ResponseContent::NodeInfo(_));
	let ni : ContentNodeInfo = match rc { ResponseContent::NodeInfo(n) => n, _ => return };
	
	assert_eq!(format!("{}", ni), "spring:foo,host:bar,state:unresponsive");
}


#[test]
fn ts_message_content_response_node_info_to_bytes_pass () {
	let o = Message::from_bytes(b"200 node spring:foo,host:bar,state:unresponsive"); 
	assert!(o.is_ok());
	let m : Message = o.unwrap();
	let s = m.to_bytes();
	let st = String::from_utf8(s).unwrap();
	assert_eq!(st, "200 node spring:foo,host:bar,state:unresponsive");
	
}


#[test]
fn ts_content_response_to_string_pass_network_pass () {
	let o = ContentResponse::from_bytes(b"200 network foo,bar,127.0.0.1,dvsp;bar,foo,127.0.0.2,http;");
	assert!(o.is_ok());
	let cr : ContentResponse = o.unwrap();
	assert_eq!(format!("{}", cr), "200 network foo,bar,127.0.0.1,dvsp;bar,foo,127.0.0.2,http;");
}


#[test]
fn ts_message_content_response_network_from_bytes_pass () {
	let o = Message::from_bytes(b"200 network foo,bar,127.0.0.1,dvsp;bar,foo,127.0.0.2,http;"); 
	assert!(o.is_ok());
	let m : Message = o.unwrap();
	
	let c = m.content;
	
	assert_match!(c, MessageContent::Response(_));
	let r : ContentResponse = match c { MessageContent::Response(r) => r, _ => return };
	assert_eq!(r.code, Response::Ok);
	
	let rc = r.content;
	assert_match!(rc, ResponseContent::Network(_));
	
	let nw : ContentNetwork = match rc { ResponseContent::Network(n) => n, _ => return };
	
	assert_eq!(format!("{}", nw), "foo,bar,127.0.0.1,dvsp;bar,foo,127.0.0.2,http;");
}

#[test]
fn ts_message_content_response_network_to_bytes_pass () {
	let o = Message::from_bytes(b"200 network foo,bar,127.0.0.1,dvsp;bar,foo,127.0.0.2,http;"); 
	assert!(o.is_ok());
	let m : Message = o.unwrap();
	let s = m.to_bytes();
	let st = String::from_utf8(s).unwrap();
	assert_eq!(st, "200 network foo,bar,127.0.0.1,dvsp;bar,foo,127.0.0.2,http;")
	
}