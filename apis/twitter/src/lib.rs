#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
pub mod apis {
    pub(super) mod auth;
    pub mod client;
    pub mod query_builder;
    pub mod responses {
        pub mod data;
        pub(super) mod example;
        pub mod meta;
        pub mod search;
        pub mod twitter_response;
    }
}
