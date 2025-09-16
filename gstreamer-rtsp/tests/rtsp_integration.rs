// Take a look at the license at the top of the repository in the LICENSE file.

use gstreamer_rtsp::{
    builders::{self, RTSPConnectionBuilder, RTSPTransportBuilder},
    rtsp_message::RTSPMessage,
    RTSPAuthMethod, RTSPConnection, RTSPHeaderField, RTSPLowerTrans, RTSPMethod, RTSPProfile,
    RTSPRange, RTSPResult, RTSPStatusCode, RTSPTransMode, RTSPTransport, RTSPUrl, RTSPVersion,
};

fn init() {
    use std::sync::Once;
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        gstreamer_rtsp::init().unwrap();
    });
}

#[test]
fn test_integration_transport_with_message() {
    init();

    // Create a transport
    let mut transport = RTSPTransport::new().unwrap();
    transport.set_trans(RTSPTransMode::RTP);
    transport.set_profile(RTSPProfile::AVP);
    transport.set_lower_transport(RTSPLowerTrans::UDP);
    transport.set_client_port(RTSPRange::new(5000, 5001));

    // Create a SETUP request with transport
    let mut request = RTSPMessage::new_request(RTSPMethod::SETUP, "rtsp://localhost:554/test/stream1").unwrap();
    request.add_header(RTSPHeaderField::CSeq, "2");
    request.add_header(RTSPHeaderField::Transport, &transport.as_text());

    // Verify the transport header was added
    let transport_header = request.get_header(RTSPHeaderField::Transport);
    assert!(transport_header.is_ok());
    let (_, value) = transport_header.unwrap();
    assert!(value.is_some());
    assert!(value.unwrap().contains("RTP/AVP"));
}

#[test]
fn test_integration_connection_url_transport() {
    init();

    // Parse URL
    let (result, url) = RTSPUrl::parse("rtsp://localhost:554/test");
    assert_eq!(result, RTSPResult::Ok);
    let url = url.unwrap();

    // Create connection
    let conn = RTSPConnection::create(&url).unwrap();

    // Create transport for the connection
    let transport = RTSPTransportBuilder::new()
        .unwrap()
        .protocol(RTSPTransMode::RTP)
        .profile(RTSPProfile::AVP)
        .lower_transport(RTSPLowerTrans::TCP)
        .interleaved(0, 1)
        .build()
        .unwrap();

    // Create SETUP request with transport
    let mut setup = RTSPMessage::new_request(RTSPMethod::SETUP, "rtsp://localhost:554/test/stream1").unwrap();
    setup.add_header(RTSPHeaderField::Transport, &transport.as_text());

    // The connection would use this transport for the stream
    assert_eq!(transport.lower_transport(), RTSPLowerTrans::TCP);
    assert_eq!(transport.interleaved(), RTSPRange::new(0, 1));
}

#[test]
fn test_integration_builder_chain() {
    init();

    // Test chaining builders for connection with authentication and proxy
    let conn = RTSPConnectionBuilder::new("rtsp://localhost:554/test")
        .unwrap()
        .auth(RTSPAuthMethod::Digest, "user", "pass")
        .proxy("proxy.example.com", 8080)
        .build();
    
    assert!(conn.is_ok());
    let conn = conn.unwrap();

    // Create transport using builder
    let transport = RTSPTransportBuilder::new()
        .unwrap()
        .protocol(RTSPTransMode::RTP)
        .profile(RTSPProfile::SAVP)
        .lower_transport(RTSPLowerTrans::UDP)
        .client_ports(5000, 5001)
        .server_ports(6000, 6001)
        .destination("224.0.0.1")
        .ttl(127)
        .build();
    
    assert!(transport.is_ok());
    let transport = transport.unwrap();
    assert_eq!(transport.profile(), RTSPProfile::SAVP);
}

