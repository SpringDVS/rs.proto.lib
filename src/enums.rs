// ----- Enumeration Lists ----- \\
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DvspMsgType {
	Undefined = 0,
	GsnRegistration = 1,
	GsnResponse = 8,

}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DvspRcode {
	NetspaceError = 101,
	NetspaceDuplication = 102,
	NetworkError = 103,
	MalformedContent = 104,
	Ok = 200,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DvspNodeType {
	Undefined = 0,
	Root = 1,
	Org = 2,
}