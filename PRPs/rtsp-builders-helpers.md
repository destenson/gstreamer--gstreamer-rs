# PRP: RTSP Builders and Helper Functions

## Overview
Implement builder patterns and helper functions for RTSPConnection and RTSPTransport to provide idiomatic Rust APIs. This includes connection builders, transport configuration helpers, and convenience methods.

## Context & Resources
- **Pattern Reference**: Builder patterns in `gstreamer-rtsp-server/src/rtsp_token.rs` (RTSPToken::builder())
- **Documentation**: https://gstreamer.freedesktop.org/documentation/rtsplib/
- **Related Work**: Existing builder patterns in gstreamer-rs codebase

## Implementation Blueprint

### RTSPTransport Builder
Provide fluent API for configuring transport parameters without direct field access.

#### Builder Methods
- protocol() - Set trans protocol
- profile() - Set transport profile  
- lower_transport() - Set lower transport
- destination() - Set destination address
- source() - Set source address
- mode() - Set play/record modes
- client_ports() - Set client port range
- server_ports() - Set server port range
- interleaved() - Set interleaved channels
- ttl() - Set TTL value
- ssrc() - Set SSRC

### RTSPConnection Builder
Simplify connection creation and configuration with builder pattern.

#### Builder Methods
- url() - Set connection URL
- proxy() - Configure proxy
- auth() - Set authentication
- timeout() - Set connection timeout
- tunneled() - Enable tunneling
- accept_certificate() - Set certificate validation

### Helper Functions

#### Transport Helpers
- parse_transport() - Parse transport string to RTSPTransport
- format_transport() - Format RTSPTransport to string
- transport_get_mime() - Get MIME type for transport
- transport_get_manager() - Get manager for transport type

#### Connection Helpers  
- options_from_uri() - Parse RTSP URI options
- default_port() - Get default port for protocol

### Tasks
1. Create builders.rs module in gstreamer-rtsp/src/
2. Implement RTSPTransportBuilder with fluent API
3. Add validation in TransportBuilder::build()
4. Implement RTSPConnectionBuilder with configuration methods
5. Add ConnectionBuilder::connect() with error handling
6. Create helper functions module
7. Implement transport parsing/formatting helpers
8. Add URI and port utility functions
9. Write documentation with examples
10. Add builder imports to prelude
11. Export builders in lib.rs

## Pattern Examples
Reference the builder pattern from RTSPToken which uses:
- Private Builder struct
- Fluent field() methods returning &mut Self
- build() method constructing final type
- Validation in build()

## Validation Gates
```bash
# Build and check
cd gstreamer-rtsp && cargo build --all-features
cargo fmt --check && cargo clippy --all-features -- -D warnings

# Test builders
cargo test builders --all-features

# Example compilation
cargo build --example rtsp_client_example
```

## Success Criteria
- Builders provide type-safe configuration
- All transport/connection options accessible via builders
- Helper functions reduce boilerplate
- Documentation includes usage examples
- Builders follow established patterns in codebase

## Confidence Score: 8/10
Well-established builder patterns exist in codebase to follow. Main work is adapting pattern to RTSP types with proper validation.