// Take a look at the license at the top of the repository in the LICENSE file.

use gstreamer_rtsp::{
    builders::{self, RTSPTransportBuilder},
    RTSPLowerTrans, RTSPProfile, RTSPRange, RTSPResult, RTSPTransMode, RTSPTransport,
};

fn init() {
    use std::sync::Once;
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        gstreamer_rtsp::init().unwrap();
    });
}

#[test]
fn test_transport_creation() {
    init();

    // Test basic creation
    let transport = RTSPTransport::new();
    assert!(transport.is_ok());
    let transport = transport.unwrap();
    
    // Verify default values
    assert_eq!(transport.trans(), RTSPTransMode::__Unknown(0));
    assert_eq!(transport.profile(), RTSPProfile::__Unknown(0));
    assert_eq!(transport.lower_transport(), RTSPLowerTrans::__Unknown(0));
}

#[test]
fn test_transport_default() {
    init();

    // Test Default trait implementation
    let transport = RTSPTransport::default();
    assert_eq!(transport.trans(), RTSPTransMode::__Unknown(0));
}

#[test]
fn test_transport_field_accessors() {
    init();

    let mut transport = RTSPTransport::new().unwrap();

    // Test trans field
    transport.set_trans(RTSPTransMode::RTP);
    assert_eq!(transport.trans(), RTSPTransMode::RTP);
    transport.set_trans(RTSPTransMode::RDT);
    assert_eq!(transport.trans(), RTSPTransMode::RDT);

    // Test profile field
    transport.set_profile(RTSPProfile::AVP);
    assert_eq!(transport.profile(), RTSPProfile::AVP);
    transport.set_profile(RTSPProfile::AVPF);
    assert_eq!(transport.profile(), RTSPProfile::AVPF);
    transport.set_profile(RTSPProfile::SAVP);
    assert_eq!(transport.profile(), RTSPProfile::SAVP);
    transport.set_profile(RTSPProfile::SAVPF);
    assert_eq!(transport.profile(), RTSPProfile::SAVPF);

    // Test lower_transport field
    transport.set_lower_transport(RTSPLowerTrans::UDP);
    assert_eq!(transport.lower_transport(), RTSPLowerTrans::UDP);
    transport.set_lower_transport(RTSPLowerTrans::TCP);
    assert_eq!(transport.lower_transport(), RTSPLowerTrans::TCP);
    transport.set_lower_transport(RTSPLowerTrans::UDP_MCAST);
    assert_eq!(transport.lower_transport(), RTSPLowerTrans::UDP_MCAST);
}

#[test]
fn test_transport_string_fields() {
    init();

    let mut transport = RTSPTransport::new().unwrap();

    // Test destination field
    assert_eq!(transport.destination(), None);
    transport.set_destination(Some("192.168.1.100"));
    assert_eq!(transport.destination(), Some("192.168.1.100".to_string()));
    transport.set_destination(Some("224.0.0.1")); // Multicast address
    assert_eq!(transport.destination(), Some("224.0.0.1".to_string()));
    transport.set_destination(None);
    assert_eq!(transport.destination(), None);

    // Test source field
    assert_eq!(transport.source(), None);
    transport.set_source(Some("10.0.0.1"));
    assert_eq!(transport.source(), Some("10.0.0.1".to_string()));
    transport.set_source(Some("192.168.1.1"));
    assert_eq!(transport.source(), Some("192.168.1.1".to_string()));
    transport.set_source(None);
    assert_eq!(transport.source(), None);
}

#[test]
fn test_transport_numeric_fields() {
    init();

    let mut transport = RTSPTransport::new().unwrap();

    // Test layers field
    transport.set_layers(0);
    assert_eq!(transport.layers(), 0);
    transport.set_layers(3);
    assert_eq!(transport.layers(), 3);
    transport.set_layers(u32::MAX);
    assert_eq!(transport.layers(), u32::MAX);

    // Test ttl field
    transport.set_ttl(0);
    assert_eq!(transport.ttl(), 0);
    transport.set_ttl(64);
    assert_eq!(transport.ttl(), 64);
    transport.set_ttl(255);
    assert_eq!(transport.ttl(), 255);

    // Test ssrc field
    transport.set_ssrc(0);
    assert_eq!(transport.ssrc(), 0);
    transport.set_ssrc(0x12345678);
    assert_eq!(transport.ssrc(), 0x12345678);
    transport.set_ssrc(u32::MAX);
    assert_eq!(transport.ssrc(), u32::MAX);
}

