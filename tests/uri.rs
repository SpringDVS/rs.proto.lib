extern crate spring_dvs;
use spring_dvs::uri::*;
#[test]

fn ts_model_uri_new_pass() {
	
	let r = Uri::new("spring://cci.esusx.uk");
	
	assert!(r.is_ok());
	
	let uri = r.unwrap();
	let gsn = uri.route();
	assert_eq!(gsn.len(), 3);
	assert_eq!(gsn[0], "cci");
	assert_eq!(gsn[1], "esusx");
	assert_eq!(gsn[2], "uk");
	assert_eq!(uri.gtn(), "uk");
}

#[test]
fn ts_model_uri_new_no_gtn_pass() {
	
	let r = Uri::new("spring://cci.esusx");
	
	assert!(r.is_ok());
	
	let uri = r.unwrap();
	let gsn = uri.route();
	assert_eq!(gsn.len(), 2);
	assert_eq!(gsn[0], "cci");
	assert_eq!(gsn[1], "esusx");
	assert_eq!(uri.gtn(), "");
}

#[test]
fn ts_model_uri_new_res_pass() {
	
	let r = Uri::new("spring://cci.esusx.uk/res");
	
	assert!(r.is_ok());
	
	let uri = r.unwrap();
	let gsn = uri.route();
	assert_eq!(gsn.len(), 3);
	assert_eq!(gsn[0], "cci");
	assert_eq!(gsn[1], "esusx");
	assert_eq!(gsn[2], "uk");
	assert_eq!(uri.gtn(), "uk");
	assert_eq!(uri.res()[0], "res");
}

#[test]
fn ts_model_uri_new_res_multi_pass() {
	
	let r = Uri::new("spring://cci.esusx.uk/res/home/");
	
	assert!(r.is_ok());
	
	let uri = r.unwrap();
	let gsn = uri.route();
	assert_eq!(gsn.len(), 3);
	assert_eq!(gsn[0], "cci");
	assert_eq!(gsn[1], "esusx");
	assert_eq!(gsn[2], "uk");
	assert_eq!(uri.gtn(), "uk");
	assert_eq!(uri.res().len(), 2);
	assert_eq!(uri.res()[0], "res");
	assert_eq!(uri.res()[1], "home");
	
}

#[test]
fn ts_model_uri_new_res_query_pass() {
	
	let r = Uri::new("spring://cci.esusx.uk/res?query=test");
	
	assert!(r.is_ok());
	
	let uri = r.unwrap();
	let gsn = uri.route();
	assert_eq!(gsn.len(), 3);
	assert_eq!(gsn[0], "cci");
	assert_eq!(gsn[1], "esusx");
	assert_eq!(gsn[2], "uk");
	assert_eq!(uri.gtn(), "uk");

	assert_eq!(uri.res()[0], "res");
	assert_eq!(uri.query(), "query=test");
}

#[test]
fn ts_model_uri_new_res_multi_query_pass() {
	
	let r = Uri::new("spring://cci.esusx.uk/res/home/?query=test");
	
	assert!(r.is_ok());
	
	let uri = r.unwrap();
	let gsn = uri.route();
	assert_eq!(gsn.len(), 3);
	assert_eq!(gsn[0], "cci");
	assert_eq!(gsn[1], "esusx");
	assert_eq!(gsn[2], "uk");
	assert_eq!(uri.gtn(), "uk");

	assert_eq!(uri.res().len(), 2);
	assert_eq!(uri.res()[0], "res");
	assert_eq!(uri.res()[1], "home");
	
	assert_eq!(uri.query(), "query=test");
}

#[test]
fn ts_model_uri_new_fail() {
	
	let r = Uri::new("cci.esusx.uk/res?query=test");
	
	assert!(r.is_err());
	assert_eq!(Failure::InvalidFormat, r.unwrap_err());
}

#[test]
fn ts_model_uri_to_string_basic_pass() {
	let s = "spring://cci.esusx.uk";
	let r = Uri::new(s);
	
	assert!(r.is_ok());
	let uri = r.unwrap();
	
	assert_eq!(s, uri.to_string());
}


#[test]
fn ts_model_uri_to_string_basic_res_query_pass() {
	let s = "spring://cci.esusx.uk/res?query=test";
	let r = Uri::new(s);
	
	assert!(r.is_ok());
	let uri = r.unwrap();
	
	assert_eq!(s, uri.to_string());
}

#[test]
fn ts_model_uri_clone_pass() {
	let s = "spring://cci.esusx.uk/res?query=test";
	let r = Uri::new(s);
	
	assert!(r.is_ok());
	let uri = r.unwrap();
	
	let cpy = uri.clone();
	
	assert_eq!(s, cpy.to_string());
}

#[test]
fn ts_model_uri_clone_from_pass() {
	let s = "spring://cci.esusx.uk/res?query=test";
	let r = Uri::new(s);
	
	assert!(r.is_ok());
	let uri = r.unwrap();
	
	let mut cpy: Uri = Uri::new("spring://abc").unwrap();
	cpy.clone_from(&uri);
	
	assert_eq!(s, cpy.to_string());
}

#[test]
fn ts_model_uri_mut_route_pass() {
	let s = "spring://cci.esusx.uk/res?query=test";
	let r = Uri::new(s);
	
	assert!(r.is_ok());
	let mut uri = r.unwrap();
	
	
	uri.route_mut().pop();
	assert_eq!("spring://cci.esusx/res?query=test", uri.to_string());
	
	uri.route_mut().pop();
	assert_eq!("spring://cci/res?query=test", uri.to_string());
	
}

#[test]
fn ts_uri_fail_no_uri() {
	let s = "spring://";
	let r = Uri::new(s);
	
	assert!(r.is_err());
	
}


#[test]
fn ts_uri_query_map_pass() {
	let s = "spring://cci.esusx.uk/res?query=test&query2=test2";
	let r = Uri::new(s);
	
	assert!(r.is_ok());
	let uri = r.unwrap();
	
	
	let o = uri.query_map();
	assert!(o.is_some());
	
	let qm = o.unwrap();
	
	assert_eq!(qm.len(), 2);
	assert_eq!(qm["query"], "test");
	assert_eq!(qm["query2"], "test2");
}

#[test]
fn ts_uri_query_map_fail() {
	let s = "spring://cci.esusx.uk/res?";
	let r = Uri::new(s);
	
	assert!(r.is_ok());
	let uri = r.unwrap();
	
	
	let o = uri.query_map();
	assert!(o.is_none());
}

#[test]
fn ts_uri_query_param_pass() {
	let r = Uri::new("spring://cci.esusx.uk/res?__meta=outcode&q2=test");
	
	assert!(r.is_ok());
	let uri = r.unwrap();
	let o = uri.query_param("__meta");
	assert!(o.is_some());
	assert_eq!(o.unwrap(), "outcode");
}

#[test]
fn ts_uri_query_param_fail() {
	let r = Uri::new("spring://cci.esusx.uk/res?__meta=outcode&q2=test");
	assert!(r.is_ok());
	
	let uri = r.unwrap();
	assert!(uri.query_param("void").is_none());
	
	let r = Uri::new("spring://cci.esusx.uk/res?");
	assert!(r.is_ok());
	let uri = r.unwrap();
	
	assert!(uri.query_param("void").is_none())
}