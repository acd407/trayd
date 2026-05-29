# trayd

Minimal Wayland-session system tray daemon (`zbus`) with a documented IPC socket for bars and other clients.

## Workspace

| Crate      | Role                                                           |
| ---------- | -------------------------------------------------------------- |
| `libtrayd` | D-Bus SNI host + DBusMenu client (library, used by trayd only) |
| `trayd`    | Persistent daemon — D-Bus host + Unix-socket IPC server        |
| `trayctl`  | One-shot menu orchestrator (IPC client + dmenu bridge)         |
| `tray-tui` | Terminal UI client (IPC socket only, Phase 4)                  |

See [`docs/PLAN.md`](docs/PLAN.md) for the architecture and [`docs/IPC.md`](docs/IPC.md) for the wire protocol.

---

## Build

```sh
cargo build --workspace
```

---

## Running

### 1. Start the daemon

```sh
trayd
# or explicitly:
trayd run
```

By default the daemon listens on `$XDG_RUNTIME_DIR/trayd.sock`.
To use a custom socket or log level, copy `examples/trayd.toml` to
`$XDG_CONFIG_HOME/trayd/trayd.toml` and edit it.

Health-check while the daemon is running:

```sh
trayd ping
```

### 2. List registered tray items

```sh
trayctl items
```

Output is a JSON array of `MinimalTrayItem` objects:

```json
[
  {
    "app_id": "org.freedesktop.NetworkManager.applet",
    "title": "Network",
    "status": "Active",
    "icon_handle": "nm-device-wireless"
  }
]
```

### 3. Open a tray menu with tofi

```sh
trayctl menu --app-id <app_id>
```

`trayctl` defaults to `tofi --mode dmenu` as the picker.  
To use a different dmenu-compatible tool (rofi, fuzzel, bemenu, …):

```sh
trayctl menu --app-id org.freedesktop.NetworkManager.applet \
             --dmenu-cmd "rofi -dmenu"

trayctl menu --app-id org.freedesktop.NetworkManager.applet \
             --dmenu-cmd "fuzzel --dmenu"
```

The submenu loop works like this:

```
trayctl                 trayd (IPC)             dmenu tool
  │                        │                       │
  │── get_menu(app_id) ──►│                        │
  │◄── [item list] ────────│                        │
  │── labels ─────────────────────────────────────►│
  │◄── selected label ─────────────────────────────│
  │                        │                        │
  │  (is submenu?)         │                        │
  │── get_menu(submenu_id)►│                        │
  │◄── [child items] ──────│                        │
  │── labels ─────────────────────────────────────►│
  │◄── selected label ─────────────────────────────│
  │                        │                        │
  │── activate(item_id) ──►│                        │
```

Press Esc or leave the picker empty at any level to cancel without activating.

### 4. Override the socket path

All tools accept `--socket`:

```sh
trayd --socket /tmp/my.sock
trayctl --socket /tmp/my.sock items
trayctl --socket /tmp/my.sock menu --app-id org.example.App
```

---

## Testing

### Unit tests (no daemon, no D-Bus required)

```sh
cargo test --workspace
```

This runs ~60 tests across all crates: IPC codec round-trips, golden JSON
fixtures, menu node parsing helpers, dmenu label-matching logic, and CLI
argument parsing.

### Integration tests against a live daemon

These tests are marked `#[ignore]` and require a running D-Bus session bus
with at least one SNI-registered tray application (e.g. NetworkManager,
Blueman, or any app that puts an icon in the system tray).

```sh
# Run all ignored (live) tests:
cargo test --workspace -- --ignored

# Or run a specific package:
cargo test --package libtrayd -- --ignored
cargo test --package trayd    -- --ignored
```

### Manual end-to-end walkthrough

```sh
# Terminal 1 — start the daemon
RUST_LOG=debug trayd

# Terminal 2 — list tray items
trayctl items

# Terminal 2 — open a menu (requires tofi, rofi, fuzzel, or similar)
APP=$(trayctl items | jq -r '.[0].app_id')
trayctl menu --app-id "$APP"

# Or with rofi:
trayctl menu --app-id "$APP" --dmenu-cmd "rofi -dmenu"
```

### Testing IPC directly with socat / nc

Use the golden fixtures in `examples/ipc-examples/` to send raw requests:

```sh
# Ping
echo '{"v":1,"cmd":"ping"}' | socat - UNIX-CONNECT:"$XDG_RUNTIME_DIR/trayd.sock"

# List items
echo '{"v":1,"cmd":"get_items"}' | socat - UNIX-CONNECT:"$XDG_RUNTIME_DIR/trayd.sock"

# Get a menu (replace app_id with a real one from get_items)
echo '{"v":1,"cmd":"get_menu","app_id":"org.example.App","submenu_id":null}' \
  | socat - UNIX-CONNECT:"$XDG_RUNTIME_DIR/trayd.sock"

# Activate a menu item (replace ids with real values)
echo '{"v":1,"cmd":"activate","app_id":"org.example.App","item_id":1}' \
  | socat - UNIX-CONNECT:"$XDG_RUNTIME_DIR/trayd.sock"
```

The `.jsonl` files in `examples/ipc-examples/` each contain one request line
followed by one expected response line.
