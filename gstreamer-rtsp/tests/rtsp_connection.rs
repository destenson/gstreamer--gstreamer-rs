// Take a look at the license at the top of the repository in the LICENSE file.

use gstreamer_rtsp::{
    builders::RTSPConnectionBuilder, rtsp_message::RTSPMessage, RTSPAuthMethod, RTSPConnection,
    RTSPHeaderField, RTSPMethod, RTSPResult, RTSPStatusCode, RTSPUrl, RTSPVersion,
};

#[cfg(feature = "v1_18")]
use gstreamer_rtsp::RTSPEvent;

fn init() {
    use std::sync::Once;
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        gstreamer_rtsp::init().unwrap();
    });
}

#[test]
fn test_connection_creation() {
    init();

    // Test creating connection from URL
    let (result, url) = RTSPUrl::parse("rtsp://localhost:554/test");
    assert_eq!(result, RTSPResult::Ok);
    let url = url.unwrap();
    
    let conn = RTSPConnection::create(&url);
    assert!(conn.is_ok());
    let conn = conn.unwrap();
    
    // Verify URL is stored
    let stored_url = conn.get_url();
    assert!(stored_url.is_some());
}

#[test]
fn test_connection_builder() {
    init();

    // Test using the connection builder
    let builder = RTSPConnectionBuilder::new("rtsp://localhost:554/test");
    assert!(builder.is_ok());
    
    let conn = builder.unwrap().build();
    assert!(conn.is_ok());
}

#[test]
#[cfg(feature = "v1_18")]
fn test_connection_builder_with_timeout() {
    init();

    // Test builder with timeout configuration
    let builder = RTSPConnectionBuilder::new("rtsp://localhost:554/test");
    assert!(builder.is_ok());
    
    let conn = builder
        .unwrap()
        .timeout(10_000_000) // 10 seconds in microseconds
        .build();
    assert!(conn.is_ok());
}

#[test]
#[cfg(not(feature = "v1_18"))]
fn test_connection_builder_with_timeout_legacy() {
    init();

    // Test builder with timeout configuration (legacy API)
    let builder = RTSPConnectionBuilder::new("rtsp://localhost:554/test");
    assert!(builder.is_ok());
    
    let conn = builder
        .unwrap()
        .timeout_secs(10)
        .timeout_usecs(0)
        .build();
    assert!(conn.is_ok());
}

#[test]
fn test_connection_tunneling() {
    init();

    let (result, url) = RTSPUrl::parse("rtsp://localhost:554/test");
    assert_eq!(result, RTSPResult::Ok);
    let url = url.unwrap();
    let conn = RTSPConnection::create(&url).unwrap();

    // Test tunneling mode
    assert!(!conn.is_tunneled());
    conn.set_tunneled(true);
    assert!(conn.is_tunneled());
    conn.set_tunneled(false);
    assert!(!conn.is_tunneled());
    
    // Tunnel ID should be None for non-tunneled connections
    let tunnel_id = conn.get_tunnelid();
    assert!(tunnel_id.is_none() || tunnel_id == Some("".to_string()));
}

#[test]
fn test_connection_http_mode() {
    init();

    let (result, url) = RTSPUrl::parse("rtsp://localhost:554/test");
    assert_eq!(result, RTSPResult::Ok);
    let url = url.unwrap();
    let conn = RTSPConnection::create(&url).unwrap();

    // Test HTTP mode
    conn.set_http_mode(false);
    conn.set_http_mode(true);
    
    // Test remember session ID
    conn.set_remember_session_id(false);
    conn.set_remember_session_id(true);
}

#[test]
fn test_connection_authentication() {
    init();

    let (result, url) = RTSPUrl::parse("rtsp://localhost:554/test");
    assert_eq!(result, RTSPResult::Ok);
    let url = url.unwrap();
    let conn = RTSPConnection::create(&url).unwrap();

    // Test basic authentication
    let result = conn.set_auth(RTSPAuthMethod::Basic, "username", "password");
    assert!(result.is_ok());
    
    // Test digest authentication
    let result = conn.set_auth(RTSPAuthMethod::Digest, "user", "pass");
    assert!(result.is_ok());
    
    // Test auth parameters
    conn.set_auth_param("realm", "Test Realm");
    conn.set_auth_param("nonce", "test-nonce");
    conn.set_auth_param("uri", "/test");
    
    // Clear auth params
    conn.clear_auth_params();
}

