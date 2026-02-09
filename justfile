project_root := justfile_directory()

serve:
    cd apps/dialogue && dx serve --addr 0.0.0.0

build:
    cargo build

install-dioxus-cli:
    @just _install_cargo_tool dioxus-cli

_install_cargo_tool *ARGS:
    cargo binstall --locked --root {{ project_root }} {{ARGS}}

