//! Tracing events that replace counters and timing metrics.
//!
//! Every event below is a plain `tracing` event; aggregate them with any subscriber or layer
//! (for example, filter on target and sum the fields).  Durations are in seconds.
//!
//! | Target | Level | Message | Fields |
//! |---|---|---|---|
//! | `carpa::client` | `DEBUG` | `request_completed` | `duration_seconds: f64`, `ok: bool` |
//! | `carpa::client` | `WARN` | `retry` | `error` (Display), `attempt: usize` (1 for the first retry), `backoff_seconds: f64` |
//! | `carpa::client` | `INFO` | `providers_configured` | `providers` (base URLs in priority order, comma-separated), `provider_count: usize` |
//! | `carpa::client` | `WARN` | `provider_switch` | `operation`, `from`, `to` (base URLs), `from_index: usize`, `to_index: usize` (0 is the first provider), `provider_count: usize`, `reason` (`out_of_credit`), `http.status_code: Option<u16>`, `error_code` (when the error carried one), `request_id: Option<&str>` |
//! | `carpa::client` | `DEBUG` | `provider_already_switched` | the `provider_switch` fields |
//! | `carpa::client` | `ERROR` | `providers_exhausted` | `operation`, `provider` (the last base URL), `provider_count: usize`, `http.status_code: Option<u16>` |
//! | `carpa::stream` | `TRACE` | `event` | (none) |
//! | `carpa::stream` | `TRACE` | `bytes` | `bytes: u64` |
//! | `carpa::stream` | `DEBUG` | `first_byte` | `ttfb_seconds: f64` |
//! | `carpa::stream` | `DEBUG` | `completed` | `duration_seconds: f64` |
//! | `carpa::stream` | `DEBUG` | `error` | (none) |
//! | `carpa::usage` | `INFO` | `usage` | `model`, `input_tokens: u64`, `output_tokens: u64`, `cache_read_input_tokens: u64`, `cache_creation_input_tokens: u64`, `cost_micro_cents: Option<u64>` (debug-formatted; `None` for unknown models) |
//!
//! `carpa::client` `request_completed` fires once per API call and provider (after any
//! retries), so a request resent on a fallback provider reports twice.  `provider_switch` fires
//! once per move of the shared provider position, and is the count of provider switches; a
//! concurrent request out of credit on a provider the client already left logs
//! `provider_already_switched` instead.  The
//! `carpa::usage` event fires once per completed `send` call and once per stream that
//! reaches `message_stop`.

use std::time::Duration;

use crate::error::{Error, ErrorCode, ErrorKind};

/// One API call finished after `duration`; `ok` says whether it succeeded.
pub(crate) fn request_completed(duration: Duration, ok: bool) {
    tracing::debug!(
        target: "carpa::client",
        duration_seconds = duration.as_secs_f64(),
        ok,
        "request_completed"
    );
}

/// A client was given fallback providers; `providers` holds every base URL in priority order.
pub(crate) fn providers_configured(providers: &[&str]) {
    tracing::info!(
        target: "carpa::client",
        providers = %providers.join(", "),
        provider_count = providers.len(),
        "providers_configured"
    );
}

/// A Messages request found the provider at `from_index` out of credit, and moves to `to_index`.
pub(crate) struct ProviderSwitch<'a> {
    pub(crate) operation: &'static str,
    pub(crate) from: &'a str,
    pub(crate) to: &'a str,
    pub(crate) from_index: usize,
    pub(crate) to_index: usize,
    pub(crate) provider_count: usize,
    /// The out-of-credit error that triggered the switch.
    pub(crate) error: &'a Error,
}

impl ProviderSwitch<'_> {
    fn error_code(&self) -> Option<&str> {
        match self.error.kind() {
            ErrorKind::Api {
                error_code: Some(ErrorCode::Anthropic(code)),
                ..
            } => Some(code),
            _ => None,
        }
    }

    /// This request moved the shared position: one `provider_switch` warning.
    pub(crate) fn switched(&self) {
        tracing::warn!(
            target: "carpa::client",
            operation = self.operation,
            from = self.from,
            to = self.to,
            from_index = self.from_index,
            to_index = self.to_index,
            provider_count = self.provider_count,
            reason = "out_of_credit",
            http.status_code = self.error.status(),
            error_code = self.error_code(),
            request_id = self.error.request_id(),
            "provider_switch"
        );
    }

    /// Another request had already moved the shared position past `from`.
    pub(crate) fn already_switched(&self) {
        tracing::debug!(
            target: "carpa::client",
            operation = self.operation,
            from = self.from,
            to = self.to,
            from_index = self.from_index,
            to_index = self.to_index,
            provider_count = self.provider_count,
            reason = "out_of_credit",
            http.status_code = self.error.status(),
            error_code = self.error_code(),
            request_id = self.error.request_id(),
            "provider_already_switched"
        );
    }
}

/// The last of `provider_count` providers was out of credit too, so `operation` failed.
pub(crate) fn providers_exhausted(
    operation: &'static str,
    provider: &str,
    provider_count: usize,
    status: Option<u16>,
) {
    tracing::error!(
        target: "carpa::client",
        operation,
        provider,
        provider_count,
        http.status_code = status,
        "providers_exhausted"
    );
}

/// One SSE event was parsed.
pub(crate) fn stream_event() {
    tracing::trace!(target: "carpa::stream", "event");
}

/// One chunk of `bytes` bytes arrived on a stream.
pub(crate) fn stream_bytes(bytes: u64) {
    tracing::trace!(target: "carpa::stream", bytes, "bytes");
}

/// The first byte of a stream arrived `time_to_first_byte` after the stream started.
pub(crate) fn stream_first_byte(time_to_first_byte: Duration) {
    tracing::debug!(
        target: "carpa::stream",
        ttfb_seconds = time_to_first_byte.as_secs_f64(),
        "first_byte"
    );
}

/// A stream reached end of input after `duration`.
pub(crate) fn stream_completed(duration: Duration) {
    tracing::debug!(
        target: "carpa::stream",
        duration_seconds = duration.as_secs_f64(),
        "completed"
    );
}

/// A stream failed.
pub(crate) fn stream_error() {
    tracing::debug!(target: "carpa::stream", "error");
}
