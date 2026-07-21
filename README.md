<div align="center">

# 🛡️ TGK
### *The Greatest Knight*

**Your data is out there. TGK finds it. TGK kills it.**

*A local-first, zero-trust personal data exposure auditor and automated opt-out tool built in Rust.*

---

[![CI](https://github.com/arschaos/tgk/actions/workflows/rust.yml/badge.svg)](https://github.com/arschas/tgk/actions)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg?style=flat-square)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.85%2B-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20macOS%20%7C%20Windows-blue?style=flat-square)](#)

</div>

---

## 🚨 The Problem

Data breaches are no longer an edge case, they are an eventuality.

* Your phone number is indexed on **Whitepages**.
* Your residential history is exposed on **Spokeo**.
* Your full name, age, and relative mapping are cataloged on **BeenVerified**.
* Your credentials from a breach five years ago remain accessible in public aggregators.

Commercial removal services like Incogni, DeleteMe, and Aura promise to clean this up. However, their model requires you to **hand over your full legal name, home address, phone numbers, date of birth, and a monthly subscription** to yet another third-party corporation.

> **To remove your data from data brokers, you shouldn't have to become customer data for a new one.**

---

## 🛡️ The TGK Solution

**TGK (The Greatest Knight)** approaches data privacy from a zero-trust, local-first operational stance:

* **Zero Cloud Dependencies:** Your identity profile, search queries, and credentials never leave your local machine.
* **Local Audit Pipeline:** TGK queries, scrapes, and aggregates public endpoints and data broker indexes directly from your machine.
* **Automated Removal Routing:** Generates and executes opt-out protocols directly between you and the broker.
* **Zero Telemetry:** No analytics, no phone-home mechanisms, no tracking.

---

## ✨ Key Features

* **🔍 Localized Footprint Mapping:** Scan major data brokers and breach registries directly from your CLI.
* **⚡ High-Performance Rust Engine:** Zero-dependency, memory-safe execution with minimal compute overhead.
* **🔒 Encrypted Local Vault:** All local configuration and target states are stored using local encrypted storage.
* **📄 Automated Opt-Out Dispatch:** Generate opt-out requests, track removal lifecycles, and audit status changes over time.

---

## 🚀 Quick Start

TGK compiles to a single, zero-dependency static binary. You **do not** need Rust or build tools installed to run TGK.

### Option 1: Standalone Binary (Recommended)

Download the pre-compiled binary for your architecture from the [Releases Page](https://github.com/arschaos/tgk/releases/latest).

#### Linux (x86_64 / ARM64)
```bash
# Download and make executable
curl -sSL https://github.com/arschaos/tgk/releases/latest/download/tgk-linux-amd64 -o tgk
chmod +x tgk

# Move to path (optional)
sudo mv tgk /usr/local/bin/

# Run initial configuration
tgk init
```

#### macOS (Apple Silicon / Intel)
```bash
curl -sSL https://github.com/arschaos/tgk/releases/latest/download/tgk-macos-universal -o tgk
chmod +x tgk
./tgk init
```

#### Windows (PowerShell)
```bash
Invoke-WebRequest -Uri "https://github.com/arschaos/tgk/releases/latest/download/tgk-windows-amd64.exe" -OutFile "tgk.exe"
.\tgk.exe init
```

### Option 2: Docker / Container Runtime
If you prefer isolated execution without installing local binaries:
```bash
# Pull and run directly
docker run --rm -it -v ~/.config/tgk:/root/.config/tgk ghcr.io/arschaos/tgk:latest init
```

### Option 3: Building From Source
Requires the Rust toolchain (`1.85.0+`):
```bash
# Clone the repository
git clone https://github.com/arschaos/tgk.git
cd tgk

# Build native release binary
cargo build --release

# Run
./target/release/tgk --help
```