use super::*;


impl TryFrom<u8> for GatewayRequestHeader {
	type Error = ();

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::BorrowResource),
			1 => Ok(Self::WriteResource),
			2 => Ok(Self::CustomCommand),
			3 => Ok(Self::ListDirectory),
			4 => Ok(Self::DuplicateDirectory),
			5 => Ok(Self::DeleteDirectory),
			_ => Err(())
		}
	}
}


impl From<GatewayRequestHeader> for u8 {
	fn from(header: GatewayRequestHeader) -> Self {
		match header {
			GatewayRequestHeader::BorrowResource => 0,
			GatewayRequestHeader::WriteResource => 1,
			GatewayRequestHeader::CustomCommand => 2,
			GatewayRequestHeader::ListDirectory => 3,
			GatewayRequestHeader::DuplicateDirectory => 4,
			GatewayRequestHeader::DeleteDirectory => 5
		}
	}
}


impl From<GatewayResponseHeader> for u8 {
	fn from(header: GatewayResponseHeader) -> Self {
		use GatewayResponseHeader::*;

		match header {
			Ok => 0,
			BadPath => 1,
			NotFound => 2,
			BadResource => 4,
			BadRequest => 5,
			InternalError => 6,
			IsDirectoryError => 7,
		}
	}
}


impl TryFrom<u8> for GatewayResponseHeader {
	type Error = ();

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::Ok),
			1 => Ok(Self::BadPath),
			2 => Ok(Self::NotFound),
			4 => Ok(Self::BadResource),
			5 => Ok(Self::BadRequest),
			6 => Ok(Self::InternalError),
			7 => Ok(Self::IsDirectoryError),
			_ => Err(())
		}
	}
}