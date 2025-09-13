// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{
    RTSPAuthMethod, RTSPConnection, RTSPLowerTrans, RTSPProfile, RTSPRange, RTSPResult,
    RTSPTransMode, RTSPTransport, RTSPUrl,
};

/// Builder for creating and configuring `RTSPTransport` instances.
///
/// # Example
///
/// ```no_run
/// # use gstreamer_rtsp::{RTSPTransportBuilder, RTSPProfile, RTSPLowerTrans, RTSPRange};
/// let transport = RTSPTransportBuilder::new()
///     .profile(RTSPProfile::AVP)
///     .lower_transport(RTSPLowerTrans::UDP)
///     .client_ports(5000, 5001)
///     .build()
///     .expect("Failed to build transport");
/// ```
#[derive(Debug)]
#[must_use = "The builder must be built to be used"]
pub struct RTSPTransportBuilder {
    transport: RTSPTransport,
}

impl RTSPTransportBuilder {
    /// Creates a new `RTSPTransportBuilder`.
    pub fn new() -> Result<Self, RTSPResult> {
        Ok(RTSPTransportBuilder {
            transport: RTSPTransport::new()?,
        })
    }

    /// Sets the transport mode (RTP or RDT).
    pub fn protocol(mut self, trans: RTSPTransMode) -> Self {
        self.transport.set_trans(trans);
        self
    }

    /// Sets the transport profile (AVP, SAVP, AVPF, or SAVPF).
    pub fn profile(mut self, profile: RTSPProfile) -> Self {
        self.transport.set_profile(profile);
        self
    }

    /// Sets the lower transport (UDP, TCP, etc.).
    pub fn lower_transport(mut self, lower_transport: RTSPLowerTrans) -> Self {
        self.transport.set_lower_transport(lower_transport);
        self
    }

    /// Sets the destination address.
    pub fn destination(mut self, destination: &str) -> Self {
        self.transport.set_destination(Some(destination));
        self
    }

    /// Sets the source address.
    pub fn source(mut self, source: &str) -> Self {
        self.transport.set_source(Some(source));
        self
    }

    /// Enables or disables play mode.
    pub fn mode_play(mut self, play: bool) -> Self {
        self.transport.set_mode_play(play);
        self
    }

    /// Enables or disables record mode.
    pub fn mode_record(mut self, record: bool) -> Self {
        self.transport.set_mode_record(record);
        self
    }

    /// Sets both play and record modes.
    pub fn mode(mut self, play: bool, record: bool) -> Self {
        self.transport.set_mode_play(play);
        self.transport.set_mode_record(record);
        self
    }

    /// Sets the client port range.
    pub fn client_ports(mut self, min: i32, max: i32) -> Self {
        self.transport.set_client_port(RTSPRange::new(min, max));
        self
    }

    /// Sets the server port range.
    pub fn server_ports(mut self, min: i32, max: i32) -> Self {
        self.transport.set_server_port(RTSPRange::new(min, max));
        self
    }

    /// Sets the interleaved channel range.
    pub fn interleaved(mut self, min: i32, max: i32) -> Self {
        self.transport.set_interleaved(RTSPRange::new(min, max));
        self
    }

    /// Sets the multicast TTL value.
    pub fn ttl(mut self, ttl: u32) -> Self {
        self.transport.set_ttl(ttl);
        self
    }

    /// Sets the SSRC value.
    pub fn ssrc(mut self, ssrc: u32) -> Self {
        self.transport.set_ssrc(ssrc);
        self
    }

    /// Sets the number of layers.
    pub fn layers(mut self, layers: u32) -> Self {
        self.transport.set_layers(layers);
        self
    }

    /// Enables or disables append mode.
    pub fn append(mut self, append: bool) -> Self {
        self.transport.set_append(append);
        self
    }

    /// Sets the multicast port range.
    pub fn port(mut self, min: i32, max: i32) -> Self {
        self.transport.set_port(RTSPRange::new(min, max));
        self
    }

    /// Builds the configured `RTSPTransport`.
    #[must_use = "Building the transport without using it has no effect"]
    pub fn build(self) -> Result<RTSPTransport, RTSPResult> {
        // Validation could be added here if needed
        Ok(self.transport)
    }
}

impl Default for RTSPTransportBuilder {
    fn default() -> Self {
        RTSPTransportBuilder::new().expect("Failed to create default RTSPTransportBuilder")
    }
}

