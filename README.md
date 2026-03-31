# Enoact Core

Discord Rich Presence client that can display anything if WebSocket client is implemented.  
This give an ability to any web app to connect via WebSocket and send presence packet.  

It can display up to 4 different activities with different app name (if you need more please open an issue)

<img width="475" height="348" alt="image" src="https://github.com/user-attachments/assets/9159f271-2c4c-4e18-ae41-62b6d3d02cfd" />

## How to build

1. Clone this repo
   
> [!IMPORTANT]  
> Web UI is currently required to build final executable

### Build Web UI

2. Install `node`, `bun` or alternatives
3. Install web UI dependencies
```nushell
bun install
```
4. Build using command below
```nushell
bun run build
```

### Build the executable

5. Run the following

```nushell
cargo b -r
```

This will produce 2 binaries: `enoact` and `enoact-presence`.  
You can ignore `enoact-presence` entirely as it is just an internal tool for `enoact` so you only need to run `enoact`.  
`enoact` rely on `enoact-presence` to display an activity so you will have to place them together or add `enoact-presence` to `PATH`.
