#[allow(clippy::wildcard_imports)]
use super::*;

impl JavaClient {
    pub async fn handle_config_acknowledged(&self, server: &Server) -> PacketHandlerResult {
        debug!("Handling config acknowledgement");
        self.connection_state.store(ConnectionState::Play);

        let profile = self.gameprofile.clone();
        let address = self.address;

        if let Some(reason) = can_not_join(&profile, &address, server).await {
            self.kick(reason).await;
            return PacketHandlerResult::Stop;
        }

        let config = self.config.load();
        PacketHandlerResult::ReadyToPlay(profile, (**config).clone())
    }
}