/// Builder for creating and configuring `RTSPConnection` instances.
///
/// # Example
///
/// ```no_run
/// # use gstreamer_rtsp::{RTSPConnectionBuilder, RTSPUrl, RTSPAuthMethod};
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let url = RTSPUrl::parse("rtsp://localhost:554/test").1.unwrap();
/// let connection = RTSPConnectionBuilder::new(&url)
///     .auth(RTSPAuthMethod::Basic, "user", "pass")
///     .proxy("proxy.example.com", 8080)
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
#[must_use = "The builder must be built to be used"]
pub struct RTSPConnectionBuilder {
    url: RTSPUrl,
    proxy_host: Option<String>,
    proxy_port: Option<u32>,
    auth_method: Option<RTSPAuthMethod>,
    auth_user: Option<String>,
    auth_pass: Option<String>,
    tunneled: bool,
    http_mode: bool,
    #[cfg(feature = "v1_18")]
    timeout: Option<i64>,
    #[cfg(not(feature = "v1_18"))]
    timeout_secs: Option<i64>,
    #[cfg(not(feature = "v1_18"))]
    timeout_usecs: Option<i64>,
}

impl RTSPConnectionBuilder {
    /// Creates a new `RTSPConnectionBuilder` for the given URL.
    pub fn new(url: &RTSPUrl) -> Self {
        RTSPConnectionBuilder {
            url: url.clone(),
            proxy_host: None,
            proxy_port: None,
            auth_method: None,
            auth_user: None,
            auth_pass: None,
            tunneled: false,
            http_mode: false,
            #[cfg(feature = "v1_18")]
            timeout: None,
            #[cfg(not(feature = "v1_18"))]
            timeout_secs: None,
            #[cfg(not(feature = "v1_18"))]
            timeout_usecs: None,
        }
    }

    /// Sets the proxy server.
    pub fn proxy(mut self, host: &str, port: u32) -> Self {
        self.proxy_host = Some(host.to_string());
        self.proxy_port = Some(port);
        self
    }

    /// Sets the authentication credentials.
    pub fn auth(mut self, method: RTSPAuthMethod, user: &str, pass: &str) -> Self {
        self.auth_method = Some(method);
        self.auth_user = Some(user.to_string());
        self.auth_pass = Some(pass.to_string());
        self
    }

    /// Sets the connection timeout in microseconds (v1_18+).
    #[cfg(feature = "v1_18")]
    pub fn timeout(mut self, timeout_usec: i64) -> Self {
        self.timeout = Some(timeout_usec);
        self
    }

    /// Sets the connection timeout (pre-v1_18).
    #[cfg(not(feature = "v1_18"))]
    pub fn timeout(mut self, secs: i64, usecs: i64) -> Self {
        self.timeout_secs = Some(secs);
        self.timeout_usecs = Some(usecs);
        self
    }

    /// Enables or disables tunneling.
    pub fn tunneled(mut self, tunneled: bool) -> Self {
        self.tunneled = tunneled;
        self
    }

    /// Enables or disables HTTP mode.
    pub fn http_mode(mut self, http_mode: bool) -> Self {
        self.http_mode = http_mode;
        self
    }

    /// Builds the `RTSPConnection` without connecting.
    #[must_use = "Building the connection without using it has no effect"]
    pub fn build(self) -> Result<RTSPConnection, RTSPResult> {
        let conn = RTSPConnection::create(&self.url)?;

        // Apply configuration
        if let (Some(host), Some(port)) = (self.proxy_host, self.proxy_port) {
            conn.set_proxy(&host, port)?;
        }

        if let (Some(method), Some(user), Some(pass)) =
            (self.auth_method, self.auth_user, self.auth_pass)
        {
            conn.set_auth(method, &user, &pass)?;
        }

        conn.set_tunneled(self.tunneled);
        conn.set_http_mode(self.http_mode);

        Ok(conn)
    }

