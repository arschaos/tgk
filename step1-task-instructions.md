# AI Task Instructions — Step 1: Profile Creator via CLI

> Part of a 3-step plan for the TGK encrypted profile maker:
> **1. Create a profile of a user through the CLI, return it to the user** ← this task
> 2. Persist the profile to a database
> 3. Encrypt the database (vault: Argon2id → AES-256-GCM)
>
> Output of this task: a clean, committed, CI-green CLI workflow for creating and displaying a user profile — no storage yet.

---

## 1. Role Definition

Act as a **senior Rust systems engineer** with deep experience in:
- Rust workspace architecture (edition 2024, `cargo` workspaces, `clap` derive API)
- CLI UX design (consistent subcommand trees, `--help` quality, human + machine-readable output)
- Defensive coding under strict lints (`clippy -D warnings`, `unsafe_code = "forbid"` culture)
- Privacy-sensitive data handling (minimizing what identity data a tool stores and echoes)

You are modifying an existing, working repository. You do **not** redesign the project; you extend it minimally and idiomatically.

---

## 2. Objective

Add a `profile` subcommand tree to the `tgk` CLI so the user can:

1. **Create** a profile in-memory (identity fields + optional registered accounts) from CLI flags or an interactive prompt.
2. **Return** the profile to the user in a human-readable summary and an optional machine-readable JSON form.

No persistence, no encryption, no network. This is the in-memory model + CLI layer only; it must be built so Steps 2 (SQLite) and 3 (encryption) can reuse it without changes.

---

## 3. Context

### Repository state (verified on disk)
- Cargo **virtual workspace** at repo root; member: `crates/tgk-cli` (binary `tgk`).
- `workspace.package`: version `0.0.1`, **edition 2024**, `rust-version = "1.85"`, license `GPL-3.0-only`.
- `workspace.dependencies` currently: `clap = { version = "4", features = ["derive"] }`.
- `crates/tgk-cli/src/main.rs`: clap `Parser` with a single `Execute` subcommand printing "Hello World"; existing tests: `cli_definition_is_valid`, `execute_is_parsed`, `unknown_command_is_rejected`.
- **CI (GitHub Actions)** runs on every push/PR across ubuntu/windows/macos: `cargo fmt --all --check`, `cargo clippy --workspace --release -- -D warnings`, `cargo test --workspace --release`, `cargo build`, **and a smoke step: `cargo run --release -- execute`** — this must keep working (or be updated in the same commit).
- Docker CI runs the built image `execute` and `--version` — unaffected unless the CLI surface breaks.
- Branch convention: work on a feature branch off `main`, small commits, PR to `main`.

### Design intent (from project plan)
- Profile logic belongs in a **library crate `crates/tgk-core`**, not in `main.rs` — the future `tgk serve` API daemon and dashboard will link against the same core.
- Profile schema (agreed): **identity** (display name, email(s), username(s)) + **registered accounts** (service, login identifier, is_broker flag, removal method, status, last run — last three nullable in Step 1).

---

## 4. Step-by-Step Execution

Follow these steps in order. Each is atomic; verify before proceeding.

### Step 4.1 — Create the core crate
1. `mkdir -p crates/tgk-core/src`.
2. `crates/tgk-core/Cargo.toml`:
   - `[package]` with `name = "tgk-core"`, inheriting `version`, `edition`, `rust-version`, `license`, `repository` from workspace (`workspace = true`).
   - `[dependencies]`: `serde = { version = "1", features = ["derive"] }`, `serde_json = "1"` — **also add both to root `[workspace.dependencies]`** and reference them as `workspace = true` (keep single source of truth).
3. Register the crate: root `Cargo.toml` already uses `members = ["crates/*"]`, so no edit needed — confirm.
4. `crates/tgk-core/src/lib.rs` defining:

```rust
pub struct Profile {
    pub display_name: String,
    pub emails: Vec<String>,
    pub usernames: Vec<String>,
    pub accounts: Vec<Account>,
}

pub struct Account {
    pub service: String,
    pub login: String,          // email or username used on that service
    pub is_broker: bool,        // data broker vs. regular service
    // Fields below: present in schema, Optional for Step 1
    pub removal_method: Option<String>,  // e.g. "api" | "email" | "web_form" | "manual"
    pub status: Option<String>,          // e.g. "none" | "requested" | "confirmed"
    pub last_run: Option<String>,        // RFC 3339 timestamp string in Step 1
}
```

5. Derive on both: `Debug, Clone, PartialEq, Serialize, Deserialize`.
6. Unit tests in `lib.rs` (or `tests/`): serde JSON round-trip of a populated `Profile`; empty-profile round-trip.

### Step 4.2 — Wire the CLI subcommands
1. In `crates/tgk-cli/Cargo.toml` add: `tgk-core = { path = "../tgk-core" }`.
2. In `main.rs` extend `enum Commands` with:

```rust
/// Create and display a user profile (in-memory, Step 1)
Profile(ProfileCmd),
```

3. `ProfileCmd` (clap derive) with a nested subcommand `Sub` enum:

   - **`create`** — flags:
     - `--name <NAME>` (required)
     - `--email <EMAIL>` (repeatable, `Vec<String>`, zero or more)
     - `--username <NAME>` (repeatable)
     - `--account <SERVICE:LOGIN>` (repeatable, parsed into `Account`; `is_broker = false` default in Step 1)
   - **`show`** — takes a `--json` flag; in Step 1 this prints a *sample* or the just-created profile — **decision: Step 1 has no storage, so `show` is NOT implemented yet.** Only `create` exists in Step 1; `show` lands in Step 2 when persistence exists. (Avoids dead code.)

4. `create` handler:
   - Parse each `--account` value by splitting on the **first** `:` into `service`/`login`; on missing or malformed `:` → `eprintln!` a clear error, return non-zero exit code (use `std::process::ExitCode`, or `anyhow` if preferred — but **do not add a new dependency** unless the error path genuinely needs it; `std` errors suffice for Step 1).
   - Build the `tgk_core::Profile`.
   - **Return it to the user** (two modes):
     - Default: human-readable summary, e.g.:
       ```
       Profile created (in-memory):
         Name:     Ethan Clark
         Emails:   ethan@example.com
         Usernames: ethan
         Accounts: 0
       ```
     - `--json`: `serde_json::to_string_pretty(&profile)` printed to stdout, nothing else on stdout.
5. **Keep `Execute` working** — CI's smoke step (`cargo run --release -- execute`) and the Docker CI run it. Two valid options; pick one:
   - (a) Leave `Execute` untouched (recommended — zero CI churn), or
   - (b) Update `ci.yml` smoke steps in the **same commit** if you rename/remove it.

### Step 4.3 — Tests (in `crates/tgk-cli/src/main.rs` test module)
Extend the existing test style:
1. `profile_create_parses` — `Cli::try_parse_from(["tgk", "profile", "create", "--name", "X", "--email", "a@b.c", "--account", "github:ethan"])` → asserts name, 1 email, 1 account with `service == "github"`.
2. `account_with_colon_in_login_parses` — `--account "example.com:user:sub"` splits on first colon only (login = `"user:sub"`).
3. `profile_json_roundtrip` — build a profile, serialize with `serde_json::to_string`, deserialize, `assert_eq`.
4. Keep all three existing tests passing (`debug_assert`, `execute_is_parsed`, `unknown_command_is_rejected`).

### Step 4.4 — Verify locally, then commit
1. `cargo fmt --all` → `cargo fmt --all --check` clean.
2. `cargo clippy --workspace --release -- -D warnings` → zero warnings.
3. `cargo test --workspace --release` → all green.
4. Manual smoke:
   - `cargo run --release -- execute` → prints `Hello World`
   - `cargo run --release -- profile create --name "Ethan" --email "ethan@example.com" --account "github:ethan"` → summary
   - same with `--json` → valid JSON (pipe to `python3 -m json.tool` to verify)
