//! `POST /_matrix/client/*/account/3pid/add`
//!
//! Add contact information to a user's account

pub mod v3 {
    //! `/v3/` ([spec])
    //!
    //! [spec]: https://spec.matrix.org/v1.4/client-server-api/#post_matrixclientv3account3pidadd

    use ruma_common::{
        api::{request, response, Metadata},
        metadata, ClientSecret, SessionId,
    };

    use crate::uiaa::{AuthData, IncomingAuthData, UiaaResponse};

    const METADATA: Metadata = metadata! {
        method: POST,
        rate_limited: true,
        authentication: AccessToken,
        history: {
            1.0 => "/_matrix/client/r0/account/3pid/add",
            1.1 => "/_matrix/client/v3/account/3pid/add",
        }
    };

    /// Request type for the `add_3pid` endpoint.
    #[request(error = UiaaResponse)]
    pub struct Request<'a> {
        /// Additional information for the User-Interactive Authentication API.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub auth: Option<AuthData<'a>>,

        /// Client-generated secret string used to protect this session.
        pub client_secret: &'a ClientSecret,

        /// The session identifier given by the identity server.
        pub sid: &'a SessionId,
    }

    /// Response type for the `add_3pid` endpoint.
    #[response(error = UiaaResponse)]
    #[derive(Default)]
    pub struct Response {}

    impl<'a> Request<'a> {
        /// Creates a new `Request` with the given client secret and session identifier.
        pub fn new(client_secret: &'a ClientSecret, sid: &'a SessionId) -> Self {
            Self { auth: None, client_secret, sid }
        }
    }

    impl Response {
        /// Creates an empty `Response`.
        pub fn new() -> Self {
            Self {}
        }
    }
}