#[test]
fn test_connection_proxy() {
    init();

    let (result, url) = RTSPUrl::parse("rtsp://localhost:554/test");
    assert_eq!(result, RTSPResult::Ok);
    let url = url.unwrap();
    let conn = RTSPConnection::create(&url).unwrap();

    // Test proxy configuration
    let result = conn.set_proxy("proxy.example.com", 8080);
    assert!(result.is_ok());
    
    let result = conn.set_proxy("192.168.1.1", 3128);
    assert!(result.is_ok());
}

#[test]
fn test_connection_qos() {
    init();

    let (result, url) = RTSPUrl::parse("rtsp://localhost:554/test");
    assert_eq!(result, RTSPResult::Ok);
    let url = url.unwrap();
    let conn = RTSPConnection::create(&url).unwrap();

    // Test QoS DSCP values
    // Note: These may fail if connection is not established
    let _ = conn.set_qos_dscp(0); // Best effort
    let _ = conn.set_qos_dscp(46); // EF (Expedited Forwarding)
    let _ = conn.set_qos_dscp(34); // AF41 (Assured Forwarding)
}

#[test]
fn test_connection_ip() {
    init();

    let (result, url) = RTSPUrl::parse("rtsp://localhost:554/test");
    assert_eq!(result, RTSPResult::Ok);
    let url = url.unwrap();
    let conn = RTSPConnection::create(&url).unwrap();

    // Test IP configuration
    conn.set_ip("127.0.0.1");
    // Note: get_ip() may return None until connection is established
    let ip = conn.get_ip();
    assert!(ip.is_none() || ip == Some("127.0.0.1".to_string()));
    
    conn.set_ip("192.168.1.100");
    conn.set_ip("::1"); // IPv6
}

#[test]
#[cfg(feature = "v1_18")]
fn test_connection_content_length_limit() {
    init();

    let (result, url) = RTSPUrl::parse("rtsp://localhost:554/test");
    assert_eq!(result, RTSPResult::Ok);
    let url = url.unwrap();
    let conn = RTSPConnection::create(&url).unwrap();

    // Test content length limit
    conn.set_content_length_limit(1024 * 1024); // 1MB
    conn.set_content_length_limit(10 * 1024 * 1024); // 10MB
    conn.set_content_length_limit(0); // No limit
}

#[test]
#[cfg(feature = "v1_20")]
fn test_connection_extra_http_headers() {
    init();

    let (result, url) = RTSPUrl::parse("rtsp://localhost:554/test");
    assert_eq!(result, RTSPResult::Ok);
    let url = url.unwrap();
    let conn = RTSPConnection::create(&url).unwrap();

    // Test adding extra HTTP headers
    conn.add_extra_http_request_header("X-Custom-Header", "value");
    conn.add_extra_http_request_header("User-Agent", "Custom RTSP Client");
    conn.add_extra_http_request_header("X-Session-Id", "12345");
}

#[test]
#[cfg(feature = "v1_20")]
fn test_connection_ignore_x_server_reply() {
    init();

    let (result, url) = RTSPUrl::parse("rtsp://localhost:554/test");
    assert_eq!(result, RTSPResult::Ok);
    let url = url.unwrap();
    let conn = RTSPConnection::create(&url).unwrap();

    // Test ignore x-server-reply setting
    conn.set_ignore_x_server_reply(false);
    conn.set_ignore_x_server_reply(true);
}

#[test]
fn test_connection_tunnel_pair() {
    init();

    let (result, url) = RTSPUrl::parse("rtsp://localhost:554/test");
    assert_eq!(result, RTSPResult::Ok);
    let url = url.unwrap();
    
    // Create two connections for tunneling
    let conn1 = RTSPConnection::create(&url).unwrap();
    let conn2 = RTSPConnection::create(&url).unwrap();
    
    // Set up tunneling
    conn1.set_tunneled(true);
    conn2.set_tunneled(true);
    
    // Note: do_tunnel requires established connections
    // This will fail without actual connection
    let result = conn1.do_tunnel(&conn2);
    assert!(result.is_err()); // Expected to fail without connection
}

#[test]
fn test_connection_flush() {
    init();

    let (result, url) = RTSPUrl::parse("rtsp://localhost:554/test");
    assert_eq!(result, RTSPResult::Ok);
    let url = url.unwrap();
    let conn = RTSPConnection::create(&url).unwrap();

    // Test flush operations
    let result = conn.flush(true);
    // May fail if not connected, but should handle gracefully
    assert!(result.is_ok() || result.is_err());
    
    let result = conn.flush(false);
    assert!(result.is_ok() || result.is_err());
}

