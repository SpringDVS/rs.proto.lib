use std::str;
use std::collections::HashMap;
pub use ::enums::Failure;
pub use ::node::Node;
#[derive(Debug)]
pub struct Uri {
	
	gsn: Vec<String>,
	gtn: String,
	res: Vec<String>,
	query: String,
}

impl Uri {
	
	pub fn new(uri: &str) -> Result<Uri, Failure> {
		
		let initial : Vec<&str> = uri.split("://").collect();

		if initial[0] != "spring" || initial.len() < 2 {
			return Err(Failure::InvalidFormat)
		}

		let mut gsn : Vec<String> = Vec::new();
		let mut res: Vec<String> = Vec::new();
		let mut query: &str = "";


		let atoms : Vec<&str> = initial[1].split('?').collect();
		if atoms.len() > 1 {
			query = atoms[1]
		}

		let atoms : Vec<&str> = atoms[0].split('/').collect();

		if atoms.len() > 1 {
			for i in 1 .. atoms.len() {
				if atoms[i].is_empty() { continue } 
				res.push(String::from(atoms[i]))
			}
		}
		
		
		let v : Vec<&str> = atoms[0].split('.').collect();
		
		
		let gtn = match v[v.len()-1] {
			"uk" => "uk",
			_ => ""
		};
		
		for s in v {
			gsn.push(String::from(s))
		}
		
		Ok(Uri {
			gsn: gsn,
			gtn: String::from(gtn),
			res: res,
			query: String::from(query),
		})
	}
		
	pub fn route(&self) -> &Vec<String> {
		&self.gsn
	}
	
	pub fn route_mut(&mut self) -> &mut Vec<String> {
		&mut self.gsn
	} 
	
	
	pub fn gtn(&self) -> &str {
		&self.gtn
	}


	pub fn query(&self) -> &str {
		&self.query
	}
	
	pub fn res(&self) -> &Vec<String> {
		&self.res
	}
	
	pub fn to_string(&self) -> String {
		
		let mut s = "spring://".to_string();
		let last = self.gsn.len()-1;
		
		for i in 0 .. last {
			s.push_str(&self.gsn[i]);
			s.push('.');
		}
		
		s.push_str(&self.gsn[last]);
		

		if self.res.len() > 0 {
			for p in &self.res {
				if p.is_empty() { continue }
				
				s.push('/');
				s.push_str(p.as_str());
			}
		}

		if self.query.len() > 0 {
			s.push('?');
			s.push_str(&self.query);
		}
		s
	}
	
	pub fn query_map(&self) -> Option<HashMap<String, String>> {
		if self.query.is_empty() { return None }
		
		
		
		let mut m = HashMap::new();
			 
		for val in self.query.split("&") {
			let (k,v) = val.split_at(match val.find("=") {
					None => val.len(),
					Some(i) => i, 
				});
			
			if v.len() > 0 {				 
				m.insert(String::from(k),String::from(&v[1..])); 
			} else { 
				m.insert(String::from(k),String::from(v));
			}
		}
			
		Some(m)
	}
	
	pub fn query_param(&self, param: &str) -> Option<String> {
		let qm = match self.query_map() {
			Some(qm) => qm,
			None => return None
		};
		
		match qm.get(param) {
			Some(s) => Some(s.clone()),
			None => None
		}
	}
	
}

impl Clone for Uri {
	fn clone(&self) -> Uri {
		Uri {
			gsn: (&self).gsn.clone(),
			gtn: (&self).gtn.to_string(),
			res: (&self).res.clone(),
			query: (&self).query.to_string()
		}
	}

	fn clone_from(&mut self, source: &Uri)  {
			self.gsn = source.route().clone();
			self.gtn = source.gtn().to_string();
			self.res = source.res().clone();
			self.query = source.query().to_string();
	}
}