use std::net::SocketAddr;

use tokio::net::TcpStream;
use hyper::{Request, Body};

// https://github.com/hyperium/hyper/issues/1402
pub struct ConnectionInfo {
    local_addr: Option<SocketAddr>,
    remote_addr: Option<SocketAddr>,
}

impl ConnectionInfo {
    pub fn new(socket: &TcpStream) -> ConnectionInfo {
        ConnectionInfo {
            local_addr: Some(socket.local_addr().unwrap()),
            remote_addr: Some(socket.peer_addr().unwrap()),
        }
    }

    pub fn get(req: &Request<Body>) -> ConnectionInfo {
        let connection_info = req.extensions().get::<ConnectionInfo>().unwrap();

        ConnectionInfo {
            local_addr: connection_info.local_addr,
            remote_addr: connection_info.remote_addr,
        }
    }
    
    pub fn set(&self, req: &mut Request<Body>) {
        req.extensions_mut().insert(ConnectionInfo {
            local_addr: self.local_addr,
            remote_addr: self.remote_addr,
        });
    }

    pub fn local_addr(&self) -> Option<SocketAddr> {
        self.local_addr
    }

    pub fn remote_addr(&self) -> Option<SocketAddr> {
        self.remote_addr
    }
}
