// Take a look at the license at the top of the repository in the LICENSE file.

// Basic RTSP Client Example
// This example demonstrates how to create an RTSP connection, send requests,
// and handle responses using the gstreamer-rtsp bindings.

use gstreamer_rtsp::{
    builders::RTSPConnectionBuilder, rtsp_message::RTSPMessage, RTSPAuthMethod, RTSPConnection,
    RTSPHeaderField, RTSPMethod, RTSPResult, RTSPStatusCode, RTSPUrl,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize GStreamer
    gstreamer_rtsp::init()?;

    // Example RTSP URL - replace with your actual RTSP server
    let rtsp_url = "rtsp://localhost:8554/videotestsrc";
    
    println!("RTSP Client Example");
    println!("===================");
    println!("Connecting to: {}", rtsp_url);
    
    // Parse the RTSP URL
    let (result, url) = RTSPUrl::parse(rtsp_url);
    if result != RTSPResult::Ok {
        return Err(format!("Failed to parse URL: {:?}", result).into());
    }
    let url = url.unwrap();
    
    // Method 1: Create connection directly
    create_connection_direct(&url)?;
    
    // Method 2: Create connection using builder
    create_connection_with_builder(rtsp_url)?;
    
    // Demonstrate message creation
    demonstrate_messages(rtsp_url)?;
    
    Ok(())
}

fn create_connection_direct(url: &RTSPUrl) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Creating Connection Directly ---");
    
    // Create the connection
    let conn = RTSPConnection::create(url)?;
    println!("✓ Connection created");
    
    // Configure the connection
    conn.set_auth(RTSPAuthMethod::Basic, "user", "password")?;
    println!("✓ Authentication configured");
    
    conn.set_remember_session_id(true);
    println!("✓ Session ID remembering enabled");
    
    // Note: In a real application, you would connect here:
    // conn.connect(timeout)?;
    // But we skip it in this example as it requires a real server
    
    println!("✓ Connection configured successfully");
    
    // Close the connection (would normally be done after communication)
    // conn.close()?;
    
    Ok(())
}

fn create_connection_with_builder(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Creating Connection with Builder ---");
    
    // Create connection using the builder pattern
    let conn = RTSPConnectionBuilder::new(url)?
        .auth(RTSPAuthMethod::Digest, "admin", "secret")
        .proxy("proxy.example.com", 8080)
        .build()?;
    
    println!("✓ Connection created with builder");
    println!("  - Authentication: Digest");
    println!("  - Proxy: proxy.example.com:8080");
    
    // Additional configuration
    conn.set_http_mode(true);
    println!("✓ HTTP tunneling mode enabled");
    
    #[cfg(feature = "v1_18")]
    {
        conn.set_content_length_limit(10 * 1024 * 1024); // 10MB
        println!("✓ Content length limit set to 10MB");
    }
    
    Ok(())
}

