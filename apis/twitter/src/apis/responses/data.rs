use serde::{Deserialize, Serialize};

use super::twitter_response::TwitterResponse;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TwitterApiResponseData<T: TwitterResponse> {
    data: T,
}
impl<T> TwitterApiResponseData<T>
where
    T: TwitterResponse,
{
    //pub fn new(response: &str) -> Result<Self> {
    //serde_json::from_str(response)
    //}
}
