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
	let bytes : [u8;14] = [1,0, 33,0,0,0, 127,0,0,1, 192,168,0,255];
	let op = Packet::deserialise(&bytes);
	
	assert!(op.is_ok());
	
	let p = op.unwrap();
	
	assert_eq!(DvspMsgType::GsnRegistration, p.header().msg_type);
	assert_eq!(false, p.header().msg_part);
	assert_eq!(33, p.header().msg_size);
	assert_eq!([192,168,0,255], p.header().addr_dest);
}

#[test]
fn ts_protocol_packet_deserialise_f() {
	// Test fail - Invalid type
	let bytes : [u8;14] = [128,0, 33,0,0,0, 127,0,0,1, 192,168,0,255];
	let r = Packet::deserialise(&bytes);
	
	assert!(r.is_err());
	assert_eq!(Failure::InvalidBytes, r.unwrap_err());
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
	
	assert!(op.is_ok());
	assert_eq!(DvspRcode::Ok, op.unwrap().code);
}






#[test]
fn ts_protocol_frame_state_update_serialise_p() {
	// Test pass
	let fr = FrameStateUpdate::new(DvspNodeState::Enabled, "springtime");
	let bytes = fr.serialise();
	
	assert_eq!(1, bytes[0]);
	assert!( { bytes.len() == 11 } );
	assert_eq!('s' as u8, bytes[1]);
	assert_eq!('e' as u8, bytes[10]);
	
	// Test pass
	let fr = FrameStateUpdate::new(DvspNodeState::Enabled, "");
	let bytes = fr.serialise();
	
	assert_eq!(1, bytes[0]);
	assert!( { bytes.len() == 1 } );
}

