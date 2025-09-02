//! UDP数据发送器

use crate::types::PlaybackError;
use log::{debug, info};
use std::net::{Ipv4Addr, SocketAddr, UdpSocket};

#[derive(Debug, Clone)]
pub enum NetworkMode {
    Broadcast,
    Multicast { group: Ipv4Addr },
    Unicast { target: SocketAddr },
}

#[derive(Debug)]
pub struct UDPSender {
    socket: UdpSocket,
    mode: NetworkMode,
    target_addr: SocketAddr,
}

impl UDPSender {
    pub fn new(mode: NetworkMode, target_addr: SocketAddr) -> Result<Self, PlaybackError> {
        let socket = match &mode {
            NetworkMode::Broadcast => {
                let socket = UdpSocket::bind("0.0.0.0:0")
                    .map_err(|e| PlaybackError::NetworkError(e.to_string()))?;
                socket
                    .set_broadcast(true)
                    .map_err(|e| PlaybackError::NetworkError(e.to_string()))?;
                socket
            }
            NetworkMode::Multicast { group: _ } => {
                let socket = UdpSocket::bind("0.0.0.0:0")
                    .map_err(|e| PlaybackError::NetworkError(e.to_string()))?;
                socket
                    .set_multicast_loop_v4(true)
                    .map_err(|e| PlaybackError::NetworkError(e.to_string()))?;
                socket
            }
            NetworkMode::Unicast { target: _ } => UdpSocket::bind("0.0.0.0:0")
                .map_err(|e| PlaybackError::NetworkError(e.to_string()))?,
        };

        info!("创建UDP发送器 - 模式: {:?}, 目标: {}", mode, target_addr);

        Ok(UDPSender {
            socket,
            mode,
            target_addr,
        })
    }

    pub fn send_data(&self, data: &[u8]) -> Result<(), PlaybackError> {
        let bytes_sent = match &self.mode {
            NetworkMode::Broadcast => self
                .socket
                .send_to(data, self.target_addr)
                .map_err(|e| PlaybackError::NetworkError(e.to_string()))?,
            NetworkMode::Multicast { group } => self
                .socket
                .send_to(
                    data,
                    SocketAddr::new(std::net::IpAddr::V4(*group), self.target_addr.port()),
                )
                .map_err(|e| PlaybackError::NetworkError(e.to_string()))?,
            NetworkMode::Unicast { target } => self
                .socket
                .send_to(data, *target)
                .map_err(|e| PlaybackError::NetworkError(e.to_string()))?,
        };

        debug!("发送数据: {} 字节到 {}", bytes_sent, self.target_addr);
        Ok(())
    }

    pub fn get_mode(&self) -> &NetworkMode {
        &self.mode
    }

    pub fn get_target_addr(&self) -> &SocketAddr {
        &self.target_addr
    }

    pub fn get_local_addr(&self) -> std::io::Result<SocketAddr> {
        self.socket.local_addr()
    }
}
