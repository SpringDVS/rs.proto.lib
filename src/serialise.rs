pub trait NetSerial : Sized {
	fn serialise(&self) -> Vec<u8>;
	fn deserialise(bytes: &[u8]) -> Option<Self>;
}

pub fn push_bytes(v: &mut Vec<u8>, bytes: &[u8]) {
	for b in bytes {
		v.push(*b)
	}
}