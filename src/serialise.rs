/* Notice:	Copyright 2016, The Care Connections Initiative c.i.c.
 * Author: 	Charlie Fyvie-Gauld (cfg@zunautica.org)
 * License: GPLv3 (http://www.gnu.org/licenses/gpl-3.0.txt)
 */
use std::mem::transmute;
use ::enums::Failure;

pub trait NetSerial : Sized {
	fn serialise(&self) -> Vec<u8>;
	fn deserialise(bytes: &[u8]) -> Result<Self, Failure>;
	fn lower_bound() -> usize;
}

pub fn push_bytes(v: &mut Vec<u8>, bytes: &[u8]) {
	for b in bytes {
		v.push(*b)
	}
}

pub fn u32_transmute_be_arr(a: &[u8]) -> u32 {
	unsafe { transmute::<[u8;4], u32>([a[3], a[2], a[1], a[0]]) } 
}

pub fn u32_transmute_le_arr(a: &[u8]) -> u32 {
	unsafe { transmute::<[u8;4], u32>([a[0], a[1], a[2], a[3]]) } 
}

pub fn array_transmute_be_u32(d: u32) -> [u8;4] {
	unsafe { transmute(d.to_be()) }
}

pub fn array_transmute_le_u32(d: u32) -> [u8;4] {
	unsafe { transmute(d.to_le()) }
}


pub fn byte_slice_4array(a: &[u8]) -> [u8;4] {
	[ a[0], a[1], a[2], a[3] ]
}

pub fn deserialise_bool(byte: u8) -> bool {
	match byte {
		0 => false,
		_ => true
	}
}

pub fn hex_str_to_byte(src: &[u8]) -> Option<u8> {
	
	let mut val = 0;
	let mut factor = 16;
	for i in 0 .. 2 {
		val +=  match src[i] {
			v @	48 ... 57 => (v - 48) * factor,
			v @ 65 ... 70 => (v - 55) * factor,
			v @ 97 ... 102 => (v - 87) * factor,
			_ => return None,   
		};
		 factor >>= 4;
	}

	Some(val)
	
}

pub fn bin_to_hex(src: &Vec<u8>) -> String {
	let mut s = String::new();
	for byte in src {
		s.push_str( format!("{:0>2x}", byte).as_ref() )
	}
	s
}