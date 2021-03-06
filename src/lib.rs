#![warn(clippy::pedantic)]
// TODO: Remove this once ready to publish.
#![allow(clippy::missing_errors_doc)]
// TODO: Uncomment this once ready to publish.
//#![warn(missing_docs)]

mod builders;
mod error;
mod mock_connection;
mod timeout;
mod vec_ext;
mod web_socket;

pub use builders::{
    WebSocketClientBuilder,
    WebSocketServerBuilder,
};
pub use error::Error;
use futures::{
    AsyncRead,
    AsyncWrite,
};
use vec_ext::VecExt;
use web_socket::MaskDirection;
pub use web_socket::{
    LastFragment,
    SinkMessage,
    StreamMessage,
    WebSocket,
};

pub trait ConnectionTx: AsyncWrite + Send + Unpin + 'static {}
impl<T: AsyncWrite + Send + Unpin + 'static> ConnectionTx for T {}

pub trait ConnectionRx: AsyncRead + Send + Unpin + 'static {}
impl<T: AsyncRead + Send + Unpin + 'static> ConnectionRx for T {}
