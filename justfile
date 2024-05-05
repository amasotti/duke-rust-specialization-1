set shell := ["bash", "-c"]

alias l := lint
alias b := build
alias t := test
alias r := run

# Simplified project path determination using a function-like approach
set-proj lesson:
    # Use a case-like structure for pattern matching
    @case "{{lesson}}" in \
        *basics*) echo "./week2/1-basics/proj";; \
        *control-flow*) echo "./week2/2-control-flow/proj";; \
        *functions*) echo "./week2/3-functions/rust-functions";; \
        *) echo "Error: Project '{{lesson}}' not found" >&2; exit 1;; \
    esac



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
