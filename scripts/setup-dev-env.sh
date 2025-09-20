#!/bin/bash

# TODO: what's a more crossplatform solution? Cmake?

# Print commands and their args
set -x
# Exit (-e) on any error (-o pipefail)
set -eo pipefail

set -x
set -eo pipefail
if ! [ -x "$(command -v psql)" ]; then
    echo >&2 "Error: psql is not installed. User a system package manager to install (postgresql-libs for Arch)"
    exit 1
fi
if ! [ -x "$(command -v sqlx)" ]; then
    echo >&2 "Error: sqlx-cli is not installed. Use a sysyem package manager or cargo to install."
    exit 1
fi

# Use the set variables or assign the defaults
DB_USER="${POSTGRES_USER:=postgres}"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=5432}"
DB_HOST="${POSTGRES_HOST:=localhost}"

docker run \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p "${DB_PORT}":5432 \
    -d postgres \
    postgres -N 1000 # Max conn num for testing only

sleep 2

>&2 echo "Postgres is up and running on port ${DB_PORT}!"

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
sqlx migrate add create_subscriptions_table
