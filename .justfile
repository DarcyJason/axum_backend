set dotenv-load := true

# Show the list of available commands
default:
    @just --list --unsorted

# Check the preparation of the project manually
check:
    @-./scripts/check.sh

# Run the whole project
run:
    @just fmt
    @-./scripts/run.sh

alias r := run

# Format the code
fmt:
    @cargo fmt --all
