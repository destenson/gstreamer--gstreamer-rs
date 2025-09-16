// Take a look at the license at the top of the repository in the LICENSE file.

// Transport Configuration Example
// This example demonstrates various RTSP transport configurations including
// UDP, TCP, multicast, and secure transports.

use gstreamer_rtsp::{
    builders::{helpers, RTSPTransportBuilder},
    RTSPLowerTrans, RTSPProfile, RTSPRange, RTSPTransMode, RTSPTransport,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize GStreamer
    gstreamer_rtsp::init()?;

    println!("RTSP Transport Configuration Examples");
    println!("=====================================\n");

    // Basic transport configurations
    create_udp_transport()?;
    create_tcp_transport()?;
    create_multicast_transport()?;
    
    // Secure transport configurations
    create_secure_transport()?;
    create_secure_feedback_transport()?;
    
    // Helper functions
    use_transport_helpers()?;
    
    // Parse transport strings
    parse_transport_strings()?;
    
    // Advanced configurations
    create_complex_transport()?;
    
    Ok(())
}

fn create_udp_transport() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- UDP Transport Configuration ---");
    
    let transport = RTSPTransportBuilder::new()?
        .protocol(RTSPTransMode::RTP)
        .profile(RTSPProfile::AVP)
        .lower_transport(RTSPLowerTrans::UDP)
        .client_ports(5000, 5001)
        .server_ports(6000, 6001)
        .build()?;
    
    println!("✓ Created UDP transport");
    println!("  Protocol: {:?}", transport.trans());
    println!("  Profile: {:?}", transport.profile());
    println!("  Lower Transport: {:?}", transport.lower_transport());
    println!("  Client Ports: {}-{}", transport.client_port().min, transport.client_port().max);
    println!("  Server Ports: {}-{}", transport.server_port().min, transport.server_port().max);
    println!("  Transport String: {}", transport.as_text());
    println!();
    
    Ok(())
}

fn create_tcp_transport() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- TCP Transport Configuration ---");
    
    let transport = RTSPTransportBuilder::new()?
        .protocol(RTSPTransMode::RTP)
        .profile(RTSPProfile::AVP)
        .lower_transport(RTSPLowerTrans::TCP)
        .interleaved(0, 1)  // RTP on channel 0, RTCP on channel 1
        .build()?;
    
    println!("✓ Created TCP transport");
    println!("  Protocol: {:?}", transport.trans());
    println!("  Profile: {:?}", transport.profile());
    println!("  Lower Transport: {:?}", transport.lower_transport());
    println!("  Interleaved Channels: {}-{}", transport.interleaved().min, transport.interleaved().max);
    println!("  Transport String: {}", transport.as_text());
    println!();
    
    // Create another TCP transport for audio stream
    let audio_transport = RTSPTransportBuilder::new()?
        .protocol(RTSPTransMode::RTP)
        .profile(RTSPProfile::AVP)
        .lower_transport(RTSPLowerTrans::TCP)
        .interleaved(2, 3)  // Audio RTP on channel 2, RTCP on channel 3
        .build()?;
    
    println!("✓ Created TCP audio transport");
    println!("  Interleaved Channels: {}-{}", audio_transport.interleaved().min, audio_transport.interleaved().max);
    println!();
    
    Ok(())
}

fn create_multicast_transport() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Multicast Transport Configuration ---");
    
    let transport = RTSPTransportBuilder::new()?
        .protocol(RTSPTransMode::RTP)
        .profile(RTSPProfile::AVP)
        .lower_transport(RTSPLowerTrans::UDP_MCAST)
        .destination("239.255.0.1")  // Multicast address
        .port(5000, 5001)
        .ttl(127)  // Time-to-live for multicast
        .build()?;
    
    println!("✓ Created multicast transport");
    println!("  Protocol: {:?}", transport.trans());
    println!("  Profile: {:?}", transport.profile());
    println!("  Lower Transport: {:?}", transport.lower_transport());
    println!("  Destination: {:?}", transport.destination());
    println!("  Port Range: {}-{}", transport.port().min, transport.port().max);
    println!("  TTL: {}", transport.ttl());
    println!("  Transport String: {}", transport.as_text());
    println!();
    
    Ok(())
}

