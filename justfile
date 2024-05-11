set shell := ["bash", "-c"]

alias l := lint
alias b := build
alias t := test
alias r := run
alias c := clean

# Simplified project path determination using a function-like approach
set-proj lesson:
    # Use a case-like structure for pattern matching
    @case "{{lesson}}" in \
        *basics*) echo "./week2/1-basics/proj";; \
        *control-flow*) echo "./week2/2-control-flow/proj";; \
        *functions*) echo "./week2/3-functions/proj";; \
        *structs*) echo "./week3/1-structured-data/proj";; \
        *lab-struct*) echo "./week3/2-struct-lab/proj";; \
        *associated-fn*) echo "./week3/3-methods-lab/proj";; \
        strings) echo "./week3/4-strings/proj";; \
        *lab-strings*) echo "./week3/5-strings-lab/proj";; \
        vectors) echo "./week3/6-intro-to-vectors/proj";; \
        lab-vec) echo "./week3/7-lab-vectors/proj";; \
        enums) echo "./week3/8-enums/proj";; \
        lab-enums) echo "./week3/9-lab-enums/proj";; \
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
    cd $(just set-proj {{project}}) && cargo fmt --all && cargo clippy

clean project:
    @echo "Cleaning {{project}}"
    @cd $(just set-proj {{project}}) && cargo clean

install-fmt:
    rustup component add rustfmt 2>/dev/null

install-clippy:
    rustup component add clippy 2>/dev/null
