use wasmtime::component::Resource;

use super::{
    pumpkin::plugin::{player, user as wit, uuid::Uuid},
    uuid::UuidExt,
};
use pumpkin_core::net::user::Edition;
use pumpkin_wasm_host_common::state::PluginHostState;

#[must_use]
pub const fn to_wasm_state(state: pumpkin_protocol::ConnectionState) -> wit::ConnectionState {
    use pumpkin_protocol::ConnectionState as C;
    match state {
        C::HandShake => wit::ConnectionState::Handshake,
        C::Status => wit::ConnectionState::Status,
        C::Login => wit::ConnectionState::Login,
        C::Transfer => wit::ConnectionState::Transfer,
        C::Config => wit::ConnectionState::Config,
        C::Play => wit::ConnectionState::Play,
    }
}

impl wit::Host for PluginHostState {}

impl wit::HostUser for PluginHostState {
    async fn get_info(&mut self, resource: Resource<wit::User>) -> wasmtime::Result<wit::UserInfo> {
        let user = self.get(&resource)?.clone();
        let info = user.info();
        Ok(wit::UserInfo {
            id: Uuid::to_wit(&user.id),
            edition: match user.edition {
                Edition::Java => wit::Edition::Java,
                Edition::Bedrock => wit::Edition::Bedrock,
            },
            decoder_state: to_wasm_state(user.decoder_state.load()),
            encoder_state: to_wasm_state(user.encoder_state.load()),
            address: info.address.to_string(),
            server_address: info.server_address,
            server_port: info.server_port,
            protocol_version: user.protocol_version.load(),
            name: info.profile.as_ref().map(|profile| profile.name.clone()),
            uuid: info
                .profile
                .as_ref()
                .map(|profile| Uuid::to_wit(&profile.id)),
            properties: info
                .profile
                .as_ref()
                .map(|profile| {
                    profile
                        .properties
                        .load()
                        .iter()
                        .map(|property| wit::ProfileProperty {
                            name: property.name.to_string(),
                            value: property.value.to_string(),
                            signature: property.signature.as_ref().map(ToString::to_string),
                        })
                        .collect()
                })
                .unwrap_or_default(),
            settings: info.config.as_ref().map(super::player::to_wasm_settings),
            brand: info.brand,
            closed: user.is_closed(),
        })
    }

    async fn get_player(
        &mut self,
        resource: Resource<wit::User>,
    ) -> wasmtime::Result<Option<Resource<player::Player>>> {
        self.get(&resource)?
            .clone()
            .player()
            .map(|player| self.add(player))
            .transpose()
    }

    async fn send_packet(
        &mut self,
        resource: Resource<wit::User>,
        packet_id: i32,
        payload: Vec<u8>,
        silent: bool,
    ) -> wasmtime::Result<Result<(), String>> {
        Ok(self
            .get(&resource)?
            .send_packet(packet_id, &payload, silent)
            .map_err(|error| error.to_string()))
    }

    async fn close(&mut self, resource: Resource<wit::User>) -> wasmtime::Result<()> {
        self.get(&resource)?.clone().close();
        Ok(())
    }

    async fn clone(
        &mut self,
        resource: Resource<wit::User>,
    ) -> wasmtime::Result<Resource<wit::User>> {
        self.add(self.get(&resource)?.clone())
    }

    async fn drop(&mut self, resource: Resource<wit::User>) -> wasmtime::Result<()> {
        self.drop(resource)
    }
}
