# flick_input

Send text from mobile flick input directly to PC via local web server.

## Overview

`flick_input` runs a lightweight HTTP/WebSocket server on PC. A mobile device connected to the same local network accesses the web interface, takes text input (using the mobile OS flick keyboard), and sends confirmed text to the PC. The PC injects received text as simulated keystrokes into the active window.

## Architecture

- **Backend (Rust)**:
  - Web server (Axum) serving a mobile-friendly web page.
  - HTTP endpoint receiving confirmed text payloads.
  - Text input injection into the active window on PC.
  - QR code generation in terminal for quick connection.
- **Frontend (HTML/JS)**:
  - Mobile web interface optimized for seamless flick typing without page reloads.
  - Submits confirmed text asynchronously (Fetch API / WebSocket) on Enter or Send button.
  - Automatically resets input field while retaining keyboard focus for continuous typing.

## Prerequisites

- Rust (cargo)
- PC and mobile device connected to the same Wi-Fi / local network.
- **macOS**: Accessibility permission must be granted to the terminal application running this program.
- **Linux**: X11 or Wayland with appropriate permissions.

## Installation & Running

```bash
cargo run --release
```

1. Run the application on PC.
2. Scan the QR code displayed in the terminal using your smartphone camera (or enter the URL manually).
3. Tap the input field on your smartphone and type using flick input.
4. Confirm input (Enter or Send button). The text will appear at the cursor on your PC.

## Configuration

Default settings can be adjusted via CLI options or environment variables:

- `--port` / `PORT`: Listening port (default: `8080`).
- `--host` / `HOST`: Bind address (default: `0.0.0.0`).

## License

MIT