    /// Builds and connects the `RTSPConnection`.
    pub fn connect(self) -> Result<RTSPConnection, RTSPResult> {
        // Save timeout values before moving self
        #[cfg(feature = "v1_18")]
        let timeout = self.timeout.unwrap_or(20_000_000); // 20 seconds default

        #[cfg(not(feature = "v1_18"))]
        let (secs, usecs) = (
            self.timeout_secs.unwrap_or(20),
            self.timeout_usecs.unwrap_or(0),
        );

        let conn = self.build()?;

        // Connect with appropriate timeout
        #[cfg(feature = "v1_18")]
        conn.connect(timeout)?;

        #[cfg(not(feature = "v1_18"))]
        conn.connect(secs, usecs)?;

        Ok(conn)
    }
}

/// Helper functions for RTSP operations.
pub mod helpers {
    use super::*;

    /// Parses a transport string into an `RTSPTransport`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use gstreamer_rtsp::builders::helpers;
    /// let transport = helpers::parse_transport("RTP/AVP;unicast;client_port=5000-5001")
    ///     .expect("Failed to parse transport");
    /// ```
    pub fn parse_transport(transport_str: &str) -> Result<RTSPTransport, RTSPResult> {
        RTSPTransport::parse(transport_str)
    }

    /// Formats an `RTSPTransport` as a string.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use gstreamer_rtsp::{RTSPTransport, builders::helpers};
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let transport = RTSPTransport::new()?;
    /// let transport_str = helpers::format_transport(&transport);
    /// # Ok(())
    /// # }
    /// ```
    pub fn format_transport(transport: &RTSPTransport) -> String {
        transport.as_text().to_string()
    }

