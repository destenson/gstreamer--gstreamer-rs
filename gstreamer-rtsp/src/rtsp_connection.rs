// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(feature = "v1_18")]
use crate::RTSPEvent;
use crate::{ffi, rtsp_message::RTSPMessage, RTSPAuthMethod, RTSPResult, RTSPUrl};
use glib::translate::*;
use std::ptr;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[doc(alias = "GstRTSPConnection")]
    pub struct RTSPConnection(Shared<ffi::GstRTSPConnection>);

    match fn {
        ref => |ptr| { let _ = ptr; ptr },
        unref => |ptr| {
            unsafe { ffi::gst_rtsp_connection_free(ptr); }
        },
    }
}

unsafe impl Send for RTSPConnection {}
unsafe impl Sync for RTSPConnection {}

impl RTSPConnection {
    #[doc(alias = "gst_rtsp_connection_create")]
    pub fn create(url: &RTSPUrl) -> Result<RTSPConnection, RTSPResult> {
        unsafe {
            let mut conn = ptr::null_mut();
            let ret = ffi::gst_rtsp_connection_create(url.to_glib_none().0, &mut conn);
            if ret == ffi::GST_RTSP_OK {
                Ok(from_glib_full(conn))
            } else {
                Err(from_glib(ret))
            }
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_rtsp_connection_create_from_socket")]
    pub fn create_from_socket(
        socket: &gio::Socket,
        ip: &str,
        port: u16,
        initial_buffer: Option<&str>,
    ) -> Result<RTSPConnection, RTSPResult> {
        unsafe {
            let mut conn = ptr::null_mut();
            let ret = ffi::gst_rtsp_connection_create_from_socket(
                socket.to_glib_none().0,
                ip.to_glib_none().0,
                port,
                initial_buffer.to_glib_none().0,
                &mut conn,
            );
            if ret == ffi::GST_RTSP_OK {
                Ok(from_glib_full(conn))
            } else {
                Err(from_glib(ret))
            }
        }
    }

    #[doc(alias = "gst_rtsp_connection_accept")]
    pub fn accept(
        socket: &gio::Socket,
        cancellable: Option<&gio::Cancellable>,
    ) -> Result<RTSPConnection, RTSPResult> {
        unsafe {
            let mut conn = ptr::null_mut();
            let ret = ffi::gst_rtsp_connection_accept(
                socket.to_glib_none().0,
                &mut conn,
                cancellable.to_glib_none().0,
            );
            if ret == ffi::GST_RTSP_OK {
                Ok(from_glib_full(conn))
            } else {
                Err(from_glib(ret))
            }
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_rtsp_connection_connect_usec")]
    pub fn connect(&self, timeout: i64) -> Result<(), RTSPResult> {
        unsafe {
            let ret = ffi::gst_rtsp_connection_connect_usec(self.to_glib_none().0, timeout);
            if ret == ffi::GST_RTSP_OK {
                Ok(())
            } else {
                Err(from_glib(ret))
            }
        }
    }

    #[cfg(not(feature = "v1_18"))]
    #[doc(alias = "gst_rtsp_connection_connect")]
    pub fn connect(&self, timeout_secs: i64, timeout_usecs: i64) -> Result<(), RTSPResult> {
        unsafe {
            let mut timeout = glib::ffi::GTimeVal {
                tv_sec: timeout_secs as libc::c_long,
                tv_usec: timeout_usecs as libc::c_long,
            };
            let ret =
                ffi::gst_rtsp_connection_connect(self.to_glib_none().0, &mut timeout as *mut _);
            if ret == ffi::GST_RTSP_OK {
                Ok(())
            } else {
                Err(from_glib(ret))
            }
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_rtsp_connection_connect_with_response_usec")]
    pub fn connect_with_response(
        &self,
        timeout: i64,
        response: &RTSPMessage,
    ) -> Result<(), RTSPResult> {
        unsafe {
            let ret = ffi::gst_rtsp_connection_connect_with_response_usec(
                self.to_glib_none().0,
                timeout,
                response.to_glib_none().0,
            );
            if ret == ffi::GST_RTSP_OK {
                Ok(())
            } else {
                Err(from_glib(ret))
            }
        }
    }

    #[doc(alias = "gst_rtsp_connection_close")]
    pub fn close(&self) -> Result<(), RTSPResult> {
        unsafe {
            let ret = ffi::gst_rtsp_connection_close(self.to_glib_none().0);
            if ret == ffi::GST_RTSP_OK {
                Ok(())
            } else {
                Err(from_glib(ret))
            }
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_rtsp_connection_send_usec")]
    pub fn send(&self, message: &RTSPMessage, timeout: i64) -> Result<(), RTSPResult> {
        unsafe {
            let ret = ffi::gst_rtsp_connection_send_usec(
                self.to_glib_none().0,
                message.to_glib_none().0,
                timeout,
            );
            if ret == ffi::GST_RTSP_OK {
                Ok(())
            } else {
                Err(from_glib(ret))
            }
        }
    }

    #[cfg(not(feature = "v1_18"))]
    #[doc(alias = "gst_rtsp_connection_send")]
    pub fn send(
        &self,
        message: &RTSPMessage,
        timeout_secs: i64,
        timeout_usecs: i64,
    ) -> Result<(), RTSPResult> {
        unsafe {
            let mut timeout = glib::ffi::GTimeVal {
                tv_sec: timeout_secs as libc::c_long,
                tv_usec: timeout_usecs as libc::c_long,
            };
            let ret = ffi::gst_rtsp_connection_send(
                self.to_glib_none().0,
                message.to_glib_none().0,
                &mut timeout as *mut _,
            );
            if ret == ffi::GST_RTSP_OK {
                Ok(())
            } else {
                Err(from_glib(ret))
            }
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_rtsp_connection_send_messages_usec")]
    pub fn send_messages(
        &self,
        messages: &[RTSPMessage],
        timeout: i64,
    ) -> Result<usize, RTSPResult> {
        unsafe {
            let mut n_messages = messages.len();
            let messages_ptr = messages.as_ptr() as *mut _;
            let ret = ffi::gst_rtsp_connection_send_messages_usec(
                self.to_glib_none().0,
                messages_ptr,
                n_messages as u32,
                timeout,
            );
            if ret == ffi::GST_RTSP_OK {
                Ok(n_messages)
            } else {
                Err(from_glib(ret))
            }
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_rtsp_connection_receive_usec")]
    pub fn receive(&self, message: &RTSPMessage, timeout: i64) -> Result<(), RTSPResult> {
        unsafe {
            let ret = ffi::gst_rtsp_connection_receive_usec(
                self.to_glib_none().0,
                message.to_glib_none().0,
                timeout,
            );
            if ret == ffi::GST_RTSP_OK {
                Ok(())
            } else {
                Err(from_glib(ret))
            }
        }
    }

    #[cfg(not(feature = "v1_18"))]
    #[doc(alias = "gst_rtsp_connection_receive")]
    pub fn receive(
        &self,
        message: &RTSPMessage,
        timeout_secs: i64,
        timeout_usecs: i64,
    ) -> Result<(), RTSPResult> {
        unsafe {
            let mut timeout = glib::ffi::GTimeVal {
                tv_sec: timeout_secs as libc::c_long,
                tv_usec: timeout_usecs as libc::c_long,
            };
            let ret = ffi::gst_rtsp_connection_receive(
                self.to_glib_none().0,
                message.to_glib_none().0,
                &mut timeout as *mut _,
            );
            if ret == ffi::GST_RTSP_OK {
                Ok(())
            } else {
                Err(from_glib(ret))
            }
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_rtsp_connection_read_usec")]
    pub fn read(&self, data: &mut [u8], timeout: i64) -> Result<usize, RTSPResult> {
        unsafe {
            let mut size = data.len();
            let ret = ffi::gst_rtsp_connection_read_usec(
                self.to_glib_none().0,
                data.as_mut_ptr(),
                size,
                timeout,
            );
            if ret == ffi::GST_RTSP_OK {
                Ok(size)
            } else {
                Err(from_glib(ret))
            }
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_rtsp_connection_write_usec")]
    pub fn write(&self, data: &[u8], timeout: i64) -> Result<usize, RTSPResult> {
        unsafe {
            let mut size = data.len();
            let ret = ffi::gst_rtsp_connection_write_usec(
                self.to_glib_none().0,
                data.as_ptr(),
                size,
                timeout,
            );
            if ret == ffi::GST_RTSP_OK {
                Ok(size)
            } else {
                Err(from_glib(ret))
            }
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_rtsp_connection_poll_usec")]
    pub fn poll(&self, events: RTSPEvent, timeout: i64) -> Result<RTSPEvent, RTSPResult> {
        unsafe {
            let mut revents = 0;
            let ret = ffi::gst_rtsp_connection_poll_usec(
                self.to_glib_none().0,
                events.into_glib() as i32,
                &mut revents,
                timeout,
            );
            if ret == ffi::GST_RTSP_OK {
                Ok(from_glib(revents as ffi::GstRTSPEvent))
            } else {
                Err(from_glib(ret))
            }
        }
    }

    #[doc(alias = "gst_rtsp_connection_flush")]
    pub fn flush(&self, flush: bool) -> Result<(), RTSPResult> {
        unsafe {
            let ret = ffi::gst_rtsp_connection_flush(self.to_glib_none().0, flush.into_glib());
            if ret == ffi::GST_RTSP_OK {
                Ok(())
            } else {
                Err(from_glib(ret))
            }
        }
    }

    #[doc(alias = "gst_rtsp_connection_set_auth")]
    pub fn set_auth(
        &self,
        method: RTSPAuthMethod,
        user: &str,
        pass: &str,
    ) -> Result<(), RTSPResult> {
        unsafe {
            let ret = ffi::gst_rtsp_connection_set_auth(
                self.to_glib_none().0,
                method.into_glib(),
                user.to_glib_none().0,
                pass.to_glib_none().0,
            );
            if ret == ffi::GST_RTSP_OK {
                Ok(())
            } else {
                Err(from_glib(ret))
            }
        }
    }

    #[doc(alias = "gst_rtsp_connection_set_auth_param")]
    pub fn set_auth_param(&self, param: &str, value: &str) {
        unsafe {
            ffi::gst_rtsp_connection_set_auth_param(
                self.to_glib_none().0,
                param.to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gst_rtsp_connection_clear_auth_params")]
    pub fn clear_auth_params(&self) {
        unsafe {
            ffi::gst_rtsp_connection_clear_auth_params(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gst_rtsp_connection_set_proxy")]
    pub fn set_proxy(&self, host: &str, port: u32) -> Result<(), RTSPResult> {
        unsafe {
            let ret = ffi::gst_rtsp_connection_set_proxy(
                self.to_glib_none().0,
                host.to_glib_none().0,
                port,
            );
            if ret == ffi::GST_RTSP_OK {
                Ok(())
            } else {
                Err(from_glib(ret))
            }
        }
    }

    #[doc(alias = "gst_rtsp_connection_set_qos_dscp")]
    pub fn set_qos_dscp(&self, qos_dscp: u32) -> Result<(), RTSPResult> {
        unsafe {
            let ret = ffi::gst_rtsp_connection_set_qos_dscp(self.to_glib_none().0, qos_dscp);
            if ret == ffi::GST_RTSP_OK {
                Ok(())
            } else {
                Err(from_glib(ret))
            }
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_rtsp_connection_set_content_length_limit")]
    pub fn set_content_length_limit(&self, limit: u32) {
        unsafe {
            ffi::gst_rtsp_connection_set_content_length_limit(self.to_glib_none().0, limit);
        }
    }

    #[doc(alias = "gst_rtsp_connection_set_http_mode")]
    pub fn set_http_mode(&self, enable: bool) {
        unsafe {
            ffi::gst_rtsp_connection_set_http_mode(self.to_glib_none().0, enable.into_glib());
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_rtsp_connection_add_extra_http_request_header")]
    pub fn add_extra_http_request_header(&self, key: &str, value: &str) {
        unsafe {
            ffi::gst_rtsp_connection_add_extra_http_request_header(
                self.to_glib_none().0,
                key.to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gst_rtsp_connection_set_tunneled")]
    pub fn set_tunneled(&self, tunneled: bool) {
        unsafe {
            ffi::gst_rtsp_connection_set_tunneled(self.to_glib_none().0, tunneled.into_glib());
        }
    }

    #[doc(alias = "gst_rtsp_connection_is_tunneled")]
    pub fn is_tunneled(&self) -> bool {
        unsafe { from_glib(ffi::gst_rtsp_connection_is_tunneled(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_rtsp_connection_get_tunnelid")]
    pub fn get_tunnelid(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::gst_rtsp_connection_get_tunnelid(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_rtsp_connection_do_tunnel")]
    pub fn do_tunnel(&self, conn2: &RTSPConnection) -> Result<(), RTSPResult> {
        unsafe {
            let ret =
                ffi::gst_rtsp_connection_do_tunnel(self.to_glib_none().0, conn2.to_glib_none().0);
            if ret == ffi::GST_RTSP_OK {
                Ok(())
            } else {
                Err(from_glib(ret))
            }
        }
    }

    #[doc(alias = "gst_rtsp_connection_set_remember_session_id")]
    pub fn set_remember_session_id(&self, remember: bool) {
        unsafe {
            ffi::gst_rtsp_connection_set_remember_session_id(
                self.to_glib_none().0,
                remember.into_glib(),
            );
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_rtsp_connection_set_ignore_x_server_reply")]
    pub fn set_ignore_x_server_reply(&self, ignore: bool) {
        unsafe {
            ffi::gst_rtsp_connection_set_ignore_x_server_reply(
                self.to_glib_none().0,
                ignore.into_glib(),
            );
        }
    }

    #[doc(alias = "gst_rtsp_connection_get_url")]
    pub fn get_url(&self) -> Option<RTSPUrl> {
        unsafe { from_glib_none(ffi::gst_rtsp_connection_get_url(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_rtsp_connection_get_ip")]
    pub fn get_ip(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::gst_rtsp_connection_get_ip(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_rtsp_connection_set_ip")]
    pub fn set_ip(&self, ip: &str) {
        unsafe {
            ffi::gst_rtsp_connection_set_ip(self.to_glib_none().0, ip.to_glib_none().0);
        }
    }

    #[doc(alias = "gst_rtsp_connection_get_read_socket")]
    pub fn get_read_socket(&self) -> Option<gio::Socket> {
        unsafe {
            from_glib_none(ffi::gst_rtsp_connection_get_read_socket(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_connection_get_write_socket")]
    pub fn get_write_socket(&self) -> Option<gio::Socket> {
        unsafe {
            from_glib_none(ffi::gst_rtsp_connection_get_write_socket(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_rtsp_connection_next_timeout_usec")]
    pub fn next_timeout(&self) -> i64 {
        unsafe { ffi::gst_rtsp_connection_next_timeout_usec(self.to_glib_none().0) }
    }

    #[doc(alias = "gst_rtsp_connection_reset_timeout")]
    pub fn reset_timeout(&self) -> Result<(), RTSPResult> {
        unsafe {
            let ret = ffi::gst_rtsp_connection_reset_timeout(self.to_glib_none().0);
            if ret == ffi::GST_RTSP_OK {
                Ok(())
            } else {
                Err(from_glib(ret))
            }
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_rtsp_connection_get_tls")]
    pub fn get_tls(&self) -> Option<gio::TlsConnection> {
        unsafe {
            let mut error = ptr::null_mut();
            let tls = ffi::gst_rtsp_connection_get_tls(self.to_glib_none().0, &mut error);
            if error.is_null() {
                from_glib_none(tls)
            } else {
                glib::ffi::g_error_free(error);
                None
            }
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_rtsp_connection_set_tls_validation_flags")]
    pub fn set_tls_validation_flags(&self, flags: gio::TlsCertificateFlags) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_connection_set_tls_validation_flags(
                self.to_glib_none().0,
                flags.into_glib(),
            ))
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_rtsp_connection_get_tls_validation_flags")]
    pub fn get_tls_validation_flags(&self) -> gio::TlsCertificateFlags {
        unsafe {
            from_glib(ffi::gst_rtsp_connection_get_tls_validation_flags(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_rtsp_connection_set_tls_database")]
    pub fn set_tls_database(&self, database: Option<&gio::TlsDatabase>) {
        unsafe {
            ffi::gst_rtsp_connection_set_tls_database(
                self.to_glib_none().0,
                database.to_glib_none().0,
            );
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_rtsp_connection_get_tls_database")]
    pub fn get_tls_database(&self) -> Option<gio::TlsDatabase> {
        unsafe {
            from_glib_none(ffi::gst_rtsp_connection_get_tls_database(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_rtsp_connection_set_tls_interaction")]
    pub fn set_tls_interaction(&self, interaction: Option<&gio::TlsInteraction>) {
        unsafe {
            ffi::gst_rtsp_connection_set_tls_interaction(
                self.to_glib_none().0,
                interaction.to_glib_none().0,
            );
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_rtsp_connection_get_tls_interaction")]
    pub fn get_tls_interaction(&self) -> Option<gio::TlsInteraction> {
        unsafe {
            from_glib_none(ffi::gst_rtsp_connection_get_tls_interaction(
                self.to_glib_none().0,
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rtsp_connection_create() {
        crate::init().unwrap();

        let (result, url) = RTSPUrl::parse("rtsp://localhost:554/test");
        assert_eq!(result, crate::RTSPResult::Ok);
        let url = url.unwrap();
        let conn = RTSPConnection::create(&url);
        assert!(conn.is_ok());
    }

    #[test]
    fn test_rtsp_connection_tunneling() {
        crate::init().unwrap();

        let (result, url) = RTSPUrl::parse("rtsp://localhost:554/test");
        assert_eq!(result, crate::RTSPResult::Ok);
        let url = url.unwrap();
        let conn = RTSPConnection::create(&url).unwrap();

        conn.set_tunneled(true);
        assert!(conn.is_tunneled());

        conn.set_tunneled(false);
        assert!(!conn.is_tunneled());
    }

    #[test]
    fn test_rtsp_connection_auth() {
        crate::init().unwrap();

        let (result, url) = RTSPUrl::parse("rtsp://localhost:554/test");
        assert_eq!(result, crate::RTSPResult::Ok);
        let url = url.unwrap();
        let conn = RTSPConnection::create(&url).unwrap();

        conn.set_auth(RTSPAuthMethod::Basic, "user", "pass")
            .unwrap();
        conn.set_auth_param("realm", "test_realm");
        conn.clear_auth_params();
    }

    #[test]
    fn test_rtsp_connection_proxy() {
        crate::init().unwrap();

        let (result, url) = RTSPUrl::parse("rtsp://localhost:554/test");
        assert_eq!(result, crate::RTSPResult::Ok);
        let url = url.unwrap();
        let conn = RTSPConnection::create(&url).unwrap();

        let result = conn.set_proxy("proxy.example.com", 8080);
        assert!(result.is_ok());
    }

    #[test]
    fn test_rtsp_connection_qos() {
        crate::init().unwrap();

        let (result, url) = RTSPUrl::parse("rtsp://localhost:554/test");
        assert_eq!(result, crate::RTSPResult::Ok);
        let url = url.unwrap();
        let conn = RTSPConnection::create(&url).unwrap();

        // Note: set_qos_dscp may fail if the connection is not yet established
        // This is expected behavior
        let _ = conn.set_qos_dscp(46); // EF DSCP value
    }

    #[test]
    fn test_rtsp_connection_http_mode() {
        crate::init().unwrap();

        let (result, url) = RTSPUrl::parse("rtsp://localhost:554/test");
        assert_eq!(result, crate::RTSPResult::Ok);
        let url = url.unwrap();
        let conn = RTSPConnection::create(&url).unwrap();

        conn.set_http_mode(true);
        conn.set_remember_session_id(true);
    }

    #[test]
    fn test_rtsp_connection_ip() {
        crate::init().unwrap();

        let (result, url) = RTSPUrl::parse("rtsp://localhost:554/test");
        assert_eq!(result, crate::RTSPResult::Ok);
        let url = url.unwrap();
        let conn = RTSPConnection::create(&url).unwrap();

        conn.set_ip("192.168.1.1");
        // Note: get_ip() may return None until connection is established
    }

    #[test]
    #[cfg(feature = "v1_18")]
    fn test_rtsp_connection_timeout() {
        crate::init().unwrap();

        let (result, url) = RTSPUrl::parse("rtsp://localhost:554/test");
        assert_eq!(result, crate::RTSPResult::Ok);
        let url = url.unwrap();
        let conn = RTSPConnection::create(&url).unwrap();

        let timeout = conn.next_timeout();
        assert!(timeout >= 0);

        let result = conn.reset_timeout();
        assert!(result.is_ok());
    }
}
