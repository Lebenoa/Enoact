# Enoact Core

Discord Rich Presence client that can display anything if WebSocket client is implemented.  
This give an ability to any web app to connect via WebSocket and send presence packet.  

It can display up to 4 different activities with different app name (if you need more please open an issue)

<img width="475" height="348" alt="image" src="https://github.com/user-attachments/assets/9159f271-2c4c-4e18-ae41-62b6d3d02cfd" />

## Consideration

- Set/Clear via HTTP request
  > This was implemented but later removed as it is unreliable to clear an activity

## How to build

> How you build Rust normally

1. Clone this repo
2. Run the following

```nushell
cargo b -r
```

This will produce 2 binaries: `enoact` and `enoact-presence`.  
You can ignore `enoact-presence` entirely as it is just an internal tool for `enoact` so you only need to run `enoact`.  
`enoact` rely on `enoact-presence` to display an activity so you will have to place them together or add `enoact-presence` to `PATH`.
