use std::{thread, time};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TwitterRateLimit {
    code: f64,
    message: String,
}
impl TwitterRateLimit {
    pub fn sleep_until_twitter_api_able(self) {
        thread::sleep(time::Duration::from_secs(900))
    }
}
