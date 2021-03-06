
use bytes::{ByteBuf, MutByteBuf};
use dns::NormalizedQuestion;
use mio::*;
use mio::timer::Timeout;

use super::DNS_QUERY_MAX_SIZE;
use tcp_listener::TCP_QUERY_HEADER_SIZE;

pub struct Client {
    pub normalized_question: Option<NormalizedQuestion>,
    pub tcp_stream: tcp::TcpStream,
    pub read_bufw: MutByteBuf,
    pub write_buf: Option<ByteBuf>,
    pub expected_len: Option<u16>,
    pub interest: Ready,
    pub timeout: Option<Timeout>,
    pub resolving: bool,
    pub attic: bool,
}

impl Client {
    pub fn new(tcp_stream: tcp::TcpStream) -> Client {
        Client {
            normalized_question: None,
            tcp_stream: tcp_stream,
            read_bufw: MutByteBuf::with_capacity(TCP_QUERY_HEADER_SIZE + DNS_QUERY_MAX_SIZE),
            write_buf: None,
            expected_len: None,
            interest: Ready::hup() | Ready::error(),
            timeout: None,
            resolving: false,
            attic: false,
        }
    }
}
