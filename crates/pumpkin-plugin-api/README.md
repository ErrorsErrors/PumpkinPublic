# Pumpkin plugin API

The default SDK targets the stable v0.1 WIT API. Enable `api-v0-2` to target
v0.2 and use connection-scoped packet events:

```toml
pumpkin-plugin-api = { path = "../Pumpkin/crates/pumpkin-plugin-api", features = ["api-v0-2"] }
```

v0.2 packet events carry a `User`, connection state and protocol version before
and after a player joins. Set `replacement` for a typed replacement, or modify
`packet-id` and `raw-payload` for raw packets. Silent sends bypass packet events.
The v0.1 packet-event layout and the default SDK remain unchanged.

Typed Java packet views and replacements use Pumpkin's current protocol.
Other negotiated versions remain raw and require a translation plugin.
