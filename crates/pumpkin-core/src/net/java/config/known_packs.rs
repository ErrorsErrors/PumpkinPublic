#[allow(clippy::wildcard_imports)]
use super::*;
use crate::net::java::login::known_packs::{synced_registries, synced_tags};

impl JavaClient {
    pub async fn handle_known_packs(&self, server: &Server) {
        for registry in synced_registries(server) {
            self.send_packet(&CRegistryData::new(
                &registry.registry_id,
                &registry.registry_entries,
            ))
            .await;
        }
        self.send_packet(&CUpdateTags::new(&synced_tags())).await;
        self.send_packet(&CFinishConfig).await;
    }
}
