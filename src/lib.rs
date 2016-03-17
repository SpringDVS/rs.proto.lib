pub mod protocol;
pub mod serialise;

pub fn it_works() {
   	println!("SpringDVS");
   	let p = protocol::Packet::new(protocol::PacketType::GsnRegisterHost);
   	let d = p.ptype as u8;
   	println!("{}", d);
   	
}

