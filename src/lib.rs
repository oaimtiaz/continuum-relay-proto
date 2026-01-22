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

// // Re-export commonly used types at crate root for convenience
// pub use relay::v1::{
//     // Message types
//     envelope, Close, Data, Envelope, OpenTunnel, Register, TunnelAccepted, TunnelRejected,
//     // Client
//     relay_service_client::RelayServiceClient,
//     // Server trait and wrapper
//     relay_service_server::{RelayService, RelayServiceServer},
// };