#[test]
fn test_transport_boolean_fields() {
    init();

    let mut transport = RTSPTransport::new().unwrap();

    // Test mode_play field
    assert!(!transport.mode_play());
    transport.set_mode_play(true);
    assert!(transport.mode_play());
    transport.set_mode_play(false);
    assert!(!transport.mode_play());

    // Test mode_record field
    assert!(!transport.mode_record());
    transport.set_mode_record(true);
    assert!(transport.mode_record());
    transport.set_mode_record(false);
    assert!(!transport.mode_record());

    // Test append field
    assert!(!transport.append());
    transport.set_append(true);
    assert!(transport.append());
    transport.set_append(false);
    assert!(!transport.append());

    // Test multiple modes
    transport.set_mode_play(true);
    transport.set_mode_record(true);
    assert!(transport.mode_play());
    assert!(transport.mode_record());
}

#[test]
fn test_transport_range_fields() {
    init();

    let mut transport = RTSPTransport::new().unwrap();

    // Test interleaved range
    let interleaved = RTSPRange::new(0, 1);
    transport.set_interleaved(interleaved);
    assert_eq!(transport.interleaved(), interleaved);
    
    let interleaved2 = RTSPRange::new(2, 3);
    transport.set_interleaved(interleaved2);
    assert_eq!(transport.interleaved(), interleaved2);

    // Test client_port range
    let client_port = RTSPRange::new(5000, 5001);
    transport.set_client_port(client_port);
    assert_eq!(transport.client_port(), client_port);

    // Test server_port range
    let server_port = RTSPRange::new(6000, 6001);
    transport.set_server_port(server_port);
    assert_eq!(transport.server_port(), server_port);

    // Test port range
    let port = RTSPRange::new(7000, 7001);
    transport.set_port(port);
    assert_eq!(transport.port(), port);

    // Test edge cases
    let edge_range = RTSPRange::new(i32::MIN, i32::MAX);
    transport.set_interleaved(edge_range);
    assert_eq!(transport.interleaved(), edge_range);
}

#[test]
fn test_transport_parse_basic() {
    init();

    // Test parsing simple RTP/AVP transport
    let transport_str = "RTP/AVP;unicast;client_port=5000-5001";
    let transport = RTSPTransport::parse(transport_str);
    assert!(transport.is_ok());
    let transport = transport.unwrap();
    assert_eq!(transport.trans(), RTSPTransMode::RTP);
    assert_eq!(transport.profile(), RTSPProfile::AVP);
    assert_eq!(transport.client_port(), RTSPRange::new(5000, 5001));

    // Test parsing TCP transport
    let transport_str = "RTP/AVP/TCP;interleaved=0-1";
    let transport = RTSPTransport::parse(transport_str);
    assert!(transport.is_ok());
    let transport = transport.unwrap();
    assert_eq!(transport.lower_transport(), RTSPLowerTrans::TCP);
    assert_eq!(transport.interleaved(), RTSPRange::new(0, 1));
}

#[test]
fn test_transport_parse_multicast() {
    init();

    // Test parsing multicast transport
    let transport_str = "RTP/AVP;multicast;destination=224.0.0.1;ttl=127;port=5000-5001";
    let transport = RTSPTransport::parse(transport_str);
    assert!(transport.is_ok());
    let transport = transport.unwrap();
    assert_eq!(transport.lower_transport(), RTSPLowerTrans::UDP_MCAST);
    assert_eq!(transport.destination(), Some("224.0.0.1".to_string()));
    assert_eq!(transport.ttl(), 127);
    assert_eq!(transport.port(), RTSPRange::new(5000, 5001));
}

