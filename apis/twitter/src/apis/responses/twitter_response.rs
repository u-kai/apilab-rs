pub trait TwitterResponse {}
#[macro_export]
macro_rules! attach_twitter_response {
    ($ ( $t:ident ),*) => {
    use crate::apis::responses::twitter_response::TwitterResponse;
       $(
        impl TwitterResponse for $t {}
       )*
    };
    ($ ( $t:ident ),+,) => {
    use crate::apis::responses::twitter_response::TwitterResponse;
       $(
        impl TwitterResponse for $t {}
       )*
    };
}