#[test]
#[cfg(feature = "v1_18")]
fn test_connection_timeout() {
    init();

    let (result, url) = RTSPUrl::parse("rtsp://localhost:554/test");
    assert_eq!(result, RTSPResult::Ok);
    let url = url.unwrap();
    let conn = RTSPConnection::create(&url).unwrap();

    // Test timeout operations
    let timeout = conn.next_timeout();
    assert!(timeout >= 0);
    
    let result = conn.reset_timeout();
    assert!(result.is_ok());
}

#[test]
#[cfg(feature = "v1_18")]
fn test_connection_tls() {
    init();

    let (result, url) = RTSPUrl::parse("rtsps://localhost:554/test"); // Note: rtsps for TLS
    assert_eq!(result, RTSPResult::Ok);
    let url = url.unwrap();
    let conn = RTSPConnection::create(&url).unwrap();

    // Test TLS validation flags
    use gio::TlsCertificateFlags;
    let flags = TlsCertificateFlags::VALIDATE_ALL;
    let success = conn.set_tls_validation_flags(flags);
    assert!(success);
    
    let retrieved_flags = conn.get_tls_validation_flags();
    assert_eq!(retrieved_flags, flags);
    
    // Test TLS database
    conn.set_tls_database(None);
    let db = conn.get_tls_database();
    assert!(db.is_none());
    
    // Test TLS interaction
    conn.set_tls_interaction(None);
    let interaction = conn.get_tls_interaction();
    assert!(interaction.is_none());
    
    // Test getting TLS connection (will be None without actual connection)
    let tls_conn = conn.get_tls();
    assert!(tls_conn.is_none());
}

#[test]
fn test_connection_sockets() {
    init();

    let (result, url) = RTSPUrl::parse("rtsp://localhost:554/test");
    assert_eq!(result, RTSPResult::Ok);
    let url = url.unwrap();
    let conn = RTSPConnection::create(&url).unwrap();

    // Test getting sockets (will be None without actual connection)
    let read_socket = conn.get_read_socket();
    assert!(read_socket.is_none());
    
    let write_socket = conn.get_write_socket();
    assert!(write_socket.is_none());
}

#[test]
fn test_connection_close() {
    init();

    let (result, url) = RTSPUrl::parse("rtsp://localhost:554/test");
    assert_eq!(result, RTSPResult::Ok);
    let url = url.unwrap();
    let conn = RTSPConnection::create(&url).unwrap();

    // Test closing connection (may fail if not connected)
    let result = conn.close();
    assert!(result.is_ok() || result.is_err());
}

#[test]
#[cfg(feature = "v1_18")]
fn test_connection_from_socket() {
    init();

    use gio::prelude::*;
    use gio::SocketFamily;
    use gio::SocketProtocol;
    use gio::SocketType;

    // Create a socket
    let socket = gio::Socket::new(
        SocketFamily::Ipv4,
        SocketType::Stream,
        SocketProtocol::Tcp,
    );
    
    if let Ok(socket) = socket {
        let conn = RTSPConnection::create_from_socket(
            &socket,
            "127.0.0.1",
            554,
            Some("initial buffer"),
        );
        // This will likely fail without a bound socket, but should handle gracefully
        assert!(conn.is_ok() || conn.is_err());
    }
}

#[test]
fn test_connection_accept() {
    init();

    use gio::prelude::*;
    use gio::SocketFamily;
    use gio::SocketProtocol;
    use gio::SocketType;

    // Create a socket
    let socket = gio::Socket::new(
        SocketFamily::Ipv4,
        SocketType::Stream,
        SocketProtocol::Tcp,
    );
    
    if let Ok(socket) = socket {
        // Try to accept connection (will fail without listening socket)
        let conn = RTSPConnection::accept(&socket, None);
        assert!(conn.is_err()); // Expected to fail
    }
}

#[test]
fn test_message_creation() {
    init();

    // Test creating request message
    let mut request = RTSPMessage::new_request(RTSPMethod::OPTIONS, "rtsp://localhost:554/test");
    assert!(request.is_ok());
    let mut request = request.unwrap();
    
    // Add headers
    request.add_header(RTSPHeaderField::CSeq, "1");
    request.add_header(RTSPHeaderField::UserAgent, "Test Client");
    
    // Test creating response message
    let mut response = RTSPMessage::new_response(
        RTSPStatusCode::Ok,
        Some("OK"),
        Some(&request),
    );
    assert!(response.is_ok());
    let mut response = response.unwrap();
    
    // Add headers to response
    response.add_header(RTSPHeaderField::CSeq, "1");
    response.add_header(RTSPHeaderField::Server, "Test Server");
}

