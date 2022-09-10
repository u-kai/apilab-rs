pub const SIGNATURE_METHOD: &'static str = "HMAC-SHA1";

#[cfg(not(test))]
pub fn gen_timestamp() -> String {
    use chrono::Utc;
    Utc::now().timestamp().to_string()
}

#[cfg(test)]
pub fn gen_timestamp() -> String {
    String::from("1600000000")
}
