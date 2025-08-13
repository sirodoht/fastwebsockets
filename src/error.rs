use thiserror::Error;

#[derive(Error, Debug)]
pub enum WebSocketError {
  #[error("Invalid fragment")]
  InvalidFragment,
  #[error("Invalid UTF-8")]
  InvalidUTF8,
  #[error("Invalid continuation frame")]
  InvalidContinuationFrame,
  #[error("Invalid status code: {0}")]
  InvalidStatusCode(u16),
  #[error("Invalid upgrade header")]
  InvalidUpgradeHeader,
  #[error("Invalid connection header")]
  InvalidConnectionHeader,
  #[error("Connection is closed")]
  ConnectionClosed,
  #[error("Invalid close frame")]
  InvalidCloseFrame,
  #[error("Invalid close code")]
  InvalidCloseCode,
  #[error("Unexpected EOF")]
  UnexpectedEOF,
  #[error("Reserved bits must be zero; found rsv1={rsv1}, rsv2={rsv2}, rsv3={rsv3} | Frame header: first_byte=0x{first_byte:02X} (binary: {first_byte:08b}), second_byte=0x{second_byte:02X} | Decoded: FIN={fin}, opcode={opcode}, masked={masked}, payload_len_code={payload_len_code}")]
  ReservedBitsNotZero {
    rsv1: bool,
    rsv2: bool,
    rsv3: bool,
    first_byte: u8,
    second_byte: u8,
    fin: bool,
    opcode: u8,
    masked: bool,
    payload_len_code: u8,
  },
  #[error("Control frame must not be fragmented")]
  ControlFrameFragmented,
  #[error("Ping frame too large")]
  PingFrameTooLarge,
  #[error("Frame too large")]
  FrameTooLarge,
  #[error("Sec-Websocket-Version must be 13")]
  InvalidSecWebsocketVersion,
  #[error("Invalid value")]
  InvalidValue,
  #[error("Sec-WebSocket-Key header is missing")]
  MissingSecWebSocketKey,
  #[error(transparent)]
  IoError(#[from] std::io::Error),
  #[cfg(feature = "upgrade")]
  #[error(transparent)]
  HTTPError(#[from] hyper::Error),
  #[cfg(feature = "unstable-split")]
  #[error("Failed to send frame")]
  SendError(#[from] Box<dyn std::error::Error + Send + Sync + 'static>),
}
