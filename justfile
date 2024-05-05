set shell := ["bash", "-c"]

alias l := lint
alias b := build
alias t := test
alias r := run

set-proj lesson:
    @if echo "{{lesson}}" | grep -qi "basics"; then \
      echo "./week2/1-basics/proj"; \
    elif echo "{{lesson}}" | grep -qi "control-flow"; then \
      echo "./week2/2-control-flow/proj"; \
    elif echo "{{lesson}}" | grep -qi "functions"; then \
      echo "./week2/3-functions/rust-functions"; \
    else \
      echo "Error: Project '{{lesson}}' not found" >&2; \
      exit 1; \
    fi


build project:
    @echo "Building {{project}}"
    @cd $(just set-proj {{project}}) && cargo build

test project:
    @echo "Testing {{project}}"
    @cd $(just set-proj {{project}}) && cargo test

run project:
    @echo "Running {{project}}"
    @cd $(just set-proj {{project}}) && cargo run

lint project: install-fmt install-clippy
    @echo "Linting {{project}}"
    cd $(just set-proj {{project}}) && cargo clippy && cargo fmt --all

install-fmt:
    rustup component add rustfmt 2>/dev/null

install-clippy:
    rustup component add clippy 2>/dev/null
