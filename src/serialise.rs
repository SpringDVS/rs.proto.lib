/* Notice:	Copyright 2016, The Care Connections Initiative c.i.c.
 * Author: 	Charlie Fyvie-Gauld (cfg@zunautica.org)
 * License: GPLv3 (http://www.gnu.org/licenses/gpl-3.0.txt)
 */
use std::mem::transmute;
use ::enums::Failure;

pub trait NetSerial : Sized {
	fn serialise(&self) -> Vec<u8>;
	fn deserialise(bytes: &[u8]) -> Result<Self, Failure>;
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