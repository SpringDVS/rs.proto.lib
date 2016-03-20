/* Notice:	Copyright 2016, The Care Connections Initiative c.i.c.
 * Author: 	Charlie Fyvie-Gauld (cfg@zunautica.org)
 * License: GPLv3 (http://www.gnu.org/licenses/gpl-3.0.txt)
 */
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
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Bounds {
	MaxNodeType = 3,
	PacketContentSize = 512,
	FrameRegisterLen = 124,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Failure {
	OutOfBounds, InvalidArgument, InvalidBytes, InvalidConversion
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Success {
	Ok	
}
