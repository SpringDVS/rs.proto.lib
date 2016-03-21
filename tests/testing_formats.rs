/* Notice:  Copyright 2016, The Care Connections Initiative c.i.c.
 * Author:  Charlie Fyvie-Gauld (cfg@zunautica.org)
 * License: GPLv3 (http://www.gnu.org/licenses/gpl-3.0.txt)
 */
extern crate spring_dvs;
use spring_dvs::formats::*;
use spring_dvs::enums::Failure;
#[test]
fn ts_formats_str_address_to_ipv4_p() {
	
	// Test pass
	let straddr = "192.168.1.2";
	let r = str_address_to_ipv4(straddr);
	
	assert!(r.is_ok());
	
	assert_eq!([192,168,1,2], r.unwrap())
}

#[test]
fn ts_formats_str_address_to_ipv4_f() {
	
	// Test fail
	
	// Invalid format
	let straddr1 = "192.168.1";
	let r1 = str_address_to_ipv4(straddr1);
	assert!(r1.is_err());
	assert_eq!(Failure::InvalidFormat, r1.unwrap_err());
	
	// Invalid bytes
	let straddr2 = "192.168.1.384";
	let r2 = str_address_to_ipv4(straddr2);
	assert!(r2.is_err());
	assert_eq!(Failure::InvalidBytes, r2.unwrap_err());
}

#[test]
fn ts_formats_ipv4_to_str_address_p() {
	
	let addr = ipv4_to_str_address([192,168,1,2]);
	
	assert_eq!("192.168.1.2", addr);
}

