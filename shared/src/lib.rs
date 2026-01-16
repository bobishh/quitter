pub mod tracker {
    include!(concat!(env!("OUT_DIR"), "/tracker.rs"));
}

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use prost::Message;

impl tracker::TrackerState {
    pub fn encode_to_url(&self) -> String {
        let mut buf = Vec::new();
        self.encode(&mut buf).unwrap();
        URL_SAFE_NO_PAD.encode(buf)
    }

    pub fn decode_from_url(s: &str) -> Result<Self, String> {
        let buf = URL_SAFE_NO_PAD.decode(s).map_err(|e| e.to_string())?;
        Self::decode(&buf[..]).map_err(|e| e.to_string())
    }
}
