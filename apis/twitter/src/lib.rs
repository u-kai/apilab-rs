#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
pub mod apis {
    pub(super) mod auth {
        pub mod auth;
        pub mod oauth1;
    }
    pub mod client;
    pub mod query {
        pub(super) mod query;
        pub mod query_builder;
    }
    pub mod responses {
        pub(super) mod example;
        pub(super) mod search;
        pub(super) mod impls {
            pub mod search;
        }
    }
}
