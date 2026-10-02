#[allow(clippy::wildcard_imports)]
use super::*;

impl JavaClient {
    pub async fn handle_resource_pack_response(
        &self,
        server: &Server,
        packet: SConfigResourcePack,
    ) {
        let resource_config = &server.advanced_config.resource_pack.java;
        if resource_config.enabled {
            use pumpkin_protocol::java::server::config::ResourcePackResponseResult;
            match packet.response_result() {
                ResourcePackResponseResult::Downloaded
                | ResourcePackResponseResult::DownloadSuccess
                | ResourcePackResponseResult::Discarded
                | ResourcePackResponseResult::Unknown(_) => {
                    self.send_known_packs(server).await;
                }
                ResourcePackResponseResult::Accepted => {}
                ResourcePackResponseResult::Declined => {
                    if resource_config.force {
                        self.kick(TextComponent::text("Required resource pack was declined"))
                            .await;
                    } else {
                        self.send_known_packs(server).await;
                    }
                }
                ResourcePackResponseResult::DownloadFail => {
                    if resource_config.force {
                        self.kick(TextComponent::text("Failed to download resource pack"))
                            .await;
                    } else {
                        self.send_known_packs(server).await;
                    }
                }
                ResourcePackResponseResult::InvalidUrl => {
                    self.kick(TextComponent::text("Invalid resource pack URL"))
                        .await;
                }
                ResourcePackResponseResult::ReloadFailed => {
                    self.kick(TextComponent::text("Failed to reload resource pack"))
                        .await;
                }
            }
        }
    }

    pub async fn send_known_packs(&self, server: &Server) {
        let features = server.get_enabled_features();
        self.send_packet(&CFeatureFlags::new(&features)).await;
        let version_str = self.version.load().to_string();
        let loaded_packs = server.datapack_manager.get_loaded_packs();
        let known_packs = server.get_known_packs(&version_str, &loaded_packs);
        self.send_packet(&CKnownPacks::new(&known_packs)).await;
    }
}