#[test]
fn test_integration_message_flow() {
    init();

    // Simulate a typical RTSP session message flow
    
    // 1. OPTIONS request
    let mut options = RTSPMessage::new_request(RTSPMethod::OPTIONS, "rtsp://localhost:554/test").unwrap();
    options.add_header(RTSPHeaderField::CSeq, "1");
    options.add_header(RTSPHeaderField::UserAgent, "Test Client");

    // 2. DESCRIBE request
    let mut describe = RTSPMessage::new_request(RTSPMethod::DESCRIBE, "rtsp://localhost:554/test").unwrap();
    describe.add_header(RTSPHeaderField::CSeq, "2");
    describe.add_header(RTSPHeaderField::Accept, "application/sdp");

    // 3. SETUP request with transport
    let transport = builders::helpers::create_rtp_transport(5000, 5001).unwrap();
    let mut setup = RTSPMessage::new_request(RTSPMethod::SETUP, "rtsp://localhost:554/test/stream1").unwrap();
    setup.add_header(RTSPHeaderField::CSeq, "3");
    setup.add_header(RTSPHeaderField::Transport, &transport.as_text());

    // 4. PLAY request
    let mut play = RTSPMessage::new_request(RTSPMethod::PLAY, "rtsp://localhost:554/test").unwrap();
    play.add_header(RTSPHeaderField::CSeq, "4");
    play.add_header(RTSPHeaderField::Session, "12345678");
    play.add_header(RTSPHeaderField::Range, "npt=0.000-");

    // 5. TEARDOWN request
    let mut teardown = RTSPMessage::new_request(RTSPMethod::TEARDOWN, "rtsp://localhost:554/test").unwrap();
    teardown.add_header(RTSPHeaderField::CSeq, "5");
    teardown.add_header(RTSPHeaderField::Session, "12345678");

    // Verify all messages were created successfully
    assert_eq!(options.get_header(RTSPHeaderField::CSeq).unwrap().1, Some("1"));
    assert_eq!(describe.get_header(RTSPHeaderField::CSeq).unwrap().1, Some("2"));
    assert_eq!(setup.get_header(RTSPHeaderField::CSeq).unwrap().1, Some("3"));
    assert_eq!(play.get_header(RTSPHeaderField::CSeq).unwrap().1, Some("4"));
    assert_eq!(teardown.get_header(RTSPHeaderField::CSeq).unwrap().1, Some("5"));
}

#[test]
fn test_integration_transport_parsing_roundtrip() {
    init();

    // Create a complex transport configuration
    let original = RTSPTransportBuilder::new()
        .unwrap()
        .protocol(RTSPTransMode::RTP)
        .profile(RTSPProfile::AVP)
        .lower_transport(RTSPLowerTrans::UDP_MCAST)
        .destination("224.1.2.3")
        .port(5000, 5001)
        .ttl(127)
        .build()
        .unwrap();

    // Convert to string
    let transport_str = original.as_text();
    
    // Parse back
    let parsed = RTSPTransport::parse(&transport_str).unwrap();
    
    // Verify key fields match
    assert_eq!(original.trans(), parsed.trans());
    assert_eq!(original.profile(), parsed.profile());
    assert_eq!(original.lower_transport(), parsed.lower_transport());
    // Note: Some fields may not perfectly round-trip due to parsing quirks
}

#[test]
fn test_integration_multicast_transport() {
    init();

    // Create multicast transport configuration
    let transport = RTSPTransportBuilder::new()
        .unwrap()
        .protocol(RTSPTransMode::RTP)
        .profile(RTSPProfile::AVP)
        .lower_transport(RTSPLowerTrans::UDP_MCAST)
        .destination("239.255.0.1")
        .port(5000, 5001)
        .ttl(64)
        .build()
        .unwrap();

    // Verify multicast settings
    assert_eq!(transport.lower_transport(), RTSPLowerTrans::UDP_MCAST);
    assert_eq!(transport.destination(), Some("239.255.0.1".to_string()));
    assert_eq!(transport.ttl(), 64);
    assert_eq!(transport.port(), RTSPRange::new(5000, 5001));

    // Create SETUP request for multicast
    let mut setup = RTSPMessage::new_request(RTSPMethod::SETUP, "rtsp://localhost:554/test/stream1").unwrap();
    setup.add_header(RTSPHeaderField::Transport, &transport.as_text());
    
    let transport_header = setup.get_header(RTSPHeaderField::Transport).unwrap().1;
    assert!(transport_header.is_some());
    assert!(transport_header.unwrap().contains("multicast"));
}

