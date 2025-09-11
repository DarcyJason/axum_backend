set dotenv-load := true

# Show the list of available commands
default:
    @just --list --unsorted

postgres_migrations := "migrations/postgres/"
DATABASE_URL := env_var('POSTGRES_DATABASE_URL')

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

# Use 'just mp help' to show help message
migrate-postgres command arg="":
    @if [ "{{command}}" = "help" ]; then \
        echo "Use 'just mp info' to show the status of migrations"; \
        echo "Use 'just mp reset' to reset migrations"; \
        echo "Use 'just mp add <MIGRATION_NAME>' to add a new migration"; \
        echo "Use 'just mp run' to run all migrations"; \
        echo "Use 'just mp revert <TARGET_VERSION>' to revert a migration"; \
    elif [ "{{command}}" = "info" ]; then \
        DATABASE_URL="{{DATABASE_URL}}" sqlx migrate info --source {{postgres_migrations}}; \
    elif [ "{{command}}" = "reset" ]; then \
        DATABASE_URL="{{DATABASE_URL}}" sqlx database reset --source {{postgres_migrations}}; \
    elif [ "{{command}}" = "add" ]; then \
        DATABASE_URL="{{DATABASE_URL}}" sqlx migrate add -r {{arg}} --source {{postgres_migrations}}; \
    elif [ "{{command}}" = "run" ]; then \
        DATABASE_URL="{{DATABASE_URL}}" sqlx migrate run --source {{postgres_migrations}}; \
    elif [ "{{command}}" = "revert" ]; then \
        DATABASE_URL="{{DATABASE_URL}}" sqlx migrate revert --target-version {{arg}} --source {{postgres_migrations}}; \
    fi

alias mp := migrate-postgres
