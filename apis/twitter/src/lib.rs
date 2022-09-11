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
        pub(super) mod rate_limit;
    }
}
