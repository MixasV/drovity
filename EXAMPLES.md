## Example 1: Basic Usage

After installing drovity, here's a complete workflow:

### Step 1: Install
```bash
curl -fsSL https://raw.githubusercontent.com/MixasV/drovity/main/install.sh | bash
```

### Step 2: Add Google Account
```bash
$ drovity
# Menu appears:
# 1. Accounts
# Select: 1

# Select "1. Add New Account"
# You'll see a URL like:
# https://accounts.google.com/o/oauth2/v2/auth?client_id=...

# Copy and paste in browser, authorize with Google
# Google shows a code, copy it back to terminal
# [SUCCESS] Account added successfully!
# Email: your.email@gmail.com
```

### Step 3: Start Proxy
```bash
# From menu: 2. API Proxy → 1. Start Proxy
# Or run directly:
$ drovity hide
[SUCCESS] Proxy started in background (PID: 12345)
   Port: 8045
   Use 'drovity stop' to stop the server
```

### Step 4: Configure Factory Droid
```bash
$ drovity
# Menu: 3. Droid Settings Setup → 1. Auto Config
[SUCCESS] Factory Droid settings updated successfully!

You can now use drovity models in Factory Droid:
  - Type /model in Factory Droid CLI
  - Select a model starting with [drovity]
```

### Step 5: Use with Factory Droid
```bash
$ droid
> /model
# Select: "Gemini 2.5 Flash [drovity]"
> What is the capital of France?
# Paris is the capital of France...
```

---

## Example 2: Using Claude Models

Claude models work through Google Gemini API (no Anthropic account needed):

```bash
$ droid
> /model
# Select: "Claude 4.5 Sonnet [drovity]"
> Write a Python function to reverse a string
# def reverse_string(text):
#     return text[::-1]
```

All Claude models are automatically mapped to Gemini API by drovity.

---

## Example 3: Managing Multiple Accounts

```bash
$ drovity
# 1. Accounts

# Current Accounts:
#   1. user1@gmail.com - [ACTIVE]
#   2. user2@gmail.com - [ACTIVE]

# Add more accounts:
# 1. Add New Account (follow OAuth flow)

# Remove account:
# 2. Remove Account → Select account → Removed
```

Drovity automatically rotates between accounts when rate limits are hit.

---

## Example 4: Daemon Mode

Run proxy in background and use your terminal freely:

```bash
# Start in background
$ drovity hide
[SUCCESS] Proxy started in background (PID: 67890)

# Check status
$ drovity status
Status: Running
PID: 67890
Port: 8045
Address: http://127.0.0.1:8045

# Stop when done
$ drovity stop
[SUCCESS] Proxy stopped (PID: 67890)
```

---

## Example 5: Manual Factory Droid Configuration

If auto-config doesn't work, configure manually:

```bash
$ drovity
# 3. Droid Settings Setup → 2. Manual Setup

# Copy the JSON shown (example):
```

```json
{
  "customModels": [
    {
      "model": "gemini-2.5-flash",
      "id": "gemini-2-5-flash-drovity",
      "baseUrl": "http://127.0.0.1:8045/",
      "apiKey": "sk-abc123...",
      "displayName": "Gemini 2.5 Flash [drovity]",
      "provider": "anthropic"
    }
  ]
}
```

Add to `~/.factory/settings.json` and restart Factory Droid.

---

## Example 6: Troubleshooting

### Proxy won't start (port in use)
```bash
$ drovity start
Error: Address already in use

# Check what's using port 8045
$ lsof -i :8045
COMMAND   PID USER
drovity 12345 user

# Stop old instance
$ drovity stop
[SUCCESS] Proxy stopped

# Start fresh
$ drovity start
```

### Factory Droid shows "400 Bad Request"
```bash
# Check proxy is running
$ drovity status
Status: Stopped  # <-- Issue found!

# Start proxy
$ drovity start

# Try Factory Droid again
$ droid
> /model
# Now works!
```

### Account authorization expired
```bash
$ drovity
# 1. Accounts
# See account marked [DISABLED]

# Solution: Re-add account
# 1. Remove old account
# 2. Add New Account → Follow OAuth flow
# Account refreshed!
```

---

## Tips & Tricks

### Use alias for quick access
```bash
# Add to ~/.bashrc or ~/.zshrc
alias drv='drovity'
alias drvh='drovity hide'
alias drvs='drovity status'

# Then:
$ drv      # Opens menu
$ drvh     # Starts in background
$ drvs     # Checks status
```

### Auto-start on system boot (Linux/macOS)

Create systemd service (Linux):
```bash
sudo nano /etc/systemd/system/drovity.service
```

```ini
[Unit]
Description=Drovity API Proxy
After=network.target

[Service]
Type=simple
User=your-username
ExecStart=/usr/local/bin/drovity start
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

```bash
sudo systemctl enable drovity
sudo systemctl start drovity
```

### Check logs (when running in foreground)
```bash
$ drovity start
Proxy server started on http://0.0.0.0:8045
Loaded 2 account(s)
# Logs appear here...
```

---

See [README.md](README.md) for full documentation.
