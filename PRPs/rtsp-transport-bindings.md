# PRP: RTSPTransport Safe Rust Bindings

## Overview
Implement safe Rust bindings for GstRTSPTransport FFI type in gstreamer-rtsp module. RTSPTransport is a fundamental structure that holds RTSP transport configuration values including protocol details, port pairs, and transport modes.

## Context & Resources
- **FFI Definition**: `gstreamer-rtsp/sys/src/lib.rs:580` - GstRTSPTransport struct
- **Documentation**: https://gstreamer.freedesktop.org/documentation/rtsplib/gstrtsptransport.html
- **Pattern Reference**: `gstreamer-rtsp/src/rtsp_message.rs` - Shows glib::wrapper! pattern for RTSP types
- **Related Types**: RTSPLowerTrans, RTSPTransMode, RTSPProfile (already available in safe bindings)

## Implementation Blueprint

### Structure
The RTSPTransport struct should be wrapped as a Boxed type similar to RTSPMessage, providing safe access to transport configuration fields.

### Key Fields to Expose (from gstrtsptransport.h)
- trans: GstRTSPTransMode (RTP/RDT transfer mode)
- profile: GstRTSPProfile (AVP/SAVP/AVPF/SAVPF)
- lower_transport: GstRTSPLowerTrans (UDP/UDP_MCAST/TCP/HTTP/TLS)
- destination: String for destination IP/hostname
- source: String for source IP/hostname
- layers: guint for number of layers
- mode_play: Boolean for play mode
- mode_record: Boolean for record mode
- append: Boolean for append mode
- interleaved: GstRTSPRange for interleaved channels
- ttl: guint for multicast TTL
- port: GstRTSPRange for multicast port pair
- client_port: GstRTSPRange for client port pair (UDP/TCP)
- server_port: GstRTSPRange for server port pair (UDP/TCP)
- ssrc: guint for RTP SSRC value

### Tasks
1. Create rtsp_transport.rs in gstreamer-rtsp/src/
2. Define RTSPRange struct wrapper for GstRTSPRange (min/max fields)
3. Implement glib::wrapper! for RTSPTransport as Boxed type
4. Add constructor using gst_rtsp_transport_new() and gst_rtsp_transport_init()
5. Implement field accessors following Rust naming conventions
6. Implement as_text() method using gst_rtsp_transport_as_text()
7. Implement parse() method using gst_rtsp_transport_parse()
8. Implement get_media_type() method using gst_rtsp_transport_get_media_type()
9. Implement get_manager() method using gst_rtsp_transport_get_manager()
10. Add Display trait implementation using as_text()
11. Add Debug trait implementation with field details
12. Update Gir.toml to include RTSPTransport manual binding
13. Export type in lib.rs

## Dependencies
- glib::wrapper! macro for type wrapping
- glib::translate traits for FFI conversion
- Existing RTSPLowerTrans, RTSPTransMode, RTSPProfile enums

## Validation Gates
```bash
# Build and check
cd gstreamer-rtsp && cargo build --all-features
cargo fmt --check && cargo clippy --all-features -- -D warnings

# Run tests
cargo test rtsp_transport --all-features
```

## Success Criteria
- RTSPTransport can be created, configured, and converted to/from string representation
- All public fields are safely accessible
- Memory management is handled correctly through Boxed wrapper
- Type integrates with existing RTSP types

## Confidence Score: 8/10
Clear FFI definitions exist, established patterns to follow, straightforward struct wrapping task.