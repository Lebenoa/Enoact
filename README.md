# Enoact Core

Discord Rich Presence client that can display anything if WebSocket client is implemented.  
This give an ability to any web app to connect via WebSocket and send presence packet.  

It can display up to 4 different activities with different app name (if you need more please open an issue)

<img width="475" height="348" alt="image" src="https://github.com/user-attachments/assets/9159f271-2c4c-4e18-ae41-62b6d3d02cfd" />

## API for developer(s)

- [`Activity`](https://github.com/Lebenoa/Enoact/blob/master/web-ui/src/lib/types/index.ts#L1-L57) structure

<details>
   <summary>For WebSocket (Prefer)</summary>
   
   1. Connect to `http://127.0.0.1:5579/ws`
   2. Send packet like below
      
   ```javascript
   {
      "action": "SET" | "CLEAR",
      "activity": { ... },
   }
   ```

   [Example](https://github.com/Lebenoa/Enoact-bext/blob/master/src/shared/youtube.js#L39-L62)
</details>

<details>
   <summary>For HTTP Endpoint</summary>

   - Set presence: `POST` to `/api/set-presence` | if `app_id` is null, it'll spawn a presence and `app_id` will be return
     ```javascript
     {
        "activity": { ... },
        "app_id": null
     }
     ```
     [Example](https://github.com/Lebenoa/Enoact/blob/master/web-ui/src/routes/Home.svelte#L41-L59)

   - Clear presence: `POST` to `/api/clear-presence` | required `app_id` to be the body
     ```javascript
     1246549832349
     ```
</details>

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