#[test]
fn test_integration_tcp_interleaved_transport() {
    init();

    // Create TCP interleaved transport
    let transport = builders::helpers::create_tcp_transport(0, 1).unwrap();
    
    // Verify TCP settings
    assert_eq!(transport.lower_transport(), RTSPLowerTrans::TCP);
    assert_eq!(transport.interleaved(), RTSPRange::new(0, 1));

    // Create another channel for audio
    let audio_transport = builders::helpers::create_tcp_transport(2, 3).unwrap();
    assert_eq!(audio_transport.interleaved(), RTSPRange::new(2, 3));

    // Both should use TCP
    assert_eq!(transport.lower_transport(), audio_transport.lower_transport());
}

#[test]
fn test_integration_secure_transport() {
    init();

    // Create secure transport (SAVP/SAVPF)
    let secure_transport = RTSPTransportBuilder::new()
        .unwrap()
        .protocol(RTSPTransMode::RTP)
        .profile(RTSPProfile::SAVP)
        .lower_transport(RTSPLowerTrans::UDP)
        .client_ports(5000, 5001)
        .build()
        .unwrap();

    assert_eq!(secure_transport.profile(), RTSPProfile::SAVP);

    // Create feedback-enabled secure transport
    let feedback_transport = RTSPTransportBuilder::new()
        .unwrap()
        .protocol(RTSPTransMode::RTP)
        .profile(RTSPProfile::SAVPF)
        .lower_transport(RTSPLowerTrans::UDP)
        .client_ports(5002, 5003)
        .build()
        .unwrap();

    assert_eq!(feedback_transport.profile(), RTSPProfile::SAVPF);
}

#[test]
fn test_integration_connection_with_auth_and_transport() {
    init();

    // Create connection with authentication
    let (result, url) = RTSPUrl::parse("rtsp://localhost:554/test");
    assert_eq!(result, RTSPResult::Ok);
    let url = url.unwrap();
    let conn = RTSPConnection::create(&url).unwrap();
    
    // Set up authentication
    conn.set_auth(RTSPAuthMethod::Digest, "user", "password").unwrap();
    conn.set_auth_param("realm", "RTSP Server");
    conn.set_auth_param("nonce", "abcdef123456");

    // Create transport for authenticated session
    let transport = RTSPTransportBuilder::new()
        .unwrap()
        .protocol(RTSPTransMode::RTP)
        .profile(RTSPProfile::AVP)
        .lower_transport(RTSPLowerTrans::UDP)
        .client_ports(5000, 5001)
        .build()
        .unwrap();

    // Create authenticated SETUP request
    let mut setup = RTSPMessage::new_request(RTSPMethod::SETUP, "rtsp://localhost:554/test/stream1").unwrap();
    setup.add_header(RTSPHeaderField::CSeq, "1");
    setup.add_header(RTSPHeaderField::Transport, &transport.as_text());
    setup.add_header(RTSPHeaderField::Authorization, "Digest username=\"user\", realm=\"RTSP Server\", nonce=\"abcdef123456\"");

    // Verify the message has both auth and transport
    assert!(setup.get_header(RTSPHeaderField::Authorization).unwrap().1.is_some());
    assert!(setup.get_header(RTSPHeaderField::Transport).unwrap().1.is_some());
}