fn demonstrate_messages(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- RTSP Message Examples ---");
    
    let mut cseq = 1;
    
    // 1. OPTIONS Request
    println!("\n1. OPTIONS Request:");
    let mut options = RTSPMessage::new_request(RTSPMethod::OPTIONS, url)?;
    options.add_header(RTSPHeaderField::CSeq, &cseq.to_string());
    options.add_header(RTSPHeaderField::UserAgent, "Example RTSP Client/1.0");
    print_message_info(&options, "OPTIONS");
    cseq += 1;
    
    // 2. DESCRIBE Request
    println!("\n2. DESCRIBE Request:");
    let mut describe = RTSPMessage::new_request(RTSPMethod::DESCRIBE, url)?;
    describe.add_header(RTSPHeaderField::CSeq, &cseq.to_string());
    describe.add_header(RTSPHeaderField::Accept, "application/sdp");
    print_message_info(&describe, "DESCRIBE");
    cseq += 1;
    
    // 3. SETUP Request
    println!("\n3. SETUP Request:");
    let stream_url = format!("{}/stream0", url);
    let mut setup = RTSPMessage::new_request(RTSPMethod::SETUP, &stream_url)?;
    setup.add_header(RTSPHeaderField::CSeq, &cseq.to_string());
    setup.add_header(RTSPHeaderField::Transport, "RTP/AVP;unicast;client_port=5000-5001");
    print_message_info(&setup, "SETUP");
    cseq += 1;
    
    // 4. PLAY Request
    println!("\n4. PLAY Request:");
    let mut play = RTSPMessage::new_request(RTSPMethod::PLAY, url)?;
    play.add_header(RTSPHeaderField::CSeq, &cseq.to_string());
    play.add_header(RTSPHeaderField::Session, "12345678");
    play.add_header(RTSPHeaderField::Range, "npt=0.000-");
    print_message_info(&play, "PLAY");
    cseq += 1;
    
    // 5. PAUSE Request
    println!("\n5. PAUSE Request:");
    let mut pause = RTSPMessage::new_request(RTSPMethod::PAUSE, url)?;
    pause.add_header(RTSPHeaderField::CSeq, &cseq.to_string());
    pause.add_header(RTSPHeaderField::Session, "12345678");
    print_message_info(&pause, "PAUSE");
    cseq += 1;
    
    // 6. TEARDOWN Request
    println!("\n6. TEARDOWN Request:");
    let mut teardown = RTSPMessage::new_request(RTSPMethod::TEARDOWN, url)?;
    teardown.add_header(RTSPHeaderField::CSeq, &cseq.to_string());
    teardown.add_header(RTSPHeaderField::Session, "12345678");
    print_message_info(&teardown, "TEARDOWN");
    
    // Example Response
    println!("\n--- Example Response ---");
    let response = RTSPMessage::new_response(
        RTSPStatusCode::Ok,
        Some("OK"),
        Some(&options),
    )?;
    println!("✓ Created response for OPTIONS request");
    println!("  Status: 200 OK");
    
    Ok(())
}

fn print_message_info(msg: &RTSPMessage, method: &str) {
    println!("  ✓ {} request created", method);
    
    // Print CSeq header if present
    if let Ok((_, Some(cseq))) = msg.get_header(RTSPHeaderField::CSeq) {
        println!("    CSeq: {}", cseq);
    }
    
    // Print Transport header if present
    if let Ok((_, Some(transport))) = msg.get_header(RTSPHeaderField::Transport) {
        println!("    Transport: {}", transport);
    }
    
    // Print Session header if present
    if let Ok((_, Some(session))) = msg.get_header(RTSPHeaderField::Session) {
        println!("    Session: {}", session);
    }
}

// Example of how to handle connection in a real scenario
#[allow(dead_code)]
fn real_connection_example(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    // This function shows how you would use the connection in a real scenario
    // with an actual RTSP server
    
    let conn = RTSPConnectionBuilder::new(url)?
        .auth(RTSPAuthMethod::Basic, "user", "pass")
        .build()?;
    
    // Connect with timeout (requires actual server)
    #[cfg(feature = "v1_18")]
    {
        // conn.connect(20_000_000)?; // 20 seconds timeout
    }
    
    #[cfg(not(feature = "v1_18"))]
    {
        // conn.connect(20, 0)?; // 20 seconds timeout
    }
    
    // Send OPTIONS request
    let options = RTSPMessage::new_request(RTSPMethod::OPTIONS, url)?;
    
    #[cfg(feature = "v1_18")]
    {
        // conn.send(&options, 5_000_000)?; // 5 seconds timeout
    }
    
    #[cfg(not(feature = "v1_18"))]
    {
        // conn.send(&options, 5, 0)?; // 5 seconds timeout
    }
    
    // Receive response
    let response = RTSPMessage::new()?;
    
    #[cfg(feature = "v1_18")]
    {
        // conn.receive(&response, 5_000_000)?; // 5 seconds timeout
    }
    
    #[cfg(not(feature = "v1_18"))]
    {
        // conn.receive(&response, 5, 0)?; // 5 seconds timeout
    }
    
    // Close connection
    // conn.close()?;
    
    Ok(())
}
