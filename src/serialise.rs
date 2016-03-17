trait NetSerial {
	fn serialise() -> [u8];
	fn deserialise(bytes: [u8]) -> Self;
}