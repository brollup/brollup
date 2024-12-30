use crate::{
    key::KeyHolder,
    nns_relay::{self, Relay},
};
use nostr_sdk::{EventBuilder, Filter, FromBech32, Kind, PublicKey};
use std::time::Duration;

#[derive(Clone)]
pub struct Client {
    nostr_client: nostr_sdk::Client,
}

impl Client {
    pub async fn new(keys: &KeyHolder) -> Self {
        let nostr_client = nostr_sdk::Client::new(keys.nostr_key_pair());
        {
            nostr_client.add_default_relay_list().await;
            nostr_client.connect().await;
        }
        Client { nostr_client }
    }

    pub async fn query_address(&self, npub: &str) -> Option<String> {
        let public_key = PublicKey::from_bech32(npub).ok()?;

        let filter = Filter::new()
            .author(public_key)
            .kind(Kind::TextNote)
            .limit(1);

        let events = {
            self.nostr_client
                .fetch_events_from(
                    nns_relay::DEFAULT_RELAY_LIST,
                    vec![filter],
                    Some(Duration::from_secs(5)),
                )
                .await
        };

        let last_event = match &events {
            Ok(events) => match events.first() {
                Some(event) => event,
                None => return None,
            },
            Err(_) => return None,
        };

        Some(last_event.content.clone())
    }

    pub async fn publish_address(&self, ip_address: &str) -> Option<[u8; 32]> {
        let note_publish_event = EventBuilder::text_note(ip_address);

        match self
            .nostr_client
            .send_event_builder(note_publish_event)
            .await
        {
            Ok(ok) => {
                return Some(ok.as_bytes().to_owned());
            }
            Err(_) => return None,
        };
    }
}