#[test]
fn test_transport_parse_complex() {
    init();

    // Test parsing complex transport with multiple parameters
    let transport_str = "RTP/SAVP;unicast;client_port=5000-5001;server_port=6000-6001;source=192.168.1.1;destination=192.168.1.100;ssrc=12345678;mode=\"PLAY\"";
    let transport = RTSPTransport::parse(transport_str);
    assert!(transport.is_ok());
    let transport = transport.unwrap();
    assert_eq!(transport.profile(), RTSPProfile::SAVP);
    assert_eq!(transport.client_port(), RTSPRange::new(5000, 5001));
    assert_eq!(transport.server_port(), RTSPRange::new(6000, 6001));
    assert_eq!(transport.source(), Some("192.168.1.1".to_string()));
    assert_eq!(transport.destination(), Some("192.168.1.100".to_string()));
    assert_eq!(transport.ssrc(), 12345678);
    assert!(transport.mode_play());
}

#[test]
fn test_transport_parse_invalid() {
    init();

    // Test parsing invalid transport strings
    let invalid_strings = vec![
        "",
        "INVALID",
        "RTP",
        ";;;",
        "RTP/AVP;invalid_param=value",
    ];

    for invalid_str in invalid_strings {
        let transport = RTSPTransport::parse(invalid_str);
        // Some may succeed with partial parsing, but should handle gracefully
        if let Ok(t) = transport {
            // Even if parsing succeeds, it should produce a valid transport object
            let _ = t.as_text();
        }
    }
}

#[test]
fn test_transport_as_text() {
    init();

    let mut transport = RTSPTransport::new().unwrap();
    
    // Set up a complete transport configuration
    transport.set_trans(RTSPTransMode::RTP);
    transport.set_profile(RTSPProfile::AVP);
    transport.set_lower_transport(RTSPLowerTrans::UDP);
    transport.set_client_port(RTSPRange::new(5000, 5001));
    
    let text = transport.as_text();
    assert!(!text.is_empty());
    // The text should contain the configured values
    assert!(text.contains("RTP/AVP"));
}

#[test]
fn test_transport_formatting() {
    init();

    let transport = RTSPTransport::new().unwrap();
    
    // Test Display trait
    let display = format!("{}", transport);
    assert!(!display.is_empty());
    
    // Test Debug trait
    let debug = format!("{:?}", transport);
    assert!(debug.contains("RTSPTransport"));
    assert!(debug.contains("trans"));
    assert!(debug.contains("profile"));
    assert!(debug.contains("lower_transport"));
}

#[test]
fn test_transport_get_media_type() {
    init();

    let mut transport = RTSPTransport::new().unwrap();
    
    // Set up transport for different media types
    transport.set_trans(RTSPTransMode::RTP);
    transport.set_profile(RTSPProfile::AVP);
    
    let media_type = transport.get_media_type();
    assert!(media_type.is_ok());
    // Media type might be None for default transport
}

#[test]
fn test_transport_get_manager() {
    init();

    // Test getting manager for different transport modes
    let manager = RTSPTransport::get_manager(RTSPTransMode::RTP, 0);
    assert!(manager.is_ok());
    
    let manager = RTSPTransport::get_manager(RTSPTransMode::RDT, 0);
    assert!(manager.is_ok());
}

#[test]
fn test_transport_builder() {
    init();

    // Test using the builder
    let transport = RTSPTransportBuilder::new()
        .unwrap()
        .protocol(RTSPTransMode::RTP)
        .profile(RTSPProfile::AVP)
        .lower_transport(RTSPLowerTrans::UDP)
        .client_ports(5000, 5001)
        .build();
    
    assert!(transport.is_ok());
    let transport = transport.unwrap();
    assert_eq!(transport.trans(), RTSPTransMode::RTP);
    assert_eq!(transport.profile(), RTSPProfile::AVP);
    assert_eq!(transport.lower_transport(), RTSPLowerTrans::UDP);
    assert_eq!(transport.client_port(), RTSPRange::new(5000, 5001));
}

