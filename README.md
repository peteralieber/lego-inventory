# LEGO Inventory

This repository is the starting point for a Rust-based desktop and web app for organizing and tracking a LEGO collection.

## Status

The codebase currently contains the default `eframe` application scaffold, while the main project-specific planning work lives in the documentation.

## Repository layout

| Path | Purpose |
| --- | --- |
| `src` | Rust application source built with `egui` and `eframe` |
| `doc/organization.md` | Draft organization model for LEGO part categories and storage |
| `assets` | Static assets used by the app |

## Running the app locally

### Native

```bash
cargo run --release
```

### Web

1. Install the web target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
2. Install Trunk:
   ```bash
   cargo install --locked trunk
   ```
3. Start the local web server:
   ```bash
   trunk serve
   ```

Then open `http://127.0.0.1:8080/index.html#dev` in your browser.

## Documentation

The primary project-specific document is:

- [`doc/organization.md`](./doc/organization.md) - category definitions, collapse rules, and storage planning for a LEGO inventory system.

## Next steps

Recommended follow-up work for the repository:

1. Replace the default `eframe` template UI with LEGO inventory workflows.
2. Turn the organization model into application data structures.
3. Add inventory import, editing, and browsing features.
