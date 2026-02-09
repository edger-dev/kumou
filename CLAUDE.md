# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Kumou is a Rust-based project using Dioxus 0.7 for web applications. The repository is organized as a Cargo workspace containing:

- **Workspace libraries** (`crates/`): Core Rust libraries
  - `kumou-core`: Core functionality library
  - `kumou-japanese`: Japanese language-specific utilities

- **Applications** (`apps/`): Dioxus-based web applications
  - `dialogue`: A fullstack Dioxus 0.7 app with routing and server functions

## Tools Usage

Always use Context7 MCP when I need library/API documentation, code generation, setup or configuration steps without me having to explicitly ask.

## Build Commands

This project uses `just` as a command runner. Common commands:

```bash
# Build the entire workspace
just build
# or
cargo build

# Install Dioxus CLI (required for serving apps)
just install-dioxus-cli

# Serve the dialogue app (web platform)
cd apps/dialogue
dx serve --platform web

# Serve for desktop platform
cd apps/dialogue
dx serve --platform desktop

# Run tests
cargo test

# Run tests for specific crate
cargo test -p kumou-core
cargo test -p kumou-japanese
```

## Dioxus 0.7 Architecture

The `dialogue` app follows the standard Dioxus 0.7 fullstack pattern:

### Key Files
- `apps/dialogue/src/main.rs`: App entry point, defines routes and root App component
- `apps/dialogue/src/views/`: Route components (Home, Blog, Navbar layout)
- `apps/dialogue/src/components/`: Reusable UI components (Hero, Echo with server functions)
- `apps/dialogue/Dioxus.toml`: Dioxus configuration
- `apps/dialogue/assets/`: Static assets (CSS, images)

### Dioxus 0.7 Critical Information

**IMPORTANT**: This project uses Dioxus 0.7, which has breaking API changes from previous versions:
- `cx`, `Scope`, and `use_state` are **removed** - use `use_signal` instead
- Components are annotated with `#[component]` macro
- Signals created with `use_signal(|| initial_value)`
- Server functions use `#[post]` and `#[get]` macros

### Feature Flags

The dialogue app requires these Cargo.toml features:
```toml
[features]
default = ["web", "server"]
web = ["dioxus/web"]
server = ["dioxus/server"]
```

**Server Functions**: For `#[post]` or `#[get]` macros to work without warnings, ensure the `server` feature is in your Cargo.toml features section.

### Routing Structure
- Routes defined in single enum in `main.rs`
- Uses `#[derive(Routable)]` with `#[route("/path")]` attributes
- Dynamic segments captured with `:param` syntax (e.g., `/blog/:id`)
- Layouts created with `#[layout(ComponentName)]` and `Outlet<Route> {}`

### Assets
- Assets referenced with `asset!("/assets/path")` macro
- Paths are relative to crate root
- Auto-minification for CSS/JS

### Tailwind CSS
- Automatically handled by Dioxus 0.7 when `dx serve` is run
- Looks for `tailwind.css` in manifest directory
- Can customize via `Dioxus.toml` with `tailwind_input` and `tailwind_output`

## Workspace Configuration

- **Resolver**: Uses Cargo resolver 3
- **Rust version**: 1.93.0 minimum
- **Edition**: 2024
- **Version**: All crates versioned as `0.1.0-dev`

## Development Workflow

1. Make changes to library crates in `crates/`
2. Test with `cargo test -p <crate-name>`
3. Use the dialogue app to test integration
4. Serve with `dx serve` from `apps/dialogue/`

## Important Notes from AGENTS.md

The dialogue app was created following Dioxus 0.7 conventions. Refer to `apps/dialogue/AGENTS.md` for comprehensive Dioxus 0.7 patterns including:
- Component creation with `#[component]` macro
- State management with `use_signal` and `use_memo`
- Context API with `use_context_provider` and `use_context`
- Async operations with `use_resource` and `use_server_future`
- Server-side rendering and hydration requirements
