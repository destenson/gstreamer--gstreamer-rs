# PRP: RTSPConnection Safe Rust Bindings

## Overview
Implement safe Rust bindings for GstRTSPConnection FFI type in gstreamer-rtsp module. RTSPConnection manages RTSP connections to servers, providing methods to send/receive messages and handle tunneling.

## Context & Resources
- **FFI Definition**: `gstreamer-rtsp/sys/src/lib.rs:360-365` - GstRTSPConnection opaque struct
- **Documentation**: https://gstreamer.freedesktop.org/documentation/rtsplib/gstrtspconnection.html
- **Pattern Reference**: Connection-style types in gstreamer typically use RefCounted or Boxed wrappers
- **Related Types**: RTSPMessage, RTSPUrl, RTSPAuthMethod (already in safe bindings)

## Implementation Blueprint

### Structure
RTSPConnection should be wrapped as a RefCounted type with Send+Sync traits, as it manages network resources and can be shared across threads.

### Core Functionality Groups (from gstrtspconnection.h)

#### Connection Management
- create() - Create connection from URL (gst_rtsp_connection_create)
- create_from_socket() - Create from existing GSocket with initial buffer
- accept() - Accept connection on socket
- connect_usec() - Connect with microsecond timeout
- connect_with_response_usec() - Connect and get response
- close() - Close the connection
- free() - Free resources

#### TLS/Security
- get_tls() - Get GTlsConnection
- set_tls_validation_flags() - Set certificate validation
- get_tls_validation_flags() - Get validation flags
- set_tls_database() - Set certificate database
- set_tls_interaction() - Set TLS interaction handler
- set_accept_certificate_func() - Custom certificate validation

#### Message Operations  
- send_usec() - Send single RTSP message with timeout
- send_messages_usec() - Send multiple messages
- receive_usec() - Receive RTSP message with timeout
- read_usec() - Read raw bytes
- write_usec() - Write raw bytes
- poll_usec() - Poll for events with timeout
- flush() - Flush connection state

#### Configuration
- set_auth() - Set authentication (method, user, pass)
- set_auth_param() - Set auth parameter
- clear_auth_params() - Clear all auth params
- set_proxy() - Configure proxy (host, port)
- set_qos_dscp() - Set QoS DSCP value
- set_content_length_limit() - Set max content length
- set_http_mode() - Enable HTTP mode
- add_extra_http_request_header() - Add HTTP headers

#### Tunneling
- set_tunneled() - Enable/disable tunneling
- is_tunneled() - Check tunnel status
- get_tunnelid() - Get tunnel ID
- do_tunnel() - Perform tunneling with another connection
- set_remember_session_id() - Session ID management
- set_ignore_x_server_reply() - Ignore server reply header

#### Information Retrieval
- get_url() - Get connection URL
- get_ip() - Get IP address
- set_ip() - Set IP address
- get_read_socket() - Get read GSocket
- get_write_socket() - Get write GSocket
- next_timeout_usec() - Get next timeout value
- reset_timeout() - Reset connection timeout

### Tasks
1. Create rtsp_connection.rs in gstreamer-rtsp/src/
2. Define RTSPConnection wrapper with appropriate lifetime management
3. Implement connection creation methods (create, create_from_socket)
4. Add connect/close methods with proper error handling
5. Implement message send/receive with RTSPMessage integration
6. Add authentication methods (clear_auth_params, set_auth, set_auth_param)
7. Implement proxy configuration methods
8. Add tunneling support methods
9. Implement timeout and polling functionality
10. Add socket access methods (get_read_socket, get_write_socket)
11. Implement TLS/certificate handling
12. Add proper Drop implementation for resource cleanup
13. Export type in lib.rs

## Dependencies
- RTSPMessage for message operations
- RTSPUrl for URL handling
- RTSPResult for error handling
- gio::Socket for socket operations
- glib MainContext for async operations

## Error Handling
Map RTSPResult error codes to Rust Result types with descriptive error messages.

## Validation Gates
```bash
# Build and check
cd gstreamer-rtsp && cargo build --all-features
cargo fmt --check && cargo clippy --all-features -- -D warnings

# Run tests
cargo test rtsp_connection --all-features

# Documentation check
cargo doc --no-deps --open
```

## Success Criteria
- Can create connections from URLs
- Can send and receive RTSP messages
- Proper timeout handling
- Authentication works correctly
- Memory and socket resources are properly managed
- Thread-safe operations where applicable

## Confidence Score: 7/10
More complex than RTSPTransport due to resource management, socket handling, and async operations. Requires careful lifetime management and error handling.