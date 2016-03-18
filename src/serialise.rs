use std::mem::transmute;

pub trait NetSerial : Sized {
	fn serialise(&self) -> Vec<u8>;
	fn deserialise(bytes: &[u8]) -> Option<Self>;
}

pub fn push_bytes(v: &mut Vec<u8>, bytes: &[u8]) {
	for b in bytes {
		v.push(*b)
	}
}

pub fn u32_transmute_le_arr(a: &[u8]) -> u32 {
	unsafe { transmute::<[u8;4], u32>([a[3], a[2], a[1], a[0]]) } 
}

pub fn u32_transmute_be_arr(a: &[u8]) -> u32 {
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