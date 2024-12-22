use crate::nns_relay;
use nostr_sdk::{Filter, FromBech32, Kind, PublicKey};
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

type NostrClient = Arc<Mutex<nostr_sdk::Client>>;

pub async fn retrieve_ip_address(npub: &str, nostr_client: &NostrClient) -> Option<String> {
    let public_key = PublicKey::from_bech32(npub).ok()?;

    let filter = Filter::new()
        .author(public_key)
        .kind(Kind::TextNote)
        .limit(1);

    let events = {
        let _client = nostr_client.lock().await;

        _client
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
