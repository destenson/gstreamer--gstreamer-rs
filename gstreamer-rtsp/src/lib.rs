// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::manual_c_str_literals)]
#![doc = include_str!("../README.md")]

pub use glib;
pub use gst;
pub use gst_sdp;
pub use gstreamer_rtsp_sys as ffi;

macro_rules! assert_initialized_main_thread {
    () => {
        if !gst::INITIALIZED.load(std::sync::atomic::Ordering::SeqCst) {
            gst::assert_initialized();
        }
    };
}

macro_rules! skip_assert_initialized {
    () => {};
}

#[allow(unused_imports)]
mod auto;
pub use crate::auto::*;

#[cfg(feature = "serde")]
mod flag_serde;

/// Builder patterns for constructing RTSP connections and transports
/// 
/// This module provides convenient builder APIs for creating and configuring
/// RTSP connections and transports with a fluent interface.
/// 
/// # Examples
/// 
/// ```no_run
/// use gstreamer_rtsp::builders::RTSPConnectionBuilder;
/// 
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let conn = RTSPConnectionBuilder::new("rtsp://localhost:554/test")?
///     .auth(gstreamer_rtsp::RTSPAuthMethod::Basic, "user", "pass")
///     .proxy("proxy.example.com", 8080)
///     .build()?;
/// # Ok(())
/// # }
/// ```
pub mod builders;

/// RTSP authentication credential handling
pub mod rtsp_auth_credential;

/// RTSP connection management
/// 
/// This module provides the RTSPConnection type for establishing and managing
/// RTSP connections to servers. It supports various features including
/// authentication, proxies, TLS, and HTTP tunneling.
/// 
/// # Examples
/// 
/// ```no_run
/// use gstreamer_rtsp::{RTSPConnection, RTSPUrl};
/// 
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let (result, url) = RTSPUrl::parse("rtsp://localhost:554/test");
/// let url = url.unwrap();
/// let conn = RTSPConnection::create(&url)?;
/// # Ok(())
/// # }
/// ```
pub mod rtsp_connection;

/// RTSP message creation and manipulation
/// 
/// This module provides types for creating and working with RTSP request
/// and response messages, including headers and body content.
pub mod rtsp_message;

/// RTSP transport configuration
/// 
/// This module provides the RTSPTransport type for configuring media
/// transport parameters including protocols, profiles, ports, and modes.
/// 
/// # Examples
/// 
/// ```no_run
/// use gstreamer_rtsp::{RTSPTransport, RTSPTransMode, RTSPProfile, RTSPLowerTrans};
/// 
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let mut transport = RTSPTransport::new()?;
/// transport.set_trans(RTSPTransMode::RTP);
/// transport.set_profile(RTSPProfile::AVP);
/// transport.set_lower_transport(RTSPLowerTrans::UDP);
/// # Ok(())
/// # }
/// ```
pub mod rtsp_transport;

pub use crate::builders::{RTSPConnectionBuilder, RTSPTransportBuilder};
pub use crate::rtsp_connection::RTSPConnection;
pub use crate::rtsp_transport::{RTSPRange, RTSPTransport};

// Re-export all the traits in a prelude module, so that applications
// can always "use gst_rtsp::prelude::*" without getting conflicts
pub mod prelude {
    #[doc(hidden)]
    pub use gst_sdp::prelude::*;

    pub use crate::builders::{helpers, RTSPConnectionBuilder, RTSPTransportBuilder};
}

pub fn init() -> Result<(), glib::Error> {
    gst::init()
}
