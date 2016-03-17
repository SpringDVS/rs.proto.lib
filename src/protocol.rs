pub enum PacketType {
	Undefined,
	GsnUnregisterHost,
	GsnRegisterHost,
}

pub struct Packet {
	pub ptype: PacketType
}

impl Packet {
	pub fn new(t: PacketType) -> Packet {
		Packet { ptype: t  }
	} 
}

impl ::serialise::NetSerial for Packet {
	fn serialise() -> [u8] {
		
	};
	
	fn deserialise(bytes: [u8]) -> Packet {
		
	};
}