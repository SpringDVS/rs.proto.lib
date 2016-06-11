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
	
	let c = match m.content {
		MessageContent::Registration(s) => s,
		_ => return
	};
	
	assert_eq!(c.ndouble.spring, "foobar");
	assert_eq!(c.ndouble.host, "hostbar");
	
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