fn create_secure_transport() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Secure Transport Configuration (SAVP) ---");
    
    let transport = RTSPTransportBuilder::new()?
        .protocol(RTSPTransMode::RTP)
        .profile(RTSPProfile::SAVP)  // Secure AVP
        .lower_transport(RTSPLowerTrans::UDP)
        .client_ports(5000, 5001)
        .build()?;
    
    println!("✓ Created secure transport");
    println!("  Protocol: {:?}", transport.trans());
    println!("  Profile: {:?} (Secure Audio Video Profile)", transport.profile());
    println!("  Lower Transport: {:?}", transport.lower_transport());
    println!("  Client Ports: {}-{}", transport.client_port().min, transport.client_port().max);
    println!("  Transport String: {}", transport.as_text());
    println!();
    
    Ok(())
}

fn create_secure_feedback_transport() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Secure Feedback Transport Configuration (SAVPF) ---");
    
    let transport = RTSPTransportBuilder::new()?
        .protocol(RTSPTransMode::RTP)
        .profile(RTSPProfile::SAVPF)  // Secure AVP with Feedback
        .lower_transport(RTSPLowerTrans::UDP)
        .client_ports(5002, 5003)
        .build()?;
    
    println!("✓ Created secure feedback transport");
    println!("  Protocol: {:?}", transport.trans());
    println!("  Profile: {:?} (Secure AVP with Feedback)", transport.profile());
    println!("  Lower Transport: {:?}", transport.lower_transport());
    println!("  Client Ports: {}-{}", transport.client_port().min, transport.client_port().max);
    println!("  Transport String: {}", transport.as_text());
    println!();
    
    Ok(())
}

fn use_transport_helpers() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Using Transport Helper Functions ---");
    
    // Create RTP transport using helper
    let rtp_transport = helpers::create_rtp_transport(5000, 5001)?;
    println!("✓ Created RTP transport using helper");
    println!("  Protocol: {:?}", rtp_transport.trans());
    println!("  Profile: {:?}", rtp_transport.profile());
    println!("  Lower Transport: {:?}", rtp_transport.lower_transport());
    println!("  Client Ports: {}-{}", rtp_transport.client_port().min, rtp_transport.client_port().max);
    println!();
    
    // Create TCP transport using helper
    let tcp_transport = helpers::create_tcp_transport(0, 1)?;
    println!("✓ Created TCP transport using helper");
    println!("  Protocol: {:?}", tcp_transport.trans());
    println!("  Profile: {:?}", tcp_transport.profile());
    println!("  Lower Transport: {:?}", tcp_transport.lower_transport());
    println!("  Interleaved: {}-{}", tcp_transport.interleaved().min, tcp_transport.interleaved().max);
    println!();
    
    Ok(())
}

fn parse_transport_strings() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Parsing Transport Strings ---");
    
    let transport_strings = vec![
        "RTP/AVP;unicast;client_port=5000-5001",
        "RTP/AVP/TCP;interleaved=0-1",
        "RTP/AVP;multicast;destination=224.0.0.1;ttl=127;port=5000-5001",
        "RTP/SAVP;unicast;client_port=5000-5001",
        "RTP/AVPF;unicast;client_port=5002-5003",
    ];
    
    for transport_str in transport_strings {
        println!("\nParsing: \"{}\"", transport_str);
        match RTSPTransport::parse(transport_str) {
            Ok(transport) => {
                println!("  ✓ Successfully parsed");
                println!("    Protocol: {:?}", transport.trans());
                println!("    Profile: {:?}", transport.profile());
                println!("    Lower Transport: {:?}", transport.lower_transport());
                
                // Print relevant fields based on transport type
                if transport.lower_transport() == RTSPLowerTrans::TCP {
                    println!("    Interleaved: {}-{}", transport.interleaved().min, transport.interleaved().max);
                } else if transport.lower_transport() == RTSPLowerTrans::UDP_MCAST {
                    if let Some(dest) = transport.destination() {
                        println!("    Destination: {}", dest);
                    }
                    println!("    TTL: {}", transport.ttl());
                } else {
                    let client_port = transport.client_port();
                    if client_port.min != -1 && client_port.max != -1 {
                        println!("    Client Ports: {}-{}", client_port.min, client_port.max);
                    }
                }
            }
            Err(e) => {
                println!("  ✗ Failed to parse: {:?}", e);
            }
        }
    }
    println!();
    
    Ok(())
}

