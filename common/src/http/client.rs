use reqwest::RequestBuilder;

pub trait HttpClient {
    fn get(&self) -> RequestBuilder;
    fn get_with_param<P: ToUrlParameter>(&self, param: P) -> RequestBuilder;
    fn post_with_json<P: ToJson>(&self, param: P) -> RequestBuilder;
    fn post_with_form_param<P: ToFormUrlEncodeParameter>(&self, param: P) -> RequestBuilder;
}

pub trait ToUrlParameter {
    fn to_parameter(&self) -> String;
}
pub trait ToJson {
    fn to_json(&self) -> String;
}
pub trait ToFormUrlEncodeParameter {
    fn to_parameter(&self) -> String;
}