5. Commit (single commit is fine for Step 1):
   ```
   feat(core): add Profile model + tgk profile create CLI

   - new crates/tgk-core library crate (Profile/Account, serde)
   - clap subcommand: tgk profile create (--name --email --username --account --json)
   - unit tests: parsing, first-colon split, JSON roundtrip
   ```
6. Push the branch; open PR to `main`. **CI must be green on all 3 OSes + both docker platform jobs before merge.**

---

## 5. Constraints & Rules

**Do:**
- Add `serde`/`serde_json` via `[workspace.dependencies]` only; member crates reference `workspace = true`.
- Derive `Debug, Clone, PartialEq, Serialize, Deserialize` on all public data types.
- Keep output deterministic: summary fields in a fixed order; `--json` is the *only* stdout content in JSON mode.
- Preserve existing tests and the `execute` subcommand's CI contract (or update `ci.yml` atomically).
- Follow existing commit style: imperative subject, brief body.
- Rust code style: no `unwrap`/`expect` outside tests (CI clippy will gate it).

**Do NOT:**
- Add SQLite, `rusqlite`, `argon2`, `aes-gcm`, or any crypto/storage dependency — that is Steps 2–3.
- Add `anyhow`/`thiserror` unless a concrete error path requires it (Step 1 does not).
- Write to disk, read env vars, or touch the network in any new code path.
- Print profile data to stderr.
- Change the root `Cargo.toml` license, edition, or `rust-version`.
- Remove or rename `crates/tgk-cli` or the `tgk` binary name (Dockerfile and CI depend on `target/release/tgk`).
- Introduce unsafe code.
- Push directly to `main`; use a branch + PR.

**Formatting rules:**
- All new Rust files pass `cargo fmt` with zero diff.
- `--help` output for `profile create` must read as a complete sentence per help string (clap derive `///` docs).
- JSON output: pretty-printed, stable key order (serde field order), trailing newline.

---

## 6. Output Specification

### Final state of the repository
```
tgk/
├── Cargo.toml                          # + serde, serde_json in [workspace.dependencies]
├── Cargo.lock
├── crates/
│   ├── tgk-core/
│   │   ├── Cargo.toml                  # new crate, serde deps
│   │   └── src/lib.rs                  # Profile, Account, unit tests
│   └── tgk-cli/
│       ├── Cargo.toml                  # + tgk-core path dep
│       └── src/main.rs                 # + Profile subcommand, + 3 new tests
```

### Command surface after this step
| Command | Behavior |
|---|---|
| `tgk execute` | Unchanged — prints `Hello World` |
| `tgk --version` | `tgk 0.0.1` |
| `tgk profile create --name N [--email E]... [--username U]... [--account SVC:LOGIN]... [--json]` | Builds in-memory profile, prints summary (or pretty JSON with `--json`), exits 0; malformed `--account` → clear stderr message, non-zero exit |

### Definition of Done
- [ ] `cargo test --workspace --release` green, including ≥5 new tests (2 core + 3 CLI).
- [ ] `cargo clippy --workspace --release -- -D warnings` clean.
- [ ] `cargo fmt --all --check` clean.
- [ ] `tgk profile create --name "Ethan" --email "a@b.c" --account "github:ethan" --json` emits valid, parseable JSON.
- [ ] `tgk execute` still works; CI smoke steps unaffected.
- [ ] One PR to `main`; CI green on ubuntu + windows + macos + docker (amd64 & arm64).
- [ ] Zero new storage/crypto/network dependencies in `Cargo.lock` beyond `serde`, `serde_json` (and their transitive `serde_derive`/`itoa`/`ryu`).
