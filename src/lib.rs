//! Protocol Buffer definitions and gRPC stubs for Continuum Relay.
//!
//! This crate provides:
//! - Generated message types from protobuf definitions
//! - `RelayServiceClient` for making gRPC calls
//! - `RelayServiceServer` for implementing gRPC servers

/// Generated protobuf types and gRPC service stubs.
pub mod relay {
    pub mod v1 {
        tonic::include_proto!("continuum.relay.v1");
    }
}

// Re-export commonly used types at crate root for convenience
pub use relay::v1::{
    // Daemon service
    daemon_relay_service_client::DaemonRelayServiceClient,
    daemon_relay_service_server::{DaemonRelayService, DaemonRelayServiceServer},
    // Client service
    client_relay_service_client::ClientRelayServiceClient,
    client_relay_service_server::{ClientRelayService, ClientRelayServiceServer},
    // Daemon envelope messages
    envelope, CloseSession, Envelope, Ping, Pong, SessionAccepted, SessionRejected, StartSession,
    Target,
    // Client messages
    CreateSessionRequest, CreateSessionResponse,
    // Tunnel messages
    tunnel_message, Close, Data, Open, TunnelMessage,
};
