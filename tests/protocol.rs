extern crate spring_dvs;

use spring_dvs::protocol::*;

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
	let o = Message::from_bytes(b"reg foobar,hostbar;org;a");
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