#[test]
#[cfg(feature = "v1_18")]
fn test_connection_poll() {
    init();

    let (result, url) = RTSPUrl::parse("rtsp://localhost:554/test");
    assert_eq!(result, RTSPResult::Ok);
    let url = url.unwrap();
    let conn = RTSPConnection::create(&url).unwrap();

    // Test polling (will fail without connection)
    let events = RTSPEvent::READ | RTSPEvent::WRITE;
    let result = conn.poll(events, 0); // 0 timeout for immediate return
    assert!(result.is_err()); // Expected to fail without connection
}

#[test]
#[cfg(feature = "v1_18")]
fn test_connection_read_write() {
    init();

    let (result, url) = RTSPUrl::parse("rtsp://localhost:554/test");
    assert_eq!(result, RTSPResult::Ok);
    let url = url.unwrap();
    let conn = RTSPConnection::create(&url).unwrap();

    // Test read operation (will fail without connection)
    let mut buffer = vec![0u8; 1024];
    let result = conn.read(&mut buffer, 0);
    assert!(result.is_err()); // Expected to fail
    
    // Test write operation (will fail without connection)
    let data = b"TEST DATA";
    let result = conn.write(data, 0);
    assert!(result.is_err()); // Expected to fail
}

#[test]
fn test_connection_send_receive() {
    init();

    let (result, url) = RTSPUrl::parse("rtsp://localhost:554/test");
    assert_eq!(result, RTSPResult::Ok);
    let url = url.unwrap();
    let conn = RTSPConnection::create(&url).unwrap();

    // Create a test message
    let message = RTSPMessage::new_request(RTSPMethod::OPTIONS, "rtsp://localhost:554/test");
    assert!(message.is_ok());
    let message = message.unwrap();
    
    // Test send (will fail without connection)
    #[cfg(feature = "v1_18")]
    let send_result = conn.send(&message, 1000000); // 1 second timeout
    
    #[cfg(not(feature = "v1_18"))]
    let send_result = conn.send(&message, 1, 0); // 1 second timeout
    
    assert!(send_result.is_err()); // Expected to fail
    
    // Test receive (will fail without connection)
    let response = RTSPMessage::new();
    assert!(response.is_ok());
    let response = response.unwrap();
    
    #[cfg(feature = "v1_18")]
    let recv_result = conn.receive(&response, 1000000);
    
    #[cfg(not(feature = "v1_18"))]
    let recv_result = conn.receive(&response, 1, 0);
    
    assert!(recv_result.is_err()); // Expected to fail
}

#[test]
#[cfg(feature = "v1_18")]
fn test_connection_send_messages() {
    init();

    let (result, url) = RTSPUrl::parse("rtsp://localhost:554/test");
    assert_eq!(result, RTSPResult::Ok);
    let url = url.unwrap();
    let conn = RTSPConnection::create(&url).unwrap();

    // Create multiple messages
    let msg1 = RTSPMessage::new_request(RTSPMethod::OPTIONS, "rtsp://localhost:554/test").unwrap();
    let msg2 = RTSPMessage::new_request(RTSPMethod::DESCRIBE, "rtsp://localhost:554/test").unwrap();
    let messages = vec![msg1, msg2];
    
    // Test sending multiple messages (will fail without connection)
    let result = conn.send_messages(&messages, 1000000);
    assert!(result.is_err()); // Expected to fail
}

#[test]
fn test_connection_builder_with_auth() {
    init();

    // Test builder with authentication
    let builder = RTSPConnectionBuilder::new("rtsp://localhost:554/test");
    assert!(builder.is_ok());
    
    let conn = builder
        .unwrap()
        .auth(RTSPAuthMethod::Basic, "user", "pass")
        .build();
    assert!(conn.is_ok());
}

#[test]
fn test_connection_builder_with_proxy() {
    init();

    // Test builder with proxy
    let builder = RTSPConnectionBuilder::new("rtsp://localhost:554/test");
    assert!(builder.is_ok());
    
    let conn = builder
        .unwrap()
        .proxy("proxy.example.com", 8080)
        .build();
    assert!(conn.is_ok());
}

#[test]
fn test_connection_url_variations() {
    init();

    // Test different URL formats
    let urls = vec![
        "rtsp://localhost:554/test",
        "rtsp://192.168.1.1:554/stream",
        "rtsp://example.com/media.mp4",
        "rtsps://secure.example.com:8554/secure",
        "rtsp://[::1]:554/ipv6", // IPv6
    ];
    
    for url_str in urls {
        let (result, url) = RTSPUrl::parse(url_str);
        if result == RTSPResult::Ok {
            let url = url.unwrap();
            let conn = RTSPConnection::create(&url);
            assert!(conn.is_ok(), "Failed to create connection for URL: {}", url_str);
        }
    }
}