// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ffi, RTSPLowerTrans, RTSPProfile, RTSPResult, RTSPTransMode};
use glib::translate::*;
use std::fmt;
use std::ptr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GstRTSPRange")]
pub struct RTSPRange {
    pub min: i32,
    pub max: i32,
}

impl RTSPRange {
    pub fn new(min: i32, max: i32) -> Self {
        RTSPRange { min, max }
    }
}

impl From<ffi::GstRTSPRange> for RTSPRange {
    fn from(range: ffi::GstRTSPRange) -> Self {
        RTSPRange {
            min: range.min,
            max: range.max,
        }
    }
}

impl From<RTSPRange> for ffi::GstRTSPRange {
    fn from(range: RTSPRange) -> Self {
        ffi::GstRTSPRange {
            min: range.min as libc::c_int,
            max: range.max as libc::c_int,
        }
    }
}

glib::wrapper! {
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[doc(alias = "GstRTSPTransport")]
    pub struct RTSPTransport(Boxed<ffi::GstRTSPTransport>);

    match fn {
        copy => |ptr| {
            let mut copy = Box::new(unsafe { ptr::read(ptr) });
            // Deep copy string fields
            unsafe {
                if !(*ptr).destination.is_null() {
                    let dest = std::ffi::CStr::from_ptr((*ptr).destination);
                    copy.destination = glib::ffi::g_strdup(dest.as_ptr());
                }
                if !(*ptr).source.is_null() {
                    let src = std::ffi::CStr::from_ptr((*ptr).source);
                    copy.source = glib::ffi::g_strdup(src.as_ptr());
                }
            }
            Box::into_raw(copy)
        },
        free => |ptr| {
            unsafe {
                // Free string fields
                if !(*ptr).destination.is_null() {
                    glib::ffi::g_free((*ptr).destination as *mut _);
                }
                if !(*ptr).source.is_null() {
                    glib::ffi::g_free((*ptr).source as *mut _);
                }
                // Free the struct itself
                let _ = Box::from_raw(ptr);
            }
        },
    }
}

impl RTSPTransport {
    #[doc(alias = "gst_rtsp_transport_new")]
    pub fn new() -> Result<RTSPTransport, RTSPResult> {
        unsafe {
            let mut transport = ptr::null_mut();
            let ret = ffi::gst_rtsp_transport_new(&mut transport);
            if ret == ffi::GST_RTSP_OK {
                // Initialize the transport
                let init_ret = ffi::gst_rtsp_transport_init(transport);
                if init_ret == ffi::GST_RTSP_OK {
                    Ok(from_glib_full(transport))
                } else {
                    ffi::gst_rtsp_transport_free(transport);
                    Err(from_glib(ret))
                }
            } else {
                Err(from_glib(ret))
            }
        }
    }

    #[doc(alias = "gst_rtsp_transport_parse")]
    pub fn parse(transport_str: &str) -> Result<RTSPTransport, RTSPResult> {
        let transport = RTSPTransport::new()?;
        unsafe {
            let ret = ffi::gst_rtsp_transport_parse(
                transport_str.to_glib_none().0,
                transport.to_glib_none().0,
            );
            if ret == ffi::GST_RTSP_OK {
                Ok(transport)
            } else {
                Err(from_glib(ret))
            }
        }
    }

    #[doc(alias = "gst_rtsp_transport_as_text")]
    pub fn as_text(&self) -> glib::GString {
        unsafe { from_glib_full(ffi::gst_rtsp_transport_as_text(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_rtsp_transport_get_media_type")]
    pub fn get_media_type(&self) -> Result<Option<String>, RTSPResult> {
        unsafe {
            let mut media_type = ptr::null();
            let ret =
                ffi::gst_rtsp_transport_get_media_type(self.to_glib_none().0, &mut media_type);
            if ret == ffi::GST_RTSP_OK {
                if media_type.is_null() {
                    Ok(None)
                } else {
                    Ok(Some(
                        std::ffi::CStr::from_ptr(media_type)
                            .to_string_lossy()
                            .into_owned(),
                    ))
                }
            } else {
                Err(from_glib(ret))
            }
        }
    }

    #[doc(alias = "gst_rtsp_transport_get_manager")]
    pub fn get_manager(trans: RTSPTransMode, option: u32) -> Result<Option<String>, RTSPResult> {
        unsafe {
            let mut manager = ptr::null();
            let ret = ffi::gst_rtsp_transport_get_manager(trans.into_glib(), &mut manager, option);
            if ret == ffi::GST_RTSP_OK {
                if manager.is_null() {
                    Ok(None)
                } else {
                    Ok(Some(
                        std::ffi::CStr::from_ptr(manager)
                            .to_string_lossy()
                            .into_owned(),
                    ))
                }
            } else {
                Err(from_glib(ret))
            }
        }
    }

    // Field accessors
    pub fn trans(&self) -> RTSPTransMode {
        unsafe {
            let ptr: *const ffi::GstRTSPTransport = self.to_glib_none().0;
            from_glib((*ptr).trans)
        }
    }

    pub fn set_trans(&mut self, trans: RTSPTransMode) {
        unsafe {
            let ptr: *mut ffi::GstRTSPTransport = self.to_glib_none().0;
            (*ptr).trans = trans.into_glib();
        }
    }

    pub fn profile(&self) -> RTSPProfile {
        unsafe {
            let ptr: *const ffi::GstRTSPTransport = self.to_glib_none().0;
            from_glib((*ptr).profile)
        }
    }

    pub fn set_profile(&mut self, profile: RTSPProfile) {
        unsafe {
            let ptr: *mut ffi::GstRTSPTransport = self.to_glib_none().0;
            (*ptr).profile = profile.into_glib();
        }
    }

    pub fn lower_transport(&self) -> RTSPLowerTrans {
        unsafe {
            let ptr: *const ffi::GstRTSPTransport = self.to_glib_none().0;
            from_glib((*ptr).lower_transport)
        }
    }

    pub fn set_lower_transport(&mut self, lower_transport: RTSPLowerTrans) {
        unsafe {
            let ptr: *mut ffi::GstRTSPTransport = self.to_glib_none().0;
            (*ptr).lower_transport = lower_transport.into_glib();
        }
    }

    pub fn destination(&self) -> Option<String> {
        unsafe {
            let ptr: *const ffi::GstRTSPTransport = self.to_glib_none().0;
            if (*ptr).destination.is_null() {
                None
            } else {
                Some(
                    std::ffi::CStr::from_ptr((*ptr).destination)
                        .to_string_lossy()
                        .into_owned(),
                )
            }
        }
    }

    pub fn set_destination(&mut self, destination: Option<&str>) {
        unsafe {
            let ptr: *mut ffi::GstRTSPTransport = self.to_glib_none().0;
            if !(*ptr).destination.is_null() {
                glib::ffi::g_free((*ptr).destination as *mut _);
            }
            (*ptr).destination = destination.to_glib_full();
        }
    }

    pub fn source(&self) -> Option<String> {
        unsafe {
            let ptr: *const ffi::GstRTSPTransport = self.to_glib_none().0;
            if (*ptr).source.is_null() {
                None
            } else {
                Some(
                    std::ffi::CStr::from_ptr((*ptr).source)
                        .to_string_lossy()
                        .into_owned(),
                )
            }
        }
    }

    pub fn set_source(&mut self, source: Option<&str>) {
        unsafe {
            let ptr: *mut ffi::GstRTSPTransport = self.to_glib_none().0;
            if !(*ptr).source.is_null() {
                glib::ffi::g_free((*ptr).source as *mut _);
            }
            (*ptr).source = source.to_glib_full();
        }
    }

    pub fn layers(&self) -> u32 {
        unsafe {
            let ptr: *const ffi::GstRTSPTransport = self.to_glib_none().0;
            (*ptr).layers as u32
        }
    }

    pub fn set_layers(&mut self, layers: u32) {
        unsafe {
            let ptr: *mut ffi::GstRTSPTransport = self.to_glib_none().0;
            (*ptr).layers = layers as libc::c_uint;
        }
    }

    pub fn mode_play(&self) -> bool {
        unsafe {
            let ptr: *const ffi::GstRTSPTransport = self.to_glib_none().0;
            from_glib((*ptr).mode_play)
        }
    }

    pub fn set_mode_play(&mut self, mode_play: bool) {
        unsafe {
            let ptr: *mut ffi::GstRTSPTransport = self.to_glib_none().0;
            (*ptr).mode_play = mode_play.into_glib();
        }
    }

    pub fn mode_record(&self) -> bool {
        unsafe {
            let ptr: *const ffi::GstRTSPTransport = self.to_glib_none().0;
            from_glib((*ptr).mode_record)
        }
    }

    pub fn set_mode_record(&mut self, mode_record: bool) {
        unsafe {
            let ptr: *mut ffi::GstRTSPTransport = self.to_glib_none().0;
            (*ptr).mode_record = mode_record.into_glib();
        }
    }

    pub fn append(&self) -> bool {
        unsafe {
            let ptr: *const ffi::GstRTSPTransport = self.to_glib_none().0;
            from_glib((*ptr).append)
        }
    }

    pub fn set_append(&mut self, append: bool) {
        unsafe {
            let ptr: *mut ffi::GstRTSPTransport = self.to_glib_none().0;
            (*ptr).append = append.into_glib();
        }
    }

    pub fn interleaved(&self) -> RTSPRange {
        unsafe {
            let ptr: *const ffi::GstRTSPTransport = self.to_glib_none().0;
            RTSPRange::from((*ptr).interleaved)
        }
    }

    pub fn set_interleaved(&mut self, interleaved: RTSPRange) {
        unsafe {
            let ptr: *mut ffi::GstRTSPTransport = self.to_glib_none().0;
            (*ptr).interleaved = interleaved.into();
        }
    }

    pub fn ttl(&self) -> u32 {
        unsafe {
            let ptr: *const ffi::GstRTSPTransport = self.to_glib_none().0;
            (*ptr).ttl as u32
        }
    }

    pub fn set_ttl(&mut self, ttl: u32) {
        unsafe {
            let ptr: *mut ffi::GstRTSPTransport = self.to_glib_none().0;
            (*ptr).ttl = ttl as libc::c_uint;
        }
    }

    pub fn port(&self) -> RTSPRange {
        unsafe {
            let ptr: *const ffi::GstRTSPTransport = self.to_glib_none().0;
            RTSPRange::from((*ptr).port)
        }
    }

    pub fn set_port(&mut self, port: RTSPRange) {
        unsafe {
            let ptr: *mut ffi::GstRTSPTransport = self.to_glib_none().0;
            (*ptr).port = port.into();
        }
    }

    pub fn client_port(&self) -> RTSPRange {
        unsafe {
            let ptr: *const ffi::GstRTSPTransport = self.to_glib_none().0;
            RTSPRange::from((*ptr).client_port)
        }
    }

    pub fn set_client_port(&mut self, client_port: RTSPRange) {
        unsafe {
            let ptr: *mut ffi::GstRTSPTransport = self.to_glib_none().0;
            (*ptr).client_port = client_port.into();
        }
    }

    pub fn server_port(&self) -> RTSPRange {
        unsafe {
            let ptr: *const ffi::GstRTSPTransport = self.to_glib_none().0;
            RTSPRange::from((*ptr).server_port)
        }
    }

    pub fn set_server_port(&mut self, server_port: RTSPRange) {
        unsafe {
            let ptr: *mut ffi::GstRTSPTransport = self.to_glib_none().0;
            (*ptr).server_port = server_port.into();
        }
    }

    pub fn ssrc(&self) -> u32 {
        unsafe {
            let ptr: *const ffi::GstRTSPTransport = self.to_glib_none().0;
            (*ptr).ssrc as u32
        }
    }

    pub fn set_ssrc(&mut self, ssrc: u32) {
        unsafe {
            let ptr: *mut ffi::GstRTSPTransport = self.to_glib_none().0;
            (*ptr).ssrc = ssrc as libc::c_uint;
        }
    }
}

impl Default for RTSPTransport {
    fn default() -> Self {
        RTSPTransport::new().expect("Failed to create RTSPTransport")
    }
}

impl fmt::Display for RTSPTransport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_text())
    }
}

impl fmt::Debug for RTSPTransport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("RTSPTransport")
            .field("trans", &self.trans())
            .field("profile", &self.profile())
            .field("lower_transport", &self.lower_transport())
            .field("destination", &self.destination())
            .field("source", &self.source())
            .field("layers", &self.layers())
            .field("mode_play", &self.mode_play())
            .field("mode_record", &self.mode_record())
            .field("append", &self.append())
            .field("interleaved", &self.interleaved())
            .field("ttl", &self.ttl())
            .field("port", &self.port())
            .field("client_port", &self.client_port())
            .field("server_port", &self.server_port())
            .field("ssrc", &self.ssrc())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rtsp_range() {
        let range = RTSPRange::new(0, 100);
        assert_eq!(range.min, 0);
        assert_eq!(range.max, 100);
    }

    #[test]
    fn test_rtsp_transport_new() {
        let transport = RTSPTransport::new();
        assert!(transport.is_ok());
    }

    #[test]
    fn test_rtsp_transport_parse() {
        // Test parsing a simple RTP/AVP transport
        let transport_str = "RTP/AVP;unicast;client_port=5000-5001";
        let transport = RTSPTransport::parse(transport_str);
        assert!(transport.is_ok());
    }

    #[test]
    fn test_rtsp_transport_fields() {
        let mut transport = RTSPTransport::new().unwrap();

        // Test trans field
        transport.set_trans(RTSPTransMode::RTP);
        assert_eq!(transport.trans(), RTSPTransMode::RTP);

        // Test profile field
        transport.set_profile(RTSPProfile::AVP);
        assert_eq!(transport.profile(), RTSPProfile::AVP);

        // Test lower_transport field
        transport.set_lower_transport(RTSPLowerTrans::UDP);
        assert_eq!(transport.lower_transport(), RTSPLowerTrans::UDP);

        // Test destination field
        transport.set_destination(Some("192.168.1.1"));
        assert_eq!(transport.destination(), Some("192.168.1.1".to_string()));

        // Test source field
        transport.set_source(Some("192.168.1.2"));
        assert_eq!(transport.source(), Some("192.168.1.2".to_string()));

        // Test layers field
        transport.set_layers(2);
        assert_eq!(transport.layers(), 2);

        // Test mode fields
        transport.set_mode_play(true);
        assert!(transport.mode_play());
        transport.set_mode_record(false);
        assert!(!transport.mode_record());

        // Test append field
        transport.set_append(true);
        assert!(transport.append());

        // Test interleaved range
        let interleaved = RTSPRange::new(0, 1);
        transport.set_interleaved(interleaved);
        assert_eq!(transport.interleaved(), interleaved);

        // Test ttl field
        transport.set_ttl(64);
        assert_eq!(transport.ttl(), 64);

        // Test port ranges
        let port_range = RTSPRange::new(5000, 5001);
        transport.set_client_port(port_range);
        assert_eq!(transport.client_port(), port_range);

        transport.set_server_port(port_range);
        assert_eq!(transport.server_port(), port_range);

        transport.set_port(port_range);
        assert_eq!(transport.port(), port_range);

        // Test ssrc field
        transport.set_ssrc(12345678);
        assert_eq!(transport.ssrc(), 12345678);
    }

    #[test]
    fn test_rtsp_transport_as_text() {
        let transport = RTSPTransport::new().unwrap();
        let text = transport.as_text();
        assert!(!text.is_empty());
    }

    #[test]
    fn test_rtsp_transport_display() {
        let transport = RTSPTransport::new().unwrap();
        let display = format!("{}", transport);
        assert!(!display.is_empty());
    }

    #[test]
    fn test_rtsp_transport_debug() {
        let transport = RTSPTransport::new().unwrap();
        let debug = format!("{:?}", transport);
        assert!(debug.contains("RTSPTransport"));
    }
}
