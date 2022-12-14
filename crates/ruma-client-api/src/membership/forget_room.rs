//! `POST /_matrix/client/*/rooms/{roomId}/forget`
//!
//! Forget a room.

pub mod v3 {
    //! `/v3/` ([spec])
    //!
    //! [spec]: https://spec.matrix.org/v1.4/client-server-api/#post_matrixclientv3roomsroomidforget

    use ruma_common::{
        api::{request, response, Metadata},
        metadata, RoomId,
    };

    const METADATA: Metadata = metadata! {
        method: POST,
        rate_limited: true,
        authentication: AccessToken,
        history: {
            1.0 => "/_matrix/client/r0/rooms/:room_id/forget",
            1.1 => "/_matrix/client/v3/rooms/:room_id/forget",
        }
    };

    /// Request type for the `forget_room` endpoint.
    #[request(error = crate::Error)]
    pub struct Request<'a> {
        /// The room to forget.
        #[ruma_api(path)]
        pub room_id: &'a RoomId,
    }

    /// Response type for the `forget_room` endpoint.
    #[response(error = crate::Error)]
    #[derive(Default)]
    pub struct Response {}

    impl<'a> Request<'a> {
        /// Creates a new `Request` with the given room id.
        pub fn new(room_id: &'a RoomId) -> Self {
            Self { room_id }
        }
    }

    impl Response {
        /// Creates an empty `Response`.
        pub fn new() -> Self {
            Self {}
        }
    }
}
