/// mime types for requests and responses

pub mod responses {
    use hyper::mime::*;

    // The macro is called per-operation to beat the recursion limit

    lazy_static! {
        /// Create Mime objects for the response content types for UserLoginPost
        pub static ref USER_LOGIN_POST_: Mime = "*/*".parse().unwrap();
    }

}

pub mod requests {
    use hyper::mime::*;

}
