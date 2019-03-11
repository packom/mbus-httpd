/// mime types for requests and responses

pub mod responses {
    use hyper::mime::*;

    // The macro is called per-operation to beat the recursion limit
    /// Create Mime objects for the response content types for Api
    lazy_static! {
        pub static ref API_OK: Mime = "text/plain".parse().unwrap();
    }
    /// Create Mime objects for the response content types for Api
    lazy_static! {
        pub static ref API_NOT_FOUND: Mime = "text/plain".parse().unwrap();
    }
    /// Create Mime objects for the response content types for Get
    lazy_static! {
        pub static ref GET_OK: Mime = "text/plain".parse().unwrap();
    }
    /// Create Mime objects for the response content types for Get
    lazy_static! {
        pub static ref GET_BAD_REQUEST: Mime = "text/plain".parse().unwrap();
    }
    /// Create Mime objects for the response content types for Get
    lazy_static! {
        pub static ref GET_NOT_FOUND: Mime = "text/plain".parse().unwrap();
    }
    /// Create Mime objects for the response content types for Hat
    lazy_static! {
        pub static ref HAT_OK: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for Hat
    lazy_static! {
        pub static ref HAT_NOT_FOUND: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for HatOff
    lazy_static! {
        pub static ref HAT_OFF_NOT_FOUND: Mime = "text/plain".parse().unwrap();
    }
    /// Create Mime objects for the response content types for HatOn
    lazy_static! {
        pub static ref HAT_ON_NOT_FOUND: Mime = "text/plain".parse().unwrap();
    }
    /// Create Mime objects for the response content types for Scan
    lazy_static! {
        pub static ref SCAN_OK: Mime = "text/plain".parse().unwrap();
    }
    /// Create Mime objects for the response content types for Scan
    lazy_static! {
        pub static ref SCAN_BAD_REQUEST: Mime = "text/plain".parse().unwrap();
    }
    /// Create Mime objects for the response content types for Scan
    lazy_static! {
        pub static ref SCAN_NOT_FOUND: Mime = "text/plain".parse().unwrap();
    }

}

pub mod requests {
    use hyper::mime::*;

}
