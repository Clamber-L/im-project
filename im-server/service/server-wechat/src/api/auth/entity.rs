use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SignTokenParam {
    pub redirect_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignTokenResponse {
    pub app_id: String,
    pub nonce_str: String,
    pub signature: String,
    pub timestamp: String,
    pub redirect_url: String,
}

impl SignTokenResponse {
    pub fn new(
        app_id: &str,
        nonce_str: String,
        signature: String,
        timestamp: String,
        redirect_url: String,
    ) -> Self {
        Self {
            app_id: app_id.into(),
            nonce_str,
            signature,
            timestamp,
            redirect_url,
        }
    }
}
