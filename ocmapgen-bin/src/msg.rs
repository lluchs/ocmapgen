use serde_cbor as cbor;
use serde::de::Deserialize;
use serde::bytes::ByteBuf;

use std::io::{self, Write};

#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    RenderMap { source: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    Image {
        fg: ByteBuf,
        bg: Option<ByteBuf>,
        warnings: Option<String>,
        script_output: Option<String>,
    },
    Error(String),
}

pub fn read_request() -> cbor::Result<Request> {
    let mut stdin = io::stdin();
    let mut deserializer = cbor::de::Deserializer::new(&mut stdin);
    Deserialize::deserialize(&mut deserializer)
}

pub fn write_response(res: &Response) -> cbor::Result<()> {
    let mut stdout = io::stdout();
    cbor::ser::to_writer(&mut stdout, res)?;
    stdout.flush().unwrap();
    Ok(())
}
