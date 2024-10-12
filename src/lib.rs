// Copyright (c) 2024 Felipe Torres González. All rights reserved.
//
// This work is licensed under the terms of the MIT license.
// For a copy, see <https://opensource.org/licenses/MIT>.

//! A client for [Mailjet's API][mapi] using [reqwest][reqwest] and [tracing][tracing].
//!
//! # Description
//!
//! This library crate includes an implementation written in Rust for a client of Mailjet's API. As of today, there
//! is no official Rust client released by the company behind Mailjet. Though there exist a few Rust
//! clients out in [](crates.io), none of them include some of the features that I needed, so here it goes another
//! client implementation in Rust!
//!
//! ## Main Features
//!
//! - **Tracing support via [Tracing](https://crates.io/crates/tracing)**: the library code includes tracing calls
//!   using `Tracing`'s API.
//! - **Usage of [Reqwest](https://crates.io/crates/reqwest) as internal HTTP client**: `Reqwest` is my crate of
//!   choice for this use-case scenarios. The crate `Reqwest-tracing` is also added to enable tracing support for
//!   the internal HTTP client.
//! - **High level of test coverage** and support for CI. Given that I aim to include this crate into another service
//!   that needs a high level of reliavility, not including a proper set of tests was a non-go.
//!
//! ## Caveats
//! Offering a full client implementation is not my initial plan, so **don't expect a full 1:1 Rust implementation of
//! the existing API** clients provided by Mailjet. This project is open-sourced, so if you need to cover some
//! missing endpoint of Mailjet's API, feel free to open a new Issue describing your needs. Either if you plan to
//! develop it by yourself, or you need somebody else to do it, it will be good to know that there's interest on
//! adding such missing feature to the client.
//!
//! # Library Structure
//!
//! The library includes a module that implements the logic to send and receive HTTP requests to the external REST API.
//! Depending on the target endpoint of that API, the request body parameters and the response object will vary.
//! The library's code attempts to abstract such thing as much as possible using traits. Visit the documentation page
//! for the module [crate::data_objects] to read a full explanation. In brief, two main objects are distinguish:
//!
//! ## Response Objects
//!
//! Responses object vary depending on the used endpoint. Whilst endpoints that implement the API v3.1 include
//! a lot of information about errors that might ocurred; endpoints that implement the API v3.0 only include a brief
//! error code when errors ocurred. Aside from that, the returned objects as the payload of the response vary quite
//! a lot.
//!
//! To offer a client as homogeneous as possible, all the client's member functions return a trait object:
//! [crate::data_objects::ResponseObject]. Then, `struct`s implement a particular response object, and you'll need to
//! cast from the trait object to a particular object based on the endpoint that you are using.
//!
//! ## Request Objects
//!
//! The same applies to request objects: client's member function expect a trait object that implements the trait
//! [crate::data_objects::RequestObject]. Then, is the client's implementation who should cast such thing to the
//! particular object expected by the external endpoint.
//!
//! # Usage
//!
//! The current client only supports sending emails, either using Mailjet's API v3 or v3.1.
//!
//! To start sending emails, instantiate the client [crate::MailjetClient] either directly using
//! [crate::MailjetClient::new] or (best choice) using the object [crate::MailjetClientBuilder], which eases the
//! construction of a new client.
//!
//! At a bare minimum, you have to provide your API credentials to the client. Check [crate::MailjetClient::new]'s doc
//! to get more details about those credentials. The rest of parameters are optional.
//!
//! ```verbatim
//! let client = MailjetClientBuilder::new(api_user, api_key)
//!     .build()
//!     .expect("Failed to build a new Mailjet client");
//! ```
//!
//! The builder object implements a fluent interface so add as many attributes as you wish to the builder object, and
//! once your done with it, call `.build()` to complete the process.
//!
//! Then, composed a new email using either [crate::data_objects::SimpleMessage] (for API v3.0) or
//! [crate::data_objects::SendEmailParams] (for API v3.1) and call the send method:
//!
//! ```verbatim
//! let message = //...;
//!
//! let response = client.send_email(&message).await?;
//! ```
//!
//! Do you need more examples? Visit the *examples* folder to get a detailed view of the usage of this crate.
//!
//!
//! [mapi]: https://dev.mailjet.com/email/reference/overview/
//! [reqwest]: https://crates.io/crates/reqwest
//! [tracing]: https://crates.io/crates/tracing

