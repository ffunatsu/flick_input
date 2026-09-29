# flick_input

[English](README.md) | [日本語](README_ja.md)

Use your smartphone's flick input as a wireless keyboard for your PC.

## Quick Start

### 1. Download & Run

Download the prebuilt binary for your OS from **[GitHub Releases](https://github.com/ffunatsu/flick_input/releases)**, then run it:

- **Windows**: Double-click `flick_input.exe`
- **macOS / Linux**: Run `./flick_input` in terminal

Or run directly from source:
```bash
cargo run --release
```

### 2. Connect from Smartphone

1. Make sure your PC and smartphone are connected to the same Wi-Fi.
2. Scan the QR code displayed in the terminal with your phone's camera (or open the printed URL in a browser).

### 3. Type

1. Tap the text box on your phone and type using flick input.
2. Press **Send** (or Enter). The confirmed text will be typed directly into your PC's active window.

---

## OS Requirements

- **Windows**: Ready to use out of the box.
- **macOS**: Grant "Accessibility" permission to the terminal running the app (**System Settings > Privacy & Security > Accessibility**).
- **Linux**: Requires X11 or Wayland with appropriate permissions.

---

## How It Works

1. `flick_input` runs a lightweight local web server on your PC.
2. The smartphone accesses the web UI without installing any app.
3. Confirmed text is sent asynchronously to the PC, which simulates keystrokes at the cursor position.

---

## Build from Source

```bash
git clone https://github.com/ffunatsu/flick_input.git
cd flick_input
cargo build --release
```

The binary will be created at `target/release/flick_input`.

## License

0BSD