    /// Gets the MIME type for a transport mode.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use gstreamer_rtsp::{RTSPTransMode, RTSPProfile, builders::helpers};
    /// let mime = helpers::transport_get_mime(RTSPTransMode::RTP, RTSPProfile::AVP);
    /// assert_eq!(mime, "application/x-rtp");
    /// ```
    pub fn transport_get_mime(trans: RTSPTransMode, profile: RTSPProfile) -> &'static str {
        match trans {
            RTSPTransMode::RTP => match profile {
                RTSPProfile::AVP | RTSPProfile::AVPF => "application/x-rtp",
                RTSPProfile::SAVP | RTSPProfile::SAVPF => "application/x-srtp",
                _ => "application/x-rtp",
            },
            RTSPTransMode::RDT => "application/x-rdt",
            _ => "application/octet-stream",
        }
    }

    /// Gets the manager element name for a transport configuration.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use gstreamer_rtsp::{RTSPTransMode, builders::helpers};
    /// let manager = helpers::transport_get_manager(RTSPTransMode::RTP, 0)
    ///     .expect("Manager found");
    /// assert_eq!(manager, "rtpbin");
    /// ```
    pub fn transport_get_manager(trans: RTSPTransMode, _option: u32) -> Option<&'static str> {
        match trans {
            RTSPTransMode::RTP => Some("rtpbin"),
            RTSPTransMode::RDT => Some("rdtmanager"),
            _ => None,
        }
    }

    /// Gets the default port for RTSP protocol.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use gstreamer_rtsp::builders::helpers;
    /// assert_eq!(helpers::default_port(false), 554);
    /// assert_eq!(helpers::default_port(true), 322);  // RTSPS
    /// ```
    pub fn default_port(secure: bool) -> u16 {
        if secure {
            322 // RTSPS default port
        } else {
            554 // RTSP default port
        }
    }

    /// Parses options from an RTSP URI.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use gstreamer_rtsp::builders::helpers;
    /// let options = helpers::options_from_uri("rtsp://localhost:554/test?timeout=30&latency=100");
    /// ```
    pub fn options_from_uri(uri: &str) -> Vec<(String, String)> {
        let mut options = Vec::new();

        if let Some(query_start) = uri.find('?') {
            let query = &uri[query_start + 1..];
            for pair in query.split('&') {
                if let Some(eq_pos) = pair.find('=') {
                    let key = pair[..eq_pos].to_string();
                    let value = pair[eq_pos + 1..].to_string();
                    options.push((key, value));
                } else {
                    options.push((pair.to_string(), String::new()));
                }
            }
        }

        options
    }

    /// Creates a simple RTP/AVP transport configuration.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use gstreamer_rtsp::builders::helpers;
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let transport = helpers::create_rtp_transport(5000, 5001)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_rtp_transport(
        client_port_min: i32,
        client_port_max: i32,
    ) -> Result<RTSPTransport, RTSPResult> {
        RTSPTransportBuilder::new()?
            .protocol(RTSPTransMode::RTP)
            .profile(RTSPProfile::AVP)
            .lower_transport(RTSPLowerTrans::UDP)
            .client_ports(client_port_min, client_port_max)
            .build()
    }

    /// Creates a TCP interleaved transport configuration.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use gstreamer_rtsp::builders::helpers;
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let transport = helpers::create_tcp_transport(0, 1)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_tcp_transport(
        channel_min: i32,
        channel_max: i32,
    ) -> Result<RTSPTransport, RTSPResult> {
        RTSPTransportBuilder::new()?
            .protocol(RTSPTransMode::RTP)
            .profile(RTSPProfile::AVP)
            .lower_transport(RTSPLowerTrans::TCP)
            .interleaved(channel_min, channel_max)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transport_builder() {
        let transport = RTSPTransportBuilder::new()
            .unwrap()
            .profile(RTSPProfile::AVP)
            .lower_transport(RTSPLowerTrans::UDP)
            .client_ports(5000, 5001)
            .server_ports(6000, 6001)
            .ttl(64)
            .build()
            .unwrap();

        assert_eq!(transport.profile(), RTSPProfile::AVP);
        assert_eq!(transport.lower_transport(), RTSPLowerTrans::UDP);
        assert_eq!(transport.client_port(), RTSPRange::new(5000, 5001));
        assert_eq!(transport.server_port(), RTSPRange::new(6000, 6001));
        assert_eq!(transport.ttl(), 64);
    }

    #[test]
    fn test_transport_builder_modes() {
        let transport = RTSPTransportBuilder::new()
            .unwrap()
            .mode(true, false)
            .build()
            .unwrap();

        assert!(transport.mode_play());
        assert!(!transport.mode_record());
    }

    #[test]
    fn test_connection_builder() {
        crate::init().unwrap();

        let (result, url) = RTSPUrl::parse("rtsp://localhost:554/test");
        assert_eq!(result, crate::RTSPResult::Ok);
        let url = url.unwrap();

        let conn = RTSPConnectionBuilder::new(&url)
            .proxy("proxy.example.com", 8080)
            .tunneled(true)
            .http_mode(true)
            .build();

        assert!(conn.is_ok());
        let conn = conn.unwrap();
        assert!(conn.is_tunneled());
    }

    #[test]
    fn test_helpers_parse_transport() {
        let transport_str = "RTP/AVP;unicast;client_port=5000-5001";
        let transport = helpers::parse_transport(transport_str);
        assert!(transport.is_ok());
    }

    #[test]
    fn test_helpers_transport_mime() {
        assert_eq!(
            helpers::transport_get_mime(RTSPTransMode::RTP, RTSPProfile::AVP),
            "application/x-rtp"
        );
        assert_eq!(
            helpers::transport_get_mime(RTSPTransMode::RTP, RTSPProfile::SAVP),
            "application/x-srtp"
        );
        assert_eq!(
            helpers::transport_get_mime(RTSPTransMode::RDT, RTSPProfile::AVP),
            "application/x-rdt"
        );
    }

    #[test]
    fn test_helpers_default_port() {
        assert_eq!(helpers::default_port(false), 554);
        assert_eq!(helpers::default_port(true), 322);
    }

    #[test]
    fn test_helpers_options_from_uri() {
        let options = helpers::options_from_uri("rtsp://localhost:554/test?timeout=30&latency=100");
        assert_eq!(options.len(), 2);
        assert_eq!(options[0], ("timeout".to_string(), "30".to_string()));
        assert_eq!(options[1], ("latency".to_string(), "100".to_string()));
    }

    #[test]
    fn test_helpers_create_transports() {
        let rtp_transport = helpers::create_rtp_transport(5000, 5001);
        assert!(rtp_transport.is_ok());
        let rtp_transport = rtp_transport.unwrap();
        assert_eq!(rtp_transport.profile(), RTSPProfile::AVP);
        assert_eq!(rtp_transport.lower_transport(), RTSPLowerTrans::UDP);

        let tcp_transport = helpers::create_tcp_transport(0, 1);
        assert!(tcp_transport.is_ok());
        let tcp_transport = tcp_transport.unwrap();
        assert_eq!(tcp_transport.lower_transport(), RTSPLowerTrans::TCP);
        assert_eq!(tcp_transport.interleaved(), RTSPRange::new(0, 1));
    }
}
