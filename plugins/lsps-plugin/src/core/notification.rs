//! LSP-wide notifications published on CLN's custom-notification bus.
//!
//! `LspNotification` groups everything this LSP publishes externally. Each
//! variant maps to one CLN topic; as the plugin grows beyond lsps2 (e.g.
//! lsps1), add a new variant + topic here, then register the topic with the
//! plugin `Builder` in `main()`.

use crate::core::lsps2::event_sink::SessionEventEnvelope;
use serde::Serialize;

/// Every lsps2 `SessionEvent` is republished here unfiltered, wrapped in its
/// envelope (scid + payment_hash + event). Consumers that only care about a
/// subset of events filter client-side on the `event` field.
pub const LSPS2_SESSION_EVENT_TOPIC: &str = "lsps2-session-event";

/// A notification published on the LSP's custom-notification bus.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum LspNotification {
    Lsps2SessionEvent(SessionEventEnvelope),
}

impl LspNotification {
    /// The CLN topic this notification is published under.
    pub fn topic(&self) -> &'static str {
        match self {
            LspNotification::Lsps2SessionEvent(_) => LSPS2_SESSION_EVENT_TOPIC,
        }
    }
}

impl From<SessionEventEnvelope> for LspNotification {
    fn from(envelope: SessionEventEnvelope) -> Self {
        Self::Lsps2SessionEvent(envelope)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::lsps2::session::SessionEvent;
    use crate::proto::lsps0::ShortChannelId;
    use bitcoin::hashes::Hash;
    use bitcoin::hashes::sha256::Hash as PaymentHash;

    fn envelope(event: SessionEvent) -> SessionEventEnvelope {
        SessionEventEnvelope {
            scid: ShortChannelId::from(100u64 << 40 | 1u64 << 16),
            payment_hash: PaymentHash::from_byte_array([1; 32]),
            event,
        }
    }

    #[test]
    fn session_event_maps_to_lsps2_session_event_topic() {
        let notification: LspNotification = envelope(SessionEvent::FundingChannel).into();
        assert_eq!(notification.topic(), LSPS2_SESSION_EVENT_TOPIC);
    }

    #[test]
    fn notification_serializes_envelope_fields_at_top_level() {
        let notification: LspNotification = envelope(SessionEvent::FundingChannel).into();

        let value = serde_json::to_value(&notification).unwrap();
        assert!(value.get("scid").is_some());
        assert!(value.get("payment_hash").is_some());
        assert!(value.get("event").is_some());
    }
}