#[test]
fn ts_protocol_frame_state_update_deserialise_p() {
	// Test pass
	let bytes = [1];
	let op = FrameStateUpdate::deserialise(&bytes);
	
	assert!(op.is_ok());
	
	let frame = op.unwrap();
	assert_eq!(DvspNodeState::Enabled, frame.status);
	assert_eq!("", frame.springname);
	
	let bytes2 = [1,'f' as u8, 'o' as u8, 'o' as u8];
	
	let op = FrameStateUpdate::deserialise(&bytes2);
	
	assert!(op.is_ok());
	
	let frame2 = op.unwrap();
	assert_eq!(DvspNodeState::Enabled, frame2.status);
	assert_eq!("foo", frame2.springname);
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
fn ts_protocol_frame_node_status_deserialise_f() {
	// Test fail
	
	// Invalid rcode
	let mut bytes = [0,200,0,0, 2];
	let op1 = FrameNodeStatus::deserialise(&bytes);
	
	assert!(op1.is_err());
	assert_eq!(Failure::InvalidBytes, op1.unwrap_err());
	
	// invalid node state
	bytes[0] = 200;
	bytes[1] = 0;
	bytes[4] = 5;	

	let op2 = FrameNodeStatus::deserialise(&bytes);
	
	assert!(op2.is_err());
	assert_eq!(Failure::InvalidBytes, op2.unwrap_err());
	
}





#[test]
fn ts_protocol_frame_network_serialise_p() {
	// Test pass
	let fr = FrameNetwork::new("foobar");
	let bytes = fr.serialise();
	
	assert_eq!('f' as u8, bytes[0]);
	assert_eq!('o' as u8, bytes[1]);
	assert_eq!('o' as u8, bytes[2]);
	assert_eq!('b' as u8, bytes[3]);
	assert_eq!('a' as u8, bytes[4]);
	assert_eq!('r' as u8, bytes[5]);
}

#[test]
fn ts_protocol_frame_network_deserialise_p() {
	// Test pass
	let bytes = vec!['f' as u8,'o' as u8,'o' as u8,'b' as u8,'a' as u8,'r' as u8];
	let r = FrameNetwork::deserialise(&bytes);
	
	assert!(r.is_ok());
	assert_eq!("foobar", String::from_utf8(r.unwrap().list).unwrap());
}


#[test]
fn ts_protocol_frame_node_request_serialise_p() {
	// Test pass
	let fr = FrameNodeRequest::new("foobar");
	let bytes = fr.serialise();
	
	assert_eq!('f' as u8, bytes[0]);
	assert_eq!('o' as u8, bytes[1]);
	assert_eq!('o' as u8, bytes[2]);
	assert_eq!('b' as u8, bytes[3]);
	assert_eq!('a' as u8, bytes[4]);
	assert_eq!('r' as u8, bytes[5]);
}

#[test]
fn ts_protocol_frame_node_request_deserialise_p() {
	// Test pass
	let bytes = vec!['f' as u8,'o' as u8,'o' as u8,'b' as u8,'a' as u8,'r' as u8];
	let r = FrameNodeRequest::deserialise(&bytes);
	
	assert!(r.is_ok());
	assert_eq!("foobar", String::from_utf8(r.unwrap().shi).unwrap());
}



#[test]
fn ts_protocol_frame_node_info_serialise_p() {
	
	// Test pass
	let frame = FrameNodeInfo::new(DvspNodeType::Root as u8, DvspService::Http, [127,0,0,1], "foobar");
	let bytes = frame.serialise();
	
	assert_eq!([200,0,0,0], bytes[0..4]);
	assert_eq!(DvspNodeType::Root as u8, bytes[4]);
	assert_eq!(DvspService::Http as u8, bytes[5]);
	assert_eq!([127,0,0,1], bytes[6..10]);
	assert!(bytes.len() > 10);
	assert_eq!('f' as u8, bytes[10]);
	assert_eq!('r' as u8, bytes[15]);
	
	let frame2 = FrameNodeInfo::new(DvspNodeType::Root as u8, DvspService::Http, [127,0,0,1], "");
	let bytes2 = frame2.serialise();
	
	assert!(bytes2.len() == 10);
}

#[test]
fn ts_protocol_frame_node_info_deserialise_p() {
	
	// Test pass
	let mut frame = FrameNodeInfo::new(DvspNodeType::Root as u8, DvspService::Http, [127,0,0,1], "foobar");
	let bytes = frame.serialise();
	
	let r = FrameNodeInfo::deserialise(&bytes);
	
	assert!(r.is_ok());
	
	let checker = r.unwrap();
	
	assert_eq!(frame.code, checker.code);
	assert_eq!(frame.ntype, checker.ntype);
	assert_eq!(frame.service, checker.service);
	assert_eq!(frame.address, checker.address);
	assert_eq!(frame.name, checker.name);
	
	frame.name = String::from("");
	let bytes2 = frame.serialise();
	let r2 = FrameNodeInfo::deserialise(&bytes2);
	
	assert!(r2.is_ok());
	
	let checker2 = r2.unwrap();
	assert_eq!(frame.name, checker2.name);
}

#[test]
fn ts_protocol_frame_node_info_deserialise_f() {
	// Test Fail
	
	// Invalid response code
	let bytes = [0,200,0,0, 1, 1, 127,0,0,1, 'f' as u8];
	let r = FrameNodeInfo::deserialise(&bytes);
	
	assert!(r.is_err());
	assert_eq!(Failure::InvalidBytes, r.unwrap_err());
	
	// Invalid node type
	let bytes2 = [200,0,0,0, 125, 1,  127,0,0,1, 'f' as u8];
	let r2 = FrameNodeInfo::deserialise(&bytes2);
	
	assert!(r2.is_err());
	assert_eq!(Failure::InvalidBytes, r2.unwrap_err());
	
	// Invalid service type
	let bytes = [200,0,0,0, 1, 125, 127,0,0,1, 'f' as u8];
	let r = FrameNodeInfo::deserialise(&bytes);
	
	assert!(r.is_err());
	assert_eq!(Failure::InvalidBytes, r.unwrap_err());
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
	
	let fr2 = FrameRegister::new(
		true,
		DvspNodeType::Org as u8, 
		DvspService::Http, 
		String::from("")
	);
	
	let bytes2 = fr2.serialise();
	assert!(bytes2.len() == FrameRegister::lower_bound());
	
}

#[test]
fn ts_protocol_frame_register_deserialise_p() {
	// Test pass
	let bytes : [u8;7] = [1,2,3,1, 'a' as u8,'b' as u8,'c' as u8];
	let op = FrameRegister::deserialise(&bytes);
	
	assert!(op.is_ok());
	
	let frame = op.unwrap();
	
	assert_eq!(true, frame.register);
	assert_eq!(2, frame.ntype);
	assert_eq!(3, frame.len);
	assert_eq!(DvspService::Dvsp, frame.service);
	assert_eq!(String::from("abc"), frame.nodereg);
	
	let bytes2 : [u8;4] = [1,2,3,1];
	let op2 = FrameRegister::deserialise(&bytes2);
	
	assert!(op2.is_ok());
	
	let frame2 = op2.unwrap();
	assert_eq!(String::from(""), frame2.nodereg);
}

#[test]
fn ts_protocol_frame_register_deserialise_f() {
	// Test fail
	
	// Invalid node type
	let mut bytes : [u8;7] = [1, Bounds::MaxNodeType as u8 + 1 ,3,1, 'a' as u8,'b' as u8,'c' as u8];
	let op1 = FrameRegister::deserialise(&bytes);
	assert!(op1.is_err());
	assert_eq!(Failure::InvalidBytes, op1.unwrap_err());
	
	// Invalid node service
	bytes[1] = 2;
	bytes[3] = 100;
	let op2 = FrameRegister::deserialise(&bytes);
	assert!(op2.is_err());
	assert_eq!(Failure::InvalidBytes, op2.unwrap_err());
	
	// Invalid nodereg len
	bytes[1] = 0;
	bytes[3] = 0;
	bytes[2] = Bounds::FrameRegisterLen as u8 + 1;
	let op3 = FrameRegister::deserialise(&bytes);
	assert!(op3.is_err());
	assert_eq!(Failure::OutOfBounds, op3.unwrap_err());
}



#[test]
fn ts_protocol_frame_type_request_serialise_p() {
	let frame = FrameTypeRequest::new(DvspNodeType::Org as u8);
	let bytes = frame.serialise();
	
	assert_eq!(DvspNodeType::Org as u8, bytes[0]);
}

#[test]
fn ts_protocol_frame_type_request_deserialise_p() {
	let f = FrameTypeRequest::new(DvspNodeType::Org as u8);
	let serial = f.serialise();
	
	let op = FrameTypeRequest::deserialise(&serial);
	assert!(op.is_ok());
	
	assert_eq!(DvspNodeType::Org as u8, op.unwrap().ntype);
}

#[test]
fn ts_protocol_frame_type_request_deserialise_f() {

	let serial: [u8;1] = [101];
	
	let op = FrameTypeRequest::deserialise(&serial);
	assert!(op.is_err());
}


#[test]
fn ts_protocol_frame_unit_test_serialise_p() {
	let frame = FrameUnitTest::new(UnitTestAction::Reset,"foobar");
	let bytes = frame.serialise();
	
	assert_eq!(UnitTestAction::Reset as u8, bytes[0]);
	assert_eq!('f' as u8, bytes[1]);
	assert_eq!('r' as u8, bytes[6]);
}

#[test]
fn ts_protocol_frame_unit_test_serialise_zero_extra_p() {
	let frame = FrameUnitTest::new(UnitTestAction::Reset,"");
	let bytes = frame.serialise();
	
	assert_eq!(UnitTestAction::Reset as u8, bytes[0]);
	assert_eq!(FrameUnitTest::lower_bound(), bytes.len()) 
}


#[test]
fn ts_protocol_frame_unit_test_deserialise_p() {
	let f = FrameUnitTest::new(UnitTestAction::Reset,"foo");
	let serial = f.serialise();
	
	let op = FrameUnitTest::deserialise(&serial);
	assert!(op.is_ok());
	let frame = op.unwrap();
	assert_eq!(UnitTestAction::Reset, frame.action);
	assert_eq!("foo", frame.extra);
}

#[test]
fn ts_protocol_frame_unit_test_deserialise_f() {

	let serial: [u8;1] = [101];
	
	let op = FrameUnitTest::deserialise(&serial);
	assert!(op.is_err());
}




#[test]
fn ts_protocol_frame_resolution_serialise_p() {
	let frame = FrameResolution::new("spring://cci.esusx.uk");
	let bytes = frame.serialise();

	assert_eq!('s' as u8, bytes[0]);
	assert_eq!('k' as u8, bytes[20]);
}

#[test]
fn ts_protocol_frame_resolution_serialise_zero_length_p() {
	let frame = FrameResolution::new("");
	let bytes = frame.serialise();

	assert_eq!(0, bytes.len());
}

#[test]
fn ts_protocol_frame_resolution_deserialise_p() {
	let f = FrameResolution::new("spring://cci.esusx.uk");
	let serial = f.serialise();
	
	let op = FrameResolution::deserialise(&serial);
	assert!(op.is_ok());
	let frame = op.unwrap();
	assert_eq!("spring://cci.esusx.uk", frame.url);
}

#[test]
fn ts_protocol_frame_resolution_deserialise_zero_length_p() {
	let f = FrameResolution::new("");
	let serial = f.serialise();
	
	let op = FrameResolution::deserialise(&serial);
	assert!(op.is_ok());
	let frame = op.unwrap();
	assert_eq!("", frame.url);
}


#[test]
fn ts_protocol_frame_geosub_serialise_p() {
	let frame = FrameGeosub::new("esusx");
	let bytes = frame.serialise();

	assert_eq!(5, bytes.len());
	assert_eq!('e' as u8, bytes[0]);
	assert_eq!('x' as u8, bytes[4]);
}

#[test]
fn ts_protocol_frame_geosub_deserialise_p() {
	let f = FrameGeosub::new("esusx");
	let serial = f.serialise();
	
	let op = FrameGeosub::deserialise(&serial);
	assert!(op.is_ok());
	let frame = op.unwrap();
	assert_eq!("esusx", frame.gsn);
}



#[test]
fn ts_protocol_packet_write_content_p() {
	let mut p = Packet::new(DvspMsgType::Undefined);
	let bytes : [u8;500] = [0;500];
	let r = p.write_content(&bytes);
	assert!(r.is_ok());
}

#[test]
fn ts_protocol_packet_write_content_f() {
	let mut p = Packet::new(DvspMsgType::Undefined);
	let bytes : [u8;513] = [0;513];
	let r = p.write_content(&bytes);
	assert!(r.is_err());
	assert_eq!(Failure::OutOfBounds, r.unwrap_err());
}



#[test]
fn ts_protocol_frame_register_gtn_serialise_p() {
	// Test pass
	let fr = FrameRegisterGtn::new(
		true,
		DvspService::Http, 
		String::from("abc")
	);
	
	let bytes = fr.serialise();
	
	assert_eq!(1, bytes[0]); // register
	assert_eq!(2, bytes[1]); // service
	assert_eq!(3, bytes[2]); // len
	
	
	assert_eq!('a' as u8, bytes[3]);
	assert_eq!('b' as u8, bytes[4]);
	assert_eq!('c' as u8, bytes[5]);
	
	let fr2 = FrameRegister::new(
		true,
		DvspNodeType::Org as u8, 
		DvspService::Http, 
		String::from("")
	);
	
	let bytes2 = fr2.serialise();
	assert!(bytes2.len() == FrameRegister::lower_bound());
	
}

#[test]
fn ts_protocol_frame_register_gtn_deserialise_p() {
	// Test pass
	let fr = FrameRegisterGtn::new(
		true,
		DvspService::Dvsp, 
		String::from("abc")
	);
	
	let bytes = fr.serialise();
	let op = FrameRegisterGtn::deserialise(&bytes);
	
	assert!(op.is_ok());
	
	let frame = op.unwrap();
	
	assert_eq!(true, frame.register);
	assert_eq!(DvspService::Dvsp, frame.service);
	assert_eq!(3, frame.len);
	assert_eq!(String::from("abc"), frame.nodereg);

}

#[test]
fn ts_protocol_frame_register_gtn_deserialise_f() {
	// Test fail
	
	// Invalid node type
	// Test pass
	let fr = FrameRegisterGtn::new(
		true,
		DvspService::Http, 
		String::from("abc")
	);
	
	let mut bytes = fr.serialise();
	
	// Invalid node service
	bytes[1] = 101;
	let op1 = FrameRegisterGtn::deserialise(&bytes);
	assert!(op1.is_err());
	assert_eq!(Failure::InvalidBytes, op1.unwrap_err());
	
	
	// Invalid nodereg len
	bytes[1] = 1;
	bytes[3] = 0;
	bytes[2] = Bounds::FrameRegisterLen as u8 + 1;
	let op3 = FrameRegister::deserialise(&bytes);
	assert!(op3.is_err());
	assert_eq!(Failure::OutOfBounds, op3.unwrap_err());
}









#[test]
fn ts_protocol_packet_content_as_p() {
	let mut p = Packet::new(DvspMsgType::Undefined);
	let fr = FrameResponse::new(DvspRcode::Ok);
	
	let r = p.write_content(fr.serialise().as_slice());
	
	assert!(r.is_ok());
	
	let op = p.content_as::<FrameResponse>();
	
	assert!(op.is_ok());
	
	let frame = op.unwrap();
	
	assert_eq!(DvspRcode::Ok, frame.code);
}

#[test]
fn ts_protocol_packet_content_as_f() {
	let mut p = Packet::new(DvspMsgType::Undefined);
	
	// Out of lower bounds
	let bytes : [u8;2] = [0;2];
	assert!(p.write_content(&bytes).is_ok());
	
	let r = p.content_as::<FrameResponse>();
	
	assert!(r.is_err());
	assert_eq!(Failure::OutOfBounds, r.unwrap_err())
}

#[test]
fn ts_http_to_bin_upper_p() {
	let r = http_to_bin("466F6F626172");
	assert!(r.is_ok());
	let s = String::from_utf8(r.unwrap()).unwrap();
	assert_eq!(String::from("Foobar"), s);
}

#[test]
fn ts_http_to_bin_lower_p() {
	let r = http_to_bin("466f6f626172");
	assert!(r.is_ok());
	let s = String::from_utf8(r.unwrap()).unwrap();
	assert_eq!(String::from("Foobar"), s);
}

#[test]
fn ts_http_to_bin_upperlower_p() {
	let r = http_to_bin("466f6F626172");
	assert!(r.is_ok());
	let s = String::from_utf8(r.unwrap()).unwrap();
	assert_eq!(String::from("Foobar"), s);
}

#[test]
fn ts_http_to_bin_out_of_bounds_f() {
	let r = http_to_bin("466f6g626172");
	assert!(r.is_err());
}

#[test]
fn ts_http_to_bin_invalid_len_f() {
	let r = http_to_bin("466f6f62617");
	assert!(r.is_err());
}