# macOS Educational Kiosk Browser (Tauri v2)

An ironclad, inescapable fullscreen kiosk browser built using **Rust** and **Tauri v2** designed to keep children focused strictly on their educational platform (**Foxford.ru**).

This application completely locks down the macOS user environment, suppressing native desktop controls, system navigation menus, and shortcut escapes to create a completely isolated workstation environment.

## 🚀 Features

* **Absolute Edge-to-Edge Fullscreen:** Bypasses native macOS Spaces to draw a borderless canvas directly over the hardware coordinates.
* **OS Interface Suppression:** Dynamically hides the macOS Status Bar (Menu Bar) and the bottom Dock framework. Moving the cursor to the margins will not trigger them.
* **System Shortcuts Blocked:** Disables system-level shortcuts including `Cmd + Q` (Quit) and `Cmd + Tab` (App Switching).
* **Escape Loophole Neutralized:** Blocks the native `Esc` key behavior from minimizing or collapsing the fullscreen presentation layer.
* **Zero Dock Footprint:** Registers as a native background accessory panel to prevent icon spawns or window grouping in the task manager.

---

## 🛠️ Tech Stack

* **Backend:** Rust, Tauri v2
* **OS Interop Layer:** `objc2`, `objc2-app-kit` (Natively targeting AppKit / Cocoa framework configurations)
* **Frontend:** Built-in Webkit View engine (`WKWebView`) pointing directly to `https://foxford.ru`

---

## ⚠️ Important: The Emergency Backdoor

Because the application disables all traditional methods of window closure and system navigation, a hardcoded global hotkey sequence has been implemented so you can cleanly exit the kiosk.

Press **`Command + Option + Shift + P`** simultaneously to instantly close the application.

---

## 🔧 Prerequisites & Setup

Ensure you have the Rust toolchain and Tauri v2 prerequisites installed on your machine.

### 1. System Dependencies
Add the native target configuration constraints to your `src-tauri/Cargo.toml`:

```toml
[target.'cfg(target_os = "macos")'.dependencies]
objc2 = "0.5"
objc2-app-kit = { version = "0.2", features = ["NSWindow", "NSApplication", "NSWindowTabGroup"] }
tauri-plugin-global-shortcut = "2.0.0"
```

### 2. Capabilities Profile
Ensure permissions for the hotkey registry are explicitly allowed for production builds in `src-tauri/capabilities/default.json`:

```json
"permissions": [
  "core:default",
  "global-shortcut:allow-is-registered",
  "global-shortcut:allow-register",
  "global-shortcut:allow-unregister"
]
```

---

## 💻 Development & Building

### Run in Development Mode
```bash
cargo tauri dev
```

### Compile Production Release Bundle
To compile a standalone, optimized macOS `.app` bundle:
```bash
cargo tauri build
```
The resulting package will be outputted to:
`src-tauri/target/release/bundle/macos/YOUR_APP_NAME.app`

---

## 📝 Gatekeeper Installation Note

Because you are compiling this binary locally without an active Apple Developer Certificate registry subscription:
1. Drag the compiled `.app` package into your `/Applications` directory.
2. **Right-click (or Ctrl+Click)** the application and select **Open**.
3. Click **Open Anyway** in the macOS security verification prompt to grant permissions permanently.

## 📄 License
This project is open-source and available under the [MIT License](LICENSE).
