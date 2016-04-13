/* Notice:	Copyright 2016, The Care Connections Initiative c.i.c.
 * Author: 	Charlie Fyvie-Gauld (cfg@zunautica.org)
 * License: GPLv3 (http://www.gnu.org/licenses/gpl-3.0.txt)
 */
// ----- Enumeration Lists ----- \\
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DvspMsgType {
	Undefined = 0,
	GsnRegistration = 1,
	GsnResolution = 2,
	GsnArea = 3,
	GsnState = 4,
	GsnNodeInfo = 5,
	GsnNodeStatus = 6,
	
	GsnTypeRequest = 8,
	
	GtnRegistration = 22,

	GsnResponse = 30,
	GsnResponseNodeInfo = 31,
	GsnResponseNetwork = 32,
	GsnResponseHigh = 33,
	GsnResponseStatus = 34,
	
	UnitTest = 101,

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

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DvspService {
	Undefined = 0,
	Dvsp = 1,
	Http = 2,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DvspNodeState {
	Disabled = 0,
	Enabled = 1,
	Unresponsive = 2,
	Unspecified = 3,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Bounds {
	MaxNodeType = 3,
	PacketContentSize = 512,
	FrameRegisterLen = 124,
	NodeRegister = 125,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Failure {
	OutOfBounds, InvalidArgument, InvalidBytes, InvalidConversion, InvalidFormat,
	Duplicate,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Success {
	Ok	
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum UnitTestAction {
	Undefined = 0,
	Reset = 1,
	UpdateAddress = 2,
}