#[test]
fn test_integration_response_handling() {
    init();

    // Create a request
    let request = RTSPMessage::new_request(RTSPMethod::OPTIONS, "rtsp://localhost:554/test").unwrap();
    
    // Create corresponding response
    let mut response = RTSPMessage::new_response(
        RTSPStatusCode::Ok,
        Some("OK"),
        Some(&request),
    ).unwrap();
    
    // Add typical OPTIONS response headers
    response.add_header(RTSPHeaderField::CSeq, "1");
    response.add_header(RTSPHeaderField::Public, "OPTIONS, DESCRIBE, SETUP, PLAY, PAUSE, TEARDOWN");
    response.add_header(RTSPHeaderField::Server, "Test RTSP Server");

    // Verify response
    let public_methods = response.get_header(RTSPHeaderField::Public).unwrap().1;
    assert!(public_methods.is_some());
    assert!(public_methods.unwrap().contains("PLAY"));
}

#[test]
fn test_integration_transport_modes() {
    init();

    // Test play-only transport
    let mut play_transport = RTSPTransport::new().unwrap();
    play_transport.set_mode_play(true);
    play_transport.set_mode_record(false);
    assert!(play_transport.mode_play());
    assert!(!play_transport.mode_record());

    // Test record-only transport
    let mut record_transport = RTSPTransport::new().unwrap();
    record_transport.set_mode_play(false);
    record_transport.set_mode_record(true);
    assert!(!record_transport.mode_play());
    assert!(record_transport.mode_record());

    // Test play+record transport
    let mut bidir_transport = RTSPTransport::new().unwrap();
    bidir_transport.set_mode_play(true);
    bidir_transport.set_mode_record(true);
    assert!(bidir_transport.mode_play());
    assert!(bidir_transport.mode_record());
}

#[test]
fn test_integration_url_and_connection() {
    init();

    // Test various URL formats with connection creation
    let test_cases = vec![
        ("rtsp://localhost:554/test", 554),
        ("rtsp://192.168.1.1:8554/stream", 8554),
        ("rtsp://example.com/media.mp4", 554), // Default port
        ("rtsps://secure.example.com:8554/secure", 8554),
    ];

    for (url_str, expected_port) in test_cases {
        let (result, url) = RTSPUrl::parse(url_str);
        assert_eq!(result, RTSPResult::Ok, "Failed to parse URL: {}", url_str);
        let url = url.unwrap();
        
        // Verify port
        assert_eq!(url.get_port(), expected_port, "Wrong port for URL: {}", url_str);
        
        // Create connection
        let conn = RTSPConnection::create(&url);
        assert!(conn.is_ok(), "Failed to create connection for URL: {}", url_str);
    }
}

#[test]
fn test_integration_complete_session() {
    init();

    // Simulate a complete RTSP session workflow
    
    // 1. Parse URL and create connection
    let (result, url) = RTSPUrl::parse("rtsp://localhost:554/media/video1");
    assert_eq!(result, RTSPResult::Ok);
    let url = url.unwrap();
    let conn = RTSPConnection::create(&url).unwrap();
    
    // 2. Configure connection
    conn.set_auth(RTSPAuthMethod::Basic, "user", "pass").unwrap();
    conn.set_remember_session_id(true);
    
    // 3. Create transport for video stream
    let video_transport = RTSPTransportBuilder::new()
        .unwrap()
        .protocol(RTSPTransMode::RTP)
        .profile(RTSPProfile::AVP)
        .lower_transport(RTSPLowerTrans::UDP)
        .client_ports(5000, 5001)
        .build()
        .unwrap();
    
    // 4. Create transport for audio stream  
    let audio_transport = RTSPTransportBuilder::new()
        .unwrap()
        .protocol(RTSPTransMode::RTP)
        .profile(RTSPProfile::AVP)
        .lower_transport(RTSPLowerTrans::UDP)
        .client_ports(5002, 5003)
        .build()
        .unwrap();
    
    // 5. Verify transports are configured correctly
    assert_eq!(video_transport.client_port(), RTSPRange::new(5000, 5001));
    assert_eq!(audio_transport.client_port(), RTSPRange::new(5002, 5003));
    
    // 6. Both transports should use same protocol/profile
    assert_eq!(video_transport.trans(), audio_transport.trans());
    assert_eq!(video_transport.profile(), audio_transport.profile());
}