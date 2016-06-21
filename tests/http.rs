extern crate spring_dvs;

use std::str::FromStr;
use std::net::SocketAddr;

use spring_dvs::enums::Failure;
use spring_dvs::protocol::{ProtocolObject,Message,MessageContent,CmdType,ContentInfoRequest,InfoContent,ContentNodeProperty};
use spring_dvs::http::HttpWrapper;


#[test]
fn ts_http_deserialise_http_request_pass() {
	let req :String = "
POST /spring/ HTTP/1.1\r
Host: {}\r
User-Agent: SpringDVS\r
Content-Type: text/plain\r
Content-Length: {}\r\n\r
info node foo state
".to_string();
	let mut v: Vec<u8> = Vec::new();
	v.extend_from_slice(req.as_bytes()); 
	let r = HttpWrapper::deserialise_request(v, &mut SocketAddr::from_str(&format!("127.0.0.1:80")).unwrap());
	assert!(r.is_ok());
	let msg = r.unwrap();
	assert_eq!(msg.cmd, CmdType::Info);
}


#[test]
fn ts_http_deserialise_http_response_pass() {
	let req :String = "
POST /spring/ HTTP/1.1\r
Host: {}\r
User-Agent: SpringDVS\r
Content-Type: text/plain\r
Content-Length: {}\r\n\r
200
".to_string();
	let mut v: Vec<u8> = Vec::new();
	v.extend_from_slice(req.as_bytes()); 
	let r = HttpWrapper::deserialise_response(v);
	assert!(r.is_ok());
	let msg = r.unwrap();
	assert_eq!(msg.cmd, CmdType::Response);
}

#[test]
fn ts_http_deserialise_tcp_response_pass() {
	let req :String = "
200
".to_string();
	let mut v: Vec<u8> = Vec::new();
	v.extend_from_slice(req.as_bytes()); 
	let r = HttpWrapper::deserialise_response(v);
	assert!(r.is_ok());
	let msg = r.unwrap();
	assert_eq!(msg.cmd, CmdType::Response);
}


#[test]
fn ts_http_serialise_http_request_pass() {
	let chk = "POST /spring/ HTTP/1.1\r
Host: foo.bar\r
User-Agent: SpringDVS\r
Content-Type: text/plain\r
Content-Length: 18\r\n\r
info node foo role";

	let bytes = HttpWrapper::serialise_request(&Message::from_bytes(b"info node foo role").unwrap(), "foo.bar");
	let r = String::from_utf8(bytes);
	assert!(r.is_ok());
	
	let s = r.unwrap();
	assert_eq!(s, chk);
}

#[test]
fn ts_http_serialise_http_response_pass() {
	let chk = "HTTP/1.1 200 OK\r
Server: SpringDVS/0.1\r
Content-Type: text/plain\r
Connection: Closed\r
Content-Length: 3\r\n\r
200";

	let bytes = HttpWrapper::serialise_response(&Message::from_bytes(b"200").unwrap());
	let r = String::from_utf8(bytes);
	assert!(r.is_ok());
	
	let s = r.unwrap();
	assert_eq!(s, chk);
}