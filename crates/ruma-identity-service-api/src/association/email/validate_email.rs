//! `POST /_matrix/identity/*/validate/email/submitToken`
//!
//! Validate an email ID after creation of a session.

pub mod v2 {
    //! `/v2/` ([spec])
    //!
    //! [spec]: https://spec.matrix.org/v1.4/identity-service-api/#post_matrixidentityv2validateemailsubmittoken

    use ruma_common::{
        api::{request, response, Metadata},
        metadata, ClientSecret, SessionId,
    };

    const METADATA: Metadata = metadata! {
        method: POST,
        rate_limited: false,
        authentication: AccessToken,
        history: {
            1.0 => "/_matrix/identity/v2/validate/email/submitToken",
        }
    };

    /// Request type for the `validate_email` endpoint.
    #[request]
    pub struct Request<'a> {
        /// The session ID, generated by the `requestToken` call.
        pub sid: &'a SessionId,

        /// The client secret that was supplied to the `requestToken` call.
        pub client_secret: &'a ClientSecret,

        /// The token generated by the `requestToken` call and emailed to the user.
        pub token: &'a str,
    }

    /// Response type for the `validate_email` endpoint.
    #[response]
    pub struct Response {
        /// Whether the validation was successful or not.
        pub success: bool,
    }

    impl<'a> Request<'a> {
        /// Create a new `Request` with the given session ID, client secret and token.
        pub fn new(sid: &'a SessionId, client_secret: &'a ClientSecret, token: &'a str) -> Self {
            Self { sid, client_secret, token }
        }
    }

    impl Response {
        /// Create a new `Response` with the success status.
        pub fn new(success: bool) -> Self {
            Self { success }
        }
    }
}
