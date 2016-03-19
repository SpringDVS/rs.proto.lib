/* Notice:	Copyright 2016, The Care Connections Initiative c.i.c.
 * Author: 	Charlie Fyvie-Gauld (cfg@zunautica.org)
 * License: GPLv3 (http://www.gnu.org/licenses/gpl-3.0.txt)
 */
extern crate spring_dvs;
use spring_dvs::protocol::*;
use spring_dvs::serialise::*;
use spring_dvs::enums::*;

// ------- Testing  -------- \\

#[test]
fn ts_protocol_packet_serialise_s() {
	let mut p: Packet = Packet::new(DvspMsgType::GsnRegistration);
	
	{
		let mut h = p.mut_header();
		h.msg_part = true;
		h.msg_size = 101;
		h.addr_orig = [192,168,1,1];
		h.addr_dest = [192,168,1,2];
	}
	
	let serial = p.serialise();
	
	assert_eq!(1, serial[0]);	// type
	assert_eq!(1, serial[1]);	// part
	
	assert_eq!(101, serial[2]);	// uint32
	assert_eq!(0, serial[3]);
	assert_eq!(0, serial[4]);
	assert_eq!(0, serial[5]);
	
	assert_eq!([192,168,1,1], byte_slice_4array(&serial[6..10]));

	assert_eq!([192,168,1,2], byte_slice_4array(&serial[10..14]));
	
}

#[test]
fn ts_protocol_packet_deserialise_p() {
	// Test good
	let bytes : [u8;14] = [1,0, 0,0,0,33, 127,0,0,1, 192,168,0,255];
	let op = Packet::deserialise(&bytes);
	
	assert!(op.is_some());
	
	let p = op.unwrap();
	
	assert_eq!(DvspMsgType::GsnRegistration, p.header().msg_type);
	assert_eq!(false, p.header().msg_part);
	assert_eq!(33, p.header().msg_size);
	assert_eq!([192,168,0,255], p.header().addr_dest);
}

#[test]
fn ts_protocol_packet_deserialise_f() {
	// Test bad
	let bytes : [u8;14] = [128,0, 0,0,0,33, 127,0,0,1, 192,168,0,255];
	let op = Packet::deserialise(&bytes);
	
	assert!(op.is_none());
}

#[test]
fn ts_protocol_frame_response_serialise_p() {
	// Test pass
	let fr = FrameResponse::new(DvspRcode::Ok);
	let bytes = fr.serialise();
	
	assert_eq!([200,0,0,0], byte_slice_4array(&bytes));
}

#[test]
fn ts_protocol_frame_response_deserialise_p() {
	// Test pass
	let bytes = [200,0,0,0];
	let op = FrameResponse::deserialise(&bytes);
	
	assert!(op.is_some());
	
	let frame = op.unwrap();
	
	assert_eq!(DvspRcode::Ok, frame.code);
}

#[test]
fn ts_protocol_frame_response_deserialise_f() {
	// Test fail
	let bytes = [0,200,0,0];
	let op = FrameResponse::deserialise(&bytes);
	
	assert!(op.is_none());
}

#[test]
fn ts_protocol_frame_node_status_serialise_p() {
	// Test pass
	let fr = FrameNodeStatus::new(DvspNodeState::Enabled);
	let bytes = fr.serialise();
	
	assert_eq!([200,0,0,0], byte_slice_4array(&bytes[0..4]));
	assert_eq!(1, bytes[4]);
}

#[test]
fn ts_protocol_frame_response_deserialis_p() {
	// Test pass
	let bytes = [200,0,0,0, 2];
	let op = FrameNodeStatus::deserialise(&bytes);
	
	assert!(op.is_some());
	
	let frame = op.unwrap();
	
	assert_eq!(DvspRcode::Ok, frame.code);
	assert_eq!(DvspNodeState::Unresponsive, frame.status);
}

#[test]
fn ts_protocol_frame_node_status_deserialise_f() {
	// Test fail
	
	// Invalid rcode
	let mut bytes = [0,200,0,0, 2];
	let op1 = FrameNodeStatus::deserialise(&bytes);
	
	assert!(op1.is_none());
	
	bytes[0] = 200;
	bytes[1] = 0;
	bytes[4] = 5;	

	let op2 = FrameNodeStatus::deserialise(&bytes);
	
	assert!(op2.is_none());
	
}

#[test]
fn ts_protocol_frame_register_serialise_p() {
	// Test pass
	let fr = FrameRegister::new(
		true,
		DvspNodeType::Org as u8, 
		DvspService::Http, 
		String::from("abc")
	);
	
	let bytes = fr.serialise();
	
	assert_eq!(1, bytes[0]); // register
	assert_eq!(2, bytes[1]); // type
	assert_eq!(3, bytes[2]); // len
	assert_eq!(2, bytes[3]); // service
	
	assert_eq!('a' as u8, bytes[4]);
	assert_eq!('b' as u8, bytes[5]);
	assert_eq!('c' as u8, bytes[6]);
}

#[test]
fn ts_protocol_frame_register_deserialise_p() {
	// Test pass
	let bytes : [u8;7] = [1,2,3,1, 'a' as u8,'b' as u8,'c' as u8];
	let op = FrameRegister::deserialise(&bytes);
	
	assert!(op.is_some());
	
	let frame = op.unwrap();
	
	assert_eq!(true, frame.register);
	assert_eq!(2, frame.ntype);
	assert_eq!(3, frame.len);
	assert_eq!(DvspService::Dvsp, frame.service);
	assert_eq!(String::from("abc"), frame.nodereg);
}

#[test]
fn ts_protocol_frame_register_deserialise_f() {
	// Test fail
	
	// Invalid node type
	let mut bytes : [u8;7] = [1, Bounds::MaxNodeType as u8 + 1 ,3,1, 'a' as u8,'b' as u8,'c' as u8];
	let op1 = FrameRegister::deserialise(&bytes);
	assert!(op1.is_none());
	
	// Invalid node service
	bytes[1] = 2;
	bytes[3] = 100;
	let op2 = FrameRegister::deserialise(&bytes);
	assert!(op2.is_none());
	
	
	// Invalid nodereg len
	bytes[1] = 2;
	bytes[3] = 1;
	bytes[2] = Bounds::FrameRegisterLen as u8 + 1;
	let op3 = FrameRegister::deserialise(&bytes);
	assert!(op3.is_none());
}

#[test]
fn ts_protocol_packet_content_as() {
	let mut p = Packet::new(DvspMsgType::Undefined);
	let fr = FrameResponse::new(DvspRcode::Ok);
	
	p.write_content(fr.serialise().as_slice());
	
	let op = p.content_as::<FrameResponse>();
	
	assert!(op.is_some());
	
	let frame = op.unwrap();
	
	assert_eq!(DvspRcode::Ok, frame.code);
}