/// This module includes all the object definitions need to interact with the external API data objects.
///
/// # Description
///
/// The objects defined in the REST [API][mapi] are directly translated into Rust's types using naming conventions of
/// Rust (snake case for `struct`'s members). Be aware of that, because some names of the objects defined in the REST
/// API don't follow any known convention. _PascalCase_ seems to be the usual choice for them, but some object's
/// attributes get awkward names such as ***DnsID***. The objects defined here in strictly follow the snake case naming
/// rules, and `serde`'s macros are used to rename the fields (see [this](https://serde.rs/field-attrs.html#rename)).
///
/// # Simple Translation Table
///
/// This is a simple table that links what objects you should use from this crate when you target a particular
/// endpoint of the external API:
///
/// |    Endpoint   | Request object | Response object |
/// |:-------------:|:---------------|:----------------|
/// | `/send` v3    | [crate::data_objects::SimpleMessage] | [crate::data_objects::MessageObject] |
/// | `/send` v3.1  | [crate::data_objects::SendEmailParams] | [crate::data_objects::SendResponseObject] |
///
/// # Mailjet REST API responses
///
/// All the responses of the API are defined this way:
/// - A status code, which stems from regular HTTP response codes, to indicate the success or failure of an API
///   request. From their API docs:
///
/// ```verbatim
/// Overall:
/// - Codes in the 2xx range indicate that the request was processed successfully.
/// - Codes in the 4xx range indicate that there was an error with the request (e.g. unauthorized access, a
///   required parameter is missing etc.).
/// - Codes in the 5xx range indicate that there is an issue with Mailjet's servers. Those are quite rare.
/// ```
///
///   Check out their [docs](https://dev.mailjet.com/email/reference/overview/errors/) to read a full explanation
///   of each status or error code.
/// - An array of objects. Each endpoint of the REST API defines a different response object.
/// - A couple of integers to inform about the amount of matches that the query generated, and the amount of items
///   returned in the response. This is useful for queries that generate high number of matches (hundreds), and
///   need pagination. However, this is gently opaqued from the implementation of this client of the API, so any
///   user needs to worry about it. **A client's call will return all the matches at once.**
///
/// Only the two first items of the list are included in the response objects returned by this client. To cope
/// with the problem of a non-homogeneous response data object definition, the trait
/// [crate::data_objects::ResponseObject] is defined in this crate. This way, a common type
/// [crate::data_objects::Response] is returned by every call of the client that wraps a GET method of the REST API.
///
/// [mapi]: https://dev.mailjet.com/email/reference/overview/
pub mod data_objects {

    use std::any::Any;

    /// An alias to ease developers difficult lives: a [Vec] of objects that implement [ResponseObject].
    pub type ObjectType = Vec<Box<dyn ResponseObject>>;

    /// Common response type for GET methods of Mailjet's API.
    ///
    /// # Description
    ///
    /// This object encapsulates the response of any endpoint of Mailjet's API. If [Response::status_code] is a
    /// success code, the payload will be populated with a particular response object (depends on the used endpoint).
    ///
    /// The [Response::payload] includes the response data from the API. The following list matches APIs's objects
    /// returned as responses and the data objects defined by this crate:
    /// - `/sender (GET)` -> [crate::data_objects::responses::Sender]
    #[derive(Debug)]
    pub struct Response {
        pub status_code: u16,
        pub payload: Option<ObjectType>,
    }

    /// Trait that identifies any object that is returned by Mailjet's REST API.
    ///
    /// # Description
    ///
    /// Every response from the API shall include a matching type in this crate that implements this trait.
    /// This is mandatory to return a generic [Response] from all the client calls provided by this client.
    pub trait ResponseObject: std::fmt::Debug {}

    /// Trait that identifies any object that is used as parameters for a request to the external API.
    pub trait RequestObject: std::fmt::Debug {
        fn as_any(&self) -> &dyn Any;
    }

    mod responses;

    pub use responses::*;

    mod message;
    pub use message::{
        Attachment, Message, MessageBuilder, MessageProperty, NameAndEmail, SimpleMessage, Track,
    };

    mod request;
    pub use request::{ContactQuery, SendEmailParams};
}

// Re-export
pub use data_objects::Response;

mod mailjet_client;
// Re-export the client.
pub use mailjet_client::MailjetClient;

mod error;
pub use error::ClientError;

mod mailjet_api;
pub use mailjet_api::ApiVersion;

mod mailjet_client_builder;
pub use mailjet_client_builder::MailjetClientBuilder;
