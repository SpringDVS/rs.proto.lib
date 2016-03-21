/* Notice:  Copyright 2016, The Care Connections Initiative c.i.c.
 * Author:  Charlie Fyvie-Gauld (cfg@zunautica.org)
 * License: GPLv3 (http://www.gnu.org/licenses/gpl-3.0.txt)
 */

use ::protocol::Ipv4;
use ::enums::Failure;

pub fn str_address_to_ipv4(address: &str) -> Result<Ipv4, Failure> {
	let atom: Vec<&str> = address.split('.').collect();
	
	if atom.len() != 4 {
		return Err(Failure::InvalidFormat);
	};
		
	let mut addr: Ipv4 = [0;4];
	
	for i in 0..4 {
		
		addr[i] = match atom[i].parse::<u32>().unwrap() {
			v if v < 0xFF  => v,
			_ => return Err(Failure::InvalidBytes)
		} as u8;
	}

	Ok(addr)
}