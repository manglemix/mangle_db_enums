mod header_bin;


#[derive(Debug)]
pub enum GatewayRequestHeader {
	BorrowResource,
	WriteResource,
	CustomCommand,
	ListDirectory,
	DuplicateDirectory,
	DeleteDirectory
}


#[derive(Debug)]
pub enum GatewayResponseHeader {
	Ok,
	BadPath,
	NotFound,
	BadResource,
	BadRequest,
	InternalError,
	IsDirectoryError,
}
