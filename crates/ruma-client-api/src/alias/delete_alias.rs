//! `DELETE /_matrix/client/*/directory/room/{roomAlias}`
//!
//! Remove an alias from a room.

pub mod v3 {
    //! `/v3/` ([spec])
    //!
    //! [spec]: https://spec.matrix.org/v1.4/client-server-api/#delete_matrixclientv3directoryroomroomalias

    use ruma_common::{
        api::{request, response, Metadata},
        metadata, RoomAliasId,
    };

    const METADATA: Metadata = metadata! {
        method: DELETE,
        rate_limited: false,
        authentication: AccessToken,
        history: {
            1.0 => "/_matrix/client/r0/directory/room/:room_alias",
            1.1 => "/_matrix/client/v3/directory/room/:room_alias",
        }
    };

    /// Request type for the `delete_alias` endpoint.
    #[request(error = crate::Error)]
    pub struct Request<'a> {
        /// The room alias to remove.
        #[ruma_api(path)]
        pub room_alias: &'a RoomAliasId,
    }

    /// Response type for the `delete_alias` endpoint.
    #[response(error = crate::Error)]
    #[derive(Default)]
    pub struct Response {}

    impl<'a> Request<'a> {
        /// Creates a new `Request` with the given room alias.
        pub fn new(room_alias: &'a RoomAliasId) -> Self {
            Self { room_alias }
        }
    }

    impl Response {
        /// Creates an empty `Response`.
        pub fn new() -> Self {
            Self {}
        }
    }
}
