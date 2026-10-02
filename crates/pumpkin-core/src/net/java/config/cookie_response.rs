#[allow(clippy::wildcard_imports)]
use super::*;

impl JavaClient {
    pub fn handle_config_cookie_response(&self, packet: &SConfigCookieResponse<'_>) {
        debug!(
            "Received cookie_response[config]: key: \"{}\", has_payload: \"{}\", payload_length: \"{:?}\"",
            packet.key,
            packet.has_payload,
            packet.payload.as_ref().map(|payload| payload.len()),
        );
    }
}