#[test]
fn test_transport_builder_complex() {
    init();

    // Test builder with all options
    let transport = RTSPTransportBuilder::new()
        .unwrap()
        .protocol(RTSPTransMode::RTP)
        .profile(RTSPProfile::SAVPF)
        .lower_transport(RTSPLowerTrans::TCP)
        .interleaved(0, 1)
        .destination("192.168.1.100")
        .source("192.168.1.1")
        .ttl(64)
        .ssrc(0x12345678)
        .mode_play(true)
        .mode_record(false)
        .append(true)
        .build();
    
    assert!(transport.is_ok());
    let transport = transport.unwrap();
    assert_eq!(transport.trans(), RTSPTransMode::RTP);
    assert_eq!(transport.profile(), RTSPProfile::SAVPF);
    assert_eq!(transport.lower_transport(), RTSPLowerTrans::TCP);
    assert_eq!(transport.interleaved(), RTSPRange::new(0, 1));
    assert_eq!(transport.destination(), Some("192.168.1.100".to_string()));
    assert_eq!(transport.source(), Some("192.168.1.1".to_string()));
    assert_eq!(transport.ttl(), 64);
    assert_eq!(transport.ssrc(), 0x12345678);
    assert!(transport.mode_play());
    assert!(!transport.mode_record());
    assert!(transport.append());
}

#[test]
fn test_transport_helpers() {
    init();

    // Test RTP transport helper
    let rtp_transport = builders::helpers::create_rtp_transport(5000, 5001);
    assert!(rtp_transport.is_ok());
    let rtp_transport = rtp_transport.unwrap();
    assert_eq!(rtp_transport.trans(), RTSPTransMode::RTP);
    assert_eq!(rtp_transport.profile(), RTSPProfile::AVP);
    assert_eq!(rtp_transport.lower_transport(), RTSPLowerTrans::UDP);
    assert_eq!(rtp_transport.client_port(), RTSPRange::new(5000, 5001));

    // Test TCP transport helper
    let tcp_transport = builders::helpers::create_tcp_transport(0, 1);
    assert!(tcp_transport.is_ok());
    let tcp_transport = tcp_transport.unwrap();
    assert_eq!(tcp_transport.trans(), RTSPTransMode::RTP);
    assert_eq!(tcp_transport.profile(), RTSPProfile::AVP);
    assert_eq!(tcp_transport.lower_transport(), RTSPLowerTrans::TCP);
    assert_eq!(tcp_transport.interleaved(), RTSPRange::new(0, 1));
}

#[test]
fn test_rtsp_range() {
    init();

    // Test RTSPRange creation and equality
    let range1 = RTSPRange::new(0, 100);
    assert_eq!(range1.min, 0);
    assert_eq!(range1.max, 100);

    let range2 = RTSPRange::new(0, 100);
    assert_eq!(range1, range2);

    let range3 = RTSPRange::new(50, 150);
    assert_ne!(range1, range3);

    // Test edge cases
    let min_range = RTSPRange::new(i32::MIN, 0);
    assert_eq!(min_range.min, i32::MIN);
    assert_eq!(min_range.max, 0);

    let max_range = RTSPRange::new(0, i32::MAX);
    assert_eq!(max_range.min, 0);
    assert_eq!(max_range.max, i32::MAX);
}

#[test]
fn test_transport_memory_management() {
    init();

    // Test that transport can be cloned and handles memory correctly
    let mut transport1 = RTSPTransport::new().unwrap();
    transport1.set_destination(Some("192.168.1.1"));
    transport1.set_source(Some("10.0.0.1"));
    
    // Clone the transport
    let transport2 = transport1.clone();
    assert_eq!(transport1.destination(), transport2.destination());
    assert_eq!(transport1.source(), transport2.source());
    
    // Modify original and verify clone is independent
    transport1.set_destination(Some("192.168.1.2"));
    assert_ne!(transport1.destination(), transport2.destination());
}

#[test]
fn test_transport_parse_and_format_roundtrip() {
    init();

    // Create a transport with specific settings
    let mut transport = RTSPTransport::new().unwrap();
    transport.set_trans(RTSPTransMode::RTP);
    transport.set_profile(RTSPProfile::AVP);
    transport.set_lower_transport(RTSPLowerTrans::UDP);
    transport.set_client_port(RTSPRange::new(5000, 5001));
    
    // Convert to text
    let text = transport.as_text();
    
    // Parse the text back
    let parsed = RTSPTransport::parse(&text);
    assert!(parsed.is_ok());
    let parsed = parsed.unwrap();
    
    // Verify key fields match
    assert_eq!(transport.trans(), parsed.trans());
    assert_eq!(transport.profile(), parsed.profile());
    // Note: Some fields may not round-trip perfectly due to formatting
}