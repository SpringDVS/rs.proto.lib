#[macro_use]
extern crate spring_dvs;

use spring_dvs::protocol::*;

macro_rules! assert_match {
	
	($chk:expr, $pass:pat) => (
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
	let o = Message::from_bytes(b"register foobar,hostbar;org;http;abcdef");
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
	assert_eq!(c.token, "abcdef");
}

#[test]
fn ts_from_bytes_reg_fail_zero() {
	let o = Message::from_bytes(b"register");
	assert!(o.is_err());
	assert!( match o {
			Err(ParseFailure::InvalidContentFormat) => true,
			_ => false,
	});	
}

#[test]
fn ts_from_bytes_reg_fail_malformed() {

	let o = Message::from_bytes(b"register foobar,bar;orgd;a;b");
	assert!(o.is_err());
	assert!( match o {
			Err(ParseFailure::InvalidRole) => true,
			_ => false,
	});

	let o = Message::from_bytes(b"register foobar,bar;org;");
	assert!(o.is_err());
	assert!( match o {
			Err(ParseFailure::InvalidContentFormat) => true,
			_ => false,
	});
	let o = Message::from_bytes(b"register foobar,bar;");
	assert!(o.is_err());
	assert!( match o {
			Err(ParseFailure::InvalidContentFormat) => true,
			_ => false,
	});

	let o = Message::from_bytes(b"register bar,foobar;;foo");
	assert!(o.is_err());
	assert!( match o {
			Err(ParseFailure::InvalidContentFormat) => true,
			_ => false,
	});
	let o = Message::from_bytes(b"register bar,foobar;;");
	assert!(o.is_err());
	assert!( match o {
			Err(ParseFailure::InvalidContentFormat) => true,
			_ => false,
	});
}

#[test]
fn ts_message_to_bytes_reg_pass() {
	let o = Message::from_bytes(b"register foobar,bar;org;dvsp;abcdef");
	assert!(o.is_ok());
	
	let m : Message = o.unwrap();
	
	let st = String::from_utf8(m.to_bytes()).unwrap();
	assert_eq!(st, "register foobar,bar;org;dvsp;abcdef");
}

#[test]
fn ts_from_bytes_ureg_pass() {
	let o = Message::from_bytes(b"unregister foobar");
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
	let o = Message::from_bytes(b"unregister");
	assert!(o.is_err());
	assert!( match o {
			Err(ParseFailure::InvalidContentFormat) => true,
			_ => false,
	});	
}

#[test]
fn ts_from_bytes_ureg_fail_invalid_name() {
	let o = Message::from_bytes(b"unregister foo.bar");
	assert!(o.is_err());
	assert!( match o {
			Err(ParseFailure::InvalidNaming) => true,
			_ => false,
	});	
}

#[test]
fn ts_message_to_bytes_ureg_pass() {
	let o = Message::from_bytes(b"unregister foobar");
	assert!(o.is_ok());
	
	let m : Message = o.unwrap();
	
	let st = String::from_utf8(m.to_bytes()).unwrap();
	assert_eq!(st, "unregister foobar");
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
	let o = ContentResponse::from_bytes(b"200 99 network foo,bar,127.0.0.1,dvsp;bar,foo,127.0.0.2,http;");
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
	let o = ContentResponse::from_bytes(b"200 99 node spring:foo,host:bar,state:unresponsive");
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
	let o = Message::from_bytes(b"200 99 node spring:foo,host:bar,state:unresponsive"); 
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
	let o = Message::from_bytes(b"200 43 node spring:foo,host:bar,state:unresponsive"); 
	assert!(o.is_ok());
	let m : Message = o.unwrap();
	let s = m.to_bytes();
	let st = String::from_utf8(s).unwrap();
	assert_eq!(st, "200 43 node spring:foo,host:bar,state:unresponsive");
	
}


#[test]
fn ts_content_response_to_string_pass_network_pass () {
	let o = ContentResponse::from_bytes(b"200 55 network foo,bar,127.0.0.1,dvsp;bar,foo,127.0.0.2,http;");
	assert!(o.is_ok());
	let cr : ContentResponse = o.unwrap();
	assert_eq!(format!("{}", cr), "200 54 network foo,bar,127.0.0.1,dvsp;bar,foo,127.0.0.2,http;");

}

#[test]
fn ts_message_content_response_network_from_bytes_pass () {
	let o = Message::from_bytes(b"200 54 network foo,bar,127.0.0.1,dvsp;bar,foo,127.0.0.2,http;"); 
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
	let o = Message::from_bytes(b"200 54 network foo,bar,127.0.0.1,dvsp;bar,foo,127.0.0.2,http;"); 
	assert!(o.is_ok());
	let m : Message = o.unwrap();
	let s = m.to_bytes();
	let st = String::from_utf8(s).unwrap();
	assert_eq!(st, "200 54 network foo,bar,127.0.0.1,dvsp;bar,foo,127.0.0.2,http;")
	
}

#[test]
fn ts_message_content_response_service_text_from_bytes_pass () {
	let o = Message::from_bytes(b"200 19 service/text foobar"); 
	assert!(o.is_ok());
	let m : Message = o.unwrap();
	
	let c = m.content;
	
	assert_match!(c, MessageContent::Response(_));
	let r : ContentResponse = match c { MessageContent::Response(r) => r, _ => return };
	assert_eq!(r.code, Response::Ok);
	
	let rc = r.content;
	assert_match!(rc, ResponseContent::ServiceText(_));
	
	let nw : ContentServiceText = match rc { ResponseContent::ServiceText(n) => n, _ => return };
	
	assert_eq!(format!("{}", nw), "foobar");
}

#[test]
fn ts_message_content_response_service_text_to_bytes_pass () {
	let o = Message::from_bytes(b"200 19 service/text foobar"); 
	assert!(o.is_ok());
	let m : Message = o.unwrap();
	let s = m.to_bytes();
	let st = String::from_utf8(s).unwrap();
	assert_eq!(st, "200 19 service/text foobar")
	
}

#[test]
fn ts_content_info_request_network_from_bytes_pass () {
	let o = ContentInfoRequest::from_bytes(b"network"); 
	assert!(o.is_ok());
	let cir : ContentInfoRequest = o.unwrap();
	assert_eq!(cir.info, InfoContent::Network);
	
	
}

#[test]
fn ts_content_info_request_network_from_bytes_fail () {
	let o = ContentInfoRequest::from_bytes(b"netwddodrk"); 
	assert!(o.is_err());
	assert_match!(o, Err(ParseFailure::InvalidContentFormat));

	let o = ContentInfoRequest::from_bytes(b""); 
	assert!(o.is_err());
	assert_match!(o, Err(ParseFailure::InvalidContentFormat));	
}

#[test]
fn ts_content_info_request_network_to_string_pass () {
	let o = ContentInfoRequest::from_bytes(b"network"); 
	assert!(o.is_ok());
	let cir = o.unwrap();
	assert_eq!(format!("{}", cir), "network");
}

#[test]
fn ts_content_info_request_node_property_from_bytes_pass () {
	let o = ContentInfoRequest::from_bytes(b"node spring all"); 
	assert!(o.is_ok());
	let cir : ContentInfoRequest = o.unwrap();
	
	let info = cir.info;
	assert_match!(info, InfoContent::Node(_));
	let ni : ContentNodeProperty = match info {
		InfoContent::Node(n) => n,
		_ => return
	};	
	assert_eq!(ni.property, NodeProperty::All);
	
	let o = ContentInfoRequest::from_bytes(b"node spring"); 
	assert!(o.is_ok());
	let cir : ContentInfoRequest = o.unwrap();
	
	let info = cir.info;
	assert_match!(info, InfoContent::Node(_));
	let ni : ContentNodeProperty = match info {
		InfoContent::Node(n) => n,
		_ => return
	};	
	assert_eq!(ni.property, NodeProperty::All);
	
	let o = ContentInfoRequest::from_bytes(b"node spring hostname"); 
	assert!(o.is_ok());
	let cir : ContentInfoRequest = o.unwrap();
	
	let info = cir.info;
	assert_match!(info, InfoContent::Node(_));
	let ni : ContentNodeProperty = match info {
		InfoContent::Node(n) => n,
		_ => return
	};	
	assert_eq!(ni.property, NodeProperty::Hostname);
	
	let o = ContentInfoRequest::from_bytes(b"node spring address"); 
	assert!(o.is_ok());
	let cir : ContentInfoRequest = o.unwrap();
	
	let info = cir.info;
	assert_match!(info, InfoContent::Node(_));
	let ni : ContentNodeProperty = match info {
		InfoContent::Node(n) => n,
		_ => return
	};	
	assert_eq!(ni.property, NodeProperty::Address);
	
	let o = ContentInfoRequest::from_bytes(b"node spring service"); 
	assert!(o.is_ok());
	let cir : ContentInfoRequest = o.unwrap();
	
	let info = cir.info;
	assert_match!(info, InfoContent::Node(_));
	let ni : ContentNodeProperty = match info {
		InfoContent::Node(n) => n,
		_ => return
	};	
	assert_eq!(ni.property, NodeProperty::Service(None));
	
	let o = ContentInfoRequest::from_bytes(b"node spring state"); 
	assert!(o.is_ok());
	let cir : ContentInfoRequest = o.unwrap();
	
	let info = cir.info;
	assert_match!(info, InfoContent::Node(_));
	let ni : ContentNodeProperty = match info {
		InfoContent::Node(n) => n,
		_ => return
	};	
	assert_eq!(ni.property, NodeProperty::State(None));
	
	let o = ContentInfoRequest::from_bytes(b"node spring role"); 
	assert!(o.is_ok());
	let cir : ContentInfoRequest = o.unwrap();
	
	let info = cir.info;
	assert_match!(info, InfoContent::Node(_));
	let ni : ContentNodeProperty = match info {
		InfoContent::Node(n) => n,
		_ => return
	};	
	assert_eq!(ni.property, NodeProperty::Role(None));
}

#[test]
fn ts_content_info_request_node_from_bytes_fail() {
	let o = ContentInfoRequest::from_bytes(b"node spring sads");
	assert!(o.is_err());
}

#[test]
fn ts_content_info_request_node_property_to_string_pass () {
	let o = ContentInfoRequest::from_bytes(b"node spring all"); 
	assert!(o.is_ok());
	assert_eq!(format!("{}", o.unwrap()), "node spring all");
	
	let o = ContentInfoRequest::from_bytes(b"node spring hostname"); 
	assert!(o.is_ok());
	assert_eq!(format!("{}", o.unwrap()), "node spring hostname");
	
	let o = ContentInfoRequest::from_bytes(b"node spring address"); 
	assert!(o.is_ok());
	assert_eq!(format!("{}", o.unwrap()), "node spring address");
	
	let o = ContentInfoRequest::from_bytes(b"node spring service"); 
	assert!(o.is_ok());
	assert_eq!(format!("{}", o.unwrap()), "node spring service");
	
	let o = ContentInfoRequest::from_bytes(b"node spring state"); 
	assert!(o.is_ok());
	assert_eq!(format!("{}", o.unwrap()), "node spring state");
}


#[test]
fn ts_message_info_request_network_to_bytes () {
	let o = Message::from_bytes(b"info network");
	assert!(o.is_ok());
	let m : Message = o.unwrap();
	let r = String::from_utf8(m.to_bytes());
	assert!(r.is_ok());
	assert_eq!(r.unwrap(), "info network");
}

#[test]
fn ts_message_info_request_node_info_to_bytes () {
	let o = Message::from_bytes(b"info node spring all");
	assert!(o.is_ok());
	let m : Message = o.unwrap();
	let r = String::from_utf8(m.to_bytes());
	assert!(r.is_ok());
	assert_eq!(r.unwrap(), "info node spring all");
}

#[test]
fn ts_message_update_node_property_from_bytes_pass () {
	
	let o = Message::from_bytes(b"update spring state enabled"); 
	assert!(o.is_ok());
	let m : Message = o.unwrap();
	
	assert_eq!(m.cmd, CmdType::Update);
	let cnp : ContentNodeProperty = match m.content { 
		MessageContent::Update(p) => p,
		_ => return
	};
	assert_eq!(cnp.property, NodeProperty::State(Some(NodeState::Enabled)));
}

#[test]
fn ts_message_update_node_property_from_bytes_fail () {
	
	let o = Message::from_bytes(b"update spring blagh enabled"); 
	assert!(o.is_err());
	
	let o = Message::from_bytes(b"update spring state void"); 
	assert!(o.is_err());
}


#[test]
fn ts_message_update_node_property_value_to_bytes_pass () {
	
	let o = Message::from_bytes(b"update spring service http"); 
	assert!(o.is_ok());
	let m : Message = o.unwrap();
	
	let r = String::from_utf8(m.to_bytes());
	assert!(r.is_ok());
	assert_eq!(r.unwrap(), "update spring service http");
}

#[test]
fn ts_content_resolve_pass () {
	let r = ContentUri::from_bytes(b"spring://esusx.uk");
	assert!(r.is_ok());
	let cr : ContentUri = r.unwrap();
	
	assert_eq!(cr.uri.route().len(), 2);
	assert_eq!(cr.uri.gtn(), "uk");
}

#[test]
fn ts_content_resolve_fail () {
	let r = ContentUri::from_bytes(b"sprog://esusx.uk");
	assert!(r.is_err());
}

#[test]
fn ts_message_resolve_pass () {
	
	let o = Message::from_bytes(b"resolve spring://cci.esusx.uk"); 
	assert!(o.is_ok());
	let m : Message = o.unwrap();
	
	assert_match!(m.content, MessageContent::Resolve(_));
	let cr = msg_resolve!(m.content);
	assert_eq!(cr.uri.route().len(), 3);
	assert_eq!(cr.uri.gtn(), "uk");
}

#[test]
fn ts_message_resolve_fail () {
	
	let o = Message::from_bytes(b"resolve sprinddg://cci.esusx.uk"); 
	assert!(o.is_err());
}

#[test]
fn ts_message_service_pass () {
	
	let o = Message::from_bytes(b"service spring://cci.esusx.uk/service/"); 
	assert!(o.is_ok());
	let m : Message = o.unwrap();
	
	assert_match!(m.content, MessageContent::Service(_));
	let cs = msg_service!(m.content);
	assert_eq!(cs.uri.route().len(), 3);
	assert_eq!(cs.uri.gtn(), "uk");
	assert_eq!(cs.uri.res().len(), 1);
	assert_eq!(cs.uri.res()[0], "service");
}

#[test]
fn ts_message_service_fail () {
	
	let o = Message::from_bytes(b"service sprinddg://cci.esusx.uk/service/"); 
	assert!(o.is_err());
}