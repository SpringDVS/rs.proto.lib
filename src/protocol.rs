/* Notice:	Copyright 2016, The Care Connections Initiative c.i.c.
 * Author: 	Charlie Fyvie-Gauld (cfg@zunautica.org)
 * License: GPLv3 (http://www.gnu.org/licenses/gpl-3.0.txt)
 */

use std::str;

pub use ::enums::{CmdType, ParseFailure};

struct Empty;

pub enum MessageContent {
	Empty,
	RegStr(ContentRegStr),
	
}

trait ProtocolObject : Sized {
	fn from_bytes(bytes: &[u8]) -> Result<Self, ParseFailure>;
	fn to_bytes(&self) -> Vec<u8>;
	
}

pub struct Message {
	pub cmd: CmdType,
	pub content: MessageContent,
}



impl Message {
	
	
	fn next(bytes: &[u8]) -> Result<(usize, &str), ParseFailure> {
		
		for i in 0 .. bytes.len() { 
			match bytes[i] {
				b' ' =>  return match str::from_utf8(&bytes[0..i]) {
							Err(_) => Err(ParseFailure::ConversionError),
							Ok(s) => Ok((i+1, s))
						},
				_ => { }
			}
		}

		match str::from_utf8(&bytes) {
			Err(_) => Err(ParseFailure::ConversionError),
			Ok(s) => Ok((bytes.len(), s))
		}	
	}
	
	fn parse_cmd(cmd: &str) -> Result<CmdType, ParseFailure> {
		match cmd {
			"reg" => Ok(CmdType::Register),
			"ureg" => Ok(CmdType::Unregister),
			_  => Err(ParseFailure::InvalidCommand)
		}
	}
	
	fn parse_content(bytes: &[u8], mtype: CmdType) -> Result<MessageContent, ParseFailure> {
		
		match mtype {
			CmdType::Register => Ok(MessageContent::RegStr(try!(ContentRegStr::from_bytes(&bytes)))),
			CmdType::Unregister => Ok(MessageContent::RegStr(try!(ContentRegStr::from_bytes(&bytes)))),
			_ => return Err(ParseFailure::InvalidCommand),
		}
		
	}
}

impl ProtocolObject for Message {
	fn from_bytes(bytes: &[u8]) -> Result<Self, ParseFailure> {

		let (index, cmd) = try!(Message::next(bytes));
		let mtype = try!(Message::parse_cmd(cmd));
		let content = try!(Message::parse_content(&bytes[index..], mtype));
		Ok(Message{
				cmd: mtype,
				content: content
			})
	}

	fn to_bytes(&self) -> Vec<u8> {
		Vec::new()
	}
	
}


pub struct ContentRegStr {
	pub regstr: String, 
}

impl ContentRegStr {
	
}

impl ProtocolObject for ContentRegStr {
	fn from_bytes(bytes: &[u8]) -> Result<Self, ParseFailure> {
		
		if bytes.len() == 0 { return Err(ParseFailure::InvalidContentFormat) }
		
		let s = match str::from_utf8(bytes) {
			Ok(s) => s,
			Err(_) => return Err(ParseFailure::ConversionError)
		};
		
		Ok( ContentRegStr{ 
				regstr: String::from(s) 
			} )
	}

	fn to_bytes(&self) -> Vec<u8> {
		Vec::new()
	}	
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
	let o = Message::from_bytes(b"reg foobar");
	assert!(o.is_ok());
	let m : Message = o.unwrap();
	assert_eq!(m.cmd, CmdType::Register);
	
	assert!( match m.content {
			MessageContent::RegStr(_) => true,
			_ => false,
	});
	
	let c = match m.content {
		MessageContent::RegStr(s) => s,
		_ => return
	};
	
	assert_eq!(c.regstr, "foobar");
	
}

#[test]
fn ts_from_bytes_reg_fail() {
	let o = Message::from_bytes(b"reg");
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
			MessageContent::RegStr(_) => true,
			_ => false,
	});
	
	let c = match m.content {
		MessageContent::RegStr(s) => s,
		_ => return
	};
	
	assert_eq!(c.regstr, "foobar");
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
