# Port Troubleshooting: bnauth on Windows

## Symptom

After completing Battle.net login, the browser stays on `us.account.battle.net`
showing a spinning wait circle. The OAuth callback never arrives.

## Root Cause: VS Code Remote SSH Port Forwarding

When connecting to a remote Linux host via VS Code's **Remote SSH** extension,
VS Code automatically forwards ports it detects in the remote terminal (including
Flask's startup port announcement). This means:

- VS Code listens on `127.0.0.1:<port>` on the **Windows** side and tunnels
  traffic to the remote host.
- If bnauth's configured port (`BNAUTH_FLASK_PORT`) matches a port already
  forwarded by VS Code, the local port is owned by VS Code's `Code.exe` process
  — not by bnauth running on the remote host.
- Battle.net's OAuth callback redirects to `http://localhost:<port>/callback`,
  which hits VS Code's forwarder rather than the Flask app. The connection hangs.

### Confirmed on cleo (April 2026)

```
netstat -ano | findstr :5050

  TCP    127.0.0.1:5050    0.0.0.0:0    LISTENING    52812
  ...

Get-Process -Id 52812 | Select-Object Id, ProcessName, Path

   Id  ProcessName  Path
52812  Code         C:\Users\...\Microsoft VS Code\Code.exe
```

Port 5050 was held by `Code.exe`, not bnauth.

## Solution

Use a port that VS Code has **not** auto-forwarded. If you're running bnauth
from a VS Code Remote SSH session, check the **Ports** tab (`Ctrl+Shift+P` →
*Forward a Port* → view the Ports panel) and pick a port not listed there.

### Steps

1. Choose a free port (e.g. `5051`).

2. Set it in `bnauth/.env`:
   ```env
   BNAUTH_FLASK_PORT=5051
   ```

3. Register the new callback URL in the
   [Blizzard Developer Portal](https://develop.battle.net/access/clients):
   ```
   http://localhost:5051/callback
   ```

4. Restart bnauth:
   ```sh
   uv run python -m bnauth.app
   ```

5. Access bnauth at `http://localhost:5051/` and authorize.

## Note: localhost vs 127.0.0.1

`localhost` works fine on Windows once the correct port is used. The
`localhost` → IPv6 issue (Windows resolving `localhost` to `::1`) was
**not** a factor here — it was a red herring investigated during debugging.
Both `http://localhost:5051/callback` and `http://127.0.0.1:5051/callback`
work correctly as redirect URIs.

## Preventing VS Code Auto-Forwarding

To stop VS Code from auto-forwarding ports when running bnauth from a Remote SSH
session, add to `battlenet-rs/.vscode/settings.json`:

```json
{
  "remote.autoForwardPorts": false
}
```

Alternatively, run bnauth from a terminal **outside** the VS Code Remote SSH
session (e.g., a standalone SSH session or Windows Terminal connecting directly
to `cleo`).

