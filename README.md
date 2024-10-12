# A Rust client for Mailjet's API

[![codecov](https://codecov.io/github/felipet/mailjet_client/graph/badge.svg?token=EONZFSSFX1)](https://codecov.io/github/felipet/mailjet_client)
![CI](https://github.com/felipet/mailjet_client/actions/workflows/rust.yml/badge.svg)

This is another implementation of a client written in Rust for [Mailjet's API](https://dev.mailjet.com/email/reference/overview/).

**What's [Mailjet](https://www.mailjet.com/)?** Mailjet is a web service that offers a platform for sending emails to many recipients using templates, and automations. Though they offer a nice and powerful web interface for their service, they also offer a REST API along a free tier for people sending low amounts of emails per month. This is a perfect combination for people that might need to deliver a few emails from a web service written in Rust (like me). However, they don't provide a Rust Client.

As the day of writting this, there are 2 available clients for the API written in Rust:
- [Mailjet-rs](https://crates.io/crates/mailjet-rs)
- [Mailjet_api_wrapper](https://crates.io/crates/mailjet_api_wrapper)

However, none of those include some of the features that I need as I aim to integrate this library into an existing web service implemented using [Actix-web](https://actix.rs/). That's the main reason to write another client implementation. Check out this implementation's features to assess whether this client suits better your needs than any of the other existing crates.

## Features

- **Tracing support via [Tracing](https://crates.io/crates/tracing)**: the library code includes tracing calls using `Tracing`'s API.
- **Usage of [Reqwest](https://crates.io/crates/reqwest) as internal HTTP client**: `Reqwest` is my crate of choice for this use-case scenarios. The crate `Reqwest-tracing` is also addedd to enable tracing support for the internal HTTP client.
- **High level of test coverage** and support for CI. Given that I aim to include this crate into another service that needs a high level of reliavility, not including a proper set of tests was a non-go.

## Caveats

Offering a full client implementation is not my initial plan, so **don't expect a full 1:1 Rust implementation of the existing API** clients provided by Mailjet. This project is open-sourced, so if you need to cover some missing endpoint of Mailjet's API, feel free to open a new Issue describing your needs. Either if you plan to develop it by yourself, or you need somebody else to do it, it will be good to know that there's interest on adding such missing feature to the client.

## Running The Integration Tests

This crate includes some integration tests to verify that the developed client works as expected. Endpoints of the external REST API that implement the latest version (v3.1) include an option to run in *sandbox mode*. That is really convenient to run tests without issuing real emails. However, most of the endpoints don't implement the latest version of the API, hence they don't allow using the *sandbox mode*.

The crate [wiremock](https://crates.io/crates/wiremock) is used to mock the external API responses for the integration tests. However, some tests do actually send requests to the real API server. These tests are marked with the attribute `#[ignore]`, and they are only planned to be executed during merge requests or before issuing a new release of the crate.

If you plan to contribute to this crate, or you simple aim to run all the integration tests, you'll need an account and an API key. Visit [Mailjet's Docs](https://app.mailjet.com/account/apikeys) to get more details about it. Once you've got a pair of API keys, and a validated email from which your enabled to send emails, set this environment variables in your shell session:

```bash
# This is what they call API KEY
export MAILJET_API_USER=<hash>
# This is what they call SECRET KEY
export MAILJET_API_KEY=<hash>
# This address must be registered in your Mailjet account
export MAILJET_FROM_EMAIL=<email address>
```

Tests include tracing support. However, it is muted by default. If you need to enable the log, use the following environment variable:

```bash
export TEST_LOG=debug
```

Finally, export an email address to which you aim to send a test message:

```bash
export MAILJET_TEST_RECIPIENT=<email address>
```

If that variable is not set, emails will be send to a dummy account provided by Mailjet (pilot@mailjet.com).

Use any valid value defined in the **tracing** crate as a level filter. Finally, a regular call to `cargo test` would run only the tests that use the mock server and the unit tests. If you need to run the set of tests that access Mailjet's API run: `cargo test -- --include-ignored`. You might also add a test filter to run only a particular ignored test. Bear in mind that running those tests will increase your sent email counter.

## Contributing

If you are interesting in contributing to this repository, feel free to contact me before starting to type new code as a crazy monkey.
