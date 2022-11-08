use std::collections::VecDeque;
use crate::{GatewayRequestHeader, GatewayResponseHeader};


pub struct Message<H: Into<u8>> {
	pub header: H,
	pub body: Option<VecDeque<u8>>
}


impl<H: Into<u8>> Message<H> {
	pub fn new<E, D, B: Into<VecDeque<u8>>>(header_byte: D, body: B) -> Result<Self, E>
		where
			D: TryInto<H, Error=E>
	{
		Ok(Self {
			header: header_byte.try_into()?,
			body: Some(body.into())
		})
	}
	pub fn new_header<E, D>(header_byte: D,) -> Result<Self, E>
		where
			D: TryInto<H, Error=E>
	{
		Ok(Self {
			header: header_byte.try_into()?,
			body: None
		})
	}

	pub fn to_bytes(self) -> Vec<u8> {
		if let Some(body) = self.body {
			let body_len = body.len();
			let mut data = Vec::with_capacity(5 + body_len);
			data.push(self.header.into());
			data.append(&mut (body_len as u32).to_be_bytes().to_vec());
			data.append(&mut body.into());
			data
		} else {
			vec![self.header.into(), 0, 0, 0, 0]
		}
	}
}


impl Message<GatewayResponseHeader> {
	pub fn new_response<E, D, B>(header_byte: D, body: B) -> Result<Message<GatewayResponseHeader>, E>
		where
			D: TryInto<GatewayResponseHeader, Error=E>,
			B: Into<VecDeque<u8>>
	{
		Ok(Message::<GatewayResponseHeader> {
			header: header_byte.try_into()?,
			body: Some(body.into())
		})
	}
	pub fn new_response_header<E, D>(header_byte: D) -> Result<Message<GatewayResponseHeader>, E>
		where
			D: TryInto<GatewayResponseHeader, Error=E>
	{
		Ok(Message::<GatewayResponseHeader> {
			header: header_byte.try_into()?,
			body: None
		})
	}
}


impl Message<GatewayRequestHeader> {
	pub fn new_request<E, D, B>(header_byte: D, body: B) -> Result<Message<GatewayRequestHeader>, E>
		where
			D: TryInto<GatewayRequestHeader, Error=E>,
			B: Into<VecDeque<u8>>
	{
		Ok(Message::<GatewayRequestHeader> {
			header: header_byte.try_into()?,
			body: Some(body.into())
		})
	}
	pub fn new_request_header<E, D>(header_byte: D) -> Result<Message<GatewayRequestHeader>, E>
		where
			D: TryInto<GatewayRequestHeader, Error=E>
	{
		Ok(Message::<GatewayRequestHeader> {
			header: header_byte.try_into()?,
			body: None
		})
	}
}