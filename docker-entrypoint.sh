#!/bin/sh

set -e

cmd="$@"

until PGPASSWORD=${ACCESSO_DATABASE__PASSWORD} psql -h ${ACCESSO_DATABASE__HOST} -U ${ACCESSO_DATABASE__USER} ${ACCESSO_DATABASE__DATABASE} -c '\q'; do
  >&2 echo "Postgres is unavailable - sleeping"
  sleep 1
done

>&2 echo "Postgres is up - executing command"
cd /app && sqlx migrate run && exec $@