fn create_complex_transport() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Complex Transport Configuration ---");
    
    let mut transport = RTSPTransportBuilder::new()?
        .protocol(RTSPTransMode::RTP)
        .profile(RTSPProfile::AVP)
        .lower_transport(RTSPLowerTrans::UDP)
        .client_ports(5000, 5001)
        .server_ports(6000, 6001)
        .destination("192.168.1.100")
        .source("192.168.1.1")
        .ssrc(0x12345678)
        .mode_play(true)
        .mode_record(false)
        .append(false)
        .layers(2)
        .build()?;
    
    println!("✓ Created complex transport configuration");
    println!("  Protocol: {:?}", transport.trans());
    println!("  Profile: {:?}", transport.profile());
    println!("  Lower Transport: {:?}", transport.lower_transport());
    println!("  Client Ports: {}-{}", transport.client_port().min, transport.client_port().max);
    println!("  Server Ports: {}-{}", transport.server_port().min, transport.server_port().max);
    println!("  Destination: {:?}", transport.destination());
    println!("  Source: {:?}", transport.source());
    println!("  SSRC: 0x{:08X}", transport.ssrc());
    println!("  Mode Play: {}", transport.mode_play());
    println!("  Mode Record: {}", transport.mode_record());
    println!("  Append: {}", transport.append());
    println!("  Layers: {}", transport.layers());
    println!("  Transport String: {}", transport.as_text());
    println!();
    
    // Demonstrate modifying an existing transport
    println!("--- Modifying Transport ---");
    transport.set_ttl(64);
    transport.set_mode_record(true);
    transport.set_append(true);
    
    println!("✓ Modified transport");
    println!("  TTL: {}", transport.ttl());
    println!("  Mode Record: {}", transport.mode_record());
    println!("  Append: {}", transport.append());
    println!("  Updated Transport String: {}", transport.as_text());
    println!();
    
    Ok(())
}

// Example function showing how to select transport based on requirements
#[allow(dead_code)]
fn select_transport_for_scenario(scenario: &str) -> Result<RTSPTransport, Box<dyn std::error::Error>> {
    match scenario {
        "low_latency" => {
            // TCP for lowest latency and reliability
            RTSPTransportBuilder::new()?
                .protocol(RTSPTransMode::RTP)
                .profile(RTSPProfile::AVP)
                .lower_transport(RTSPLowerTrans::TCP)
                .interleaved(0, 1)
                .build()
                .map_err(Into::into)
        }
        "broadcast" => {
            // Multicast for broadcast scenarios
            RTSPTransportBuilder::new()?
                .protocol(RTSPTransMode::RTP)
                .profile(RTSPProfile::AVP)
                .lower_transport(RTSPLowerTrans::UDP_MCAST)
                .destination("239.255.0.1")
                .port(5000, 5001)
                .ttl(127)
                .build()
                .map_err(Into::into)
        }
        "secure" => {
            // Secure transport for encrypted streams
            RTSPTransportBuilder::new()?
                .protocol(RTSPTransMode::RTP)
                .profile(RTSPProfile::SAVP)
                .lower_transport(RTSPLowerTrans::UDP)
                .client_ports(5000, 5001)
                .build()
                .map_err(Into::into)
        }
        _ => {
            // Default UDP transport
            helpers::create_rtp_transport(5000, 5001)
                .map_err(Into::into)
        }
    }
}

// Example showing transport negotiation logic
#[allow(dead_code)]
fn negotiate_transport(
    client_preferred: &RTSPTransport,
    server_capabilities: &[RTSPTransport],
) -> Option<RTSPTransport> {
    // Find a compatible transport from server capabilities
    for server_transport in server_capabilities {
        if server_transport.trans() == client_preferred.trans()
            && server_transport.profile() == client_preferred.profile()
        {
            // Found a match, use server's port configuration
            let mut negotiated = client_preferred.clone();
            negotiated.set_server_port(server_transport.server_port());
            return Some(negotiated);
        }
    }
    None
}