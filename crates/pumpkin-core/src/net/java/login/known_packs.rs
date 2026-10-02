#[allow(clippy::wildcard_imports)]
use super::*;

impl PendingConnection {
    pub async fn handle_known_packs(&mut self, server: &Server) {
        for registry in synced_registries(server) {
            self.send_packet_now(&CRegistryData::new(
                &registry.registry_id,
                &registry.registry_entries,
            ))
            .await;
        }
        let tags = synced_tags();
        self.send_packet_now(&CUpdateTags::new(&tags)).await;
        self.send_packet_now(&CFinishConfig).await;
    }
}

pub(crate) fn synced_registries(server: &Server) -> Vec<pumpkin_data::registry::Registry> {
    pumpkin_data::registry::Registry::get_synced(CURRENT_MC_VERSION)
        .into_iter()
        .map(|mut registry| {
            registry.registry_entries = if registry.registry_id == "minecraft:damage_type" {
                server
                    .datapack_manager
                    .merge_damage_type_entries(&registry.registry_entries)
            } else {
                server
                    .datapack_manager
                    .merge_registry_entries(&registry.registry_id, &registry.registry_entries)
            };
            registry
        })
        .collect()
}

pub(crate) fn synced_tags() -> Vec<pumpkin_data::tag::RegistryKey> {
    pumpkin_data::tag::RegistryKey::NETWORK_KEYS
        .iter()
        .copied()
        .filter(|&key| {
            pumpkin_data::tag::get_registry_key_tags(CURRENT_MC_VERSION, key)
                .is_some_and(|tags| !tags.is_empty())
        })
        .collect()
}
