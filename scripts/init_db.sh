#! /bin/bash

set -x 
set -eo pipefail 

type psql>/dev/null 2>&1 || {
	echo >&2 "Error: psql is not installed."
	exit 1
}

type sqlx>/dev/null 2>&1 || {
	echo >&2 "Error sqlx is not installed."
	echo >&2 "Use:"
	echo >&2 "    cargo install sqlx-cli"
	echo >&2 "to install it."
	exit 1
}

POSTGRES_IMAGE="postgres:latest"
# Check if the custom user hash been set, otherwise default to 'postgres'
DB_USER="${POSTGRES_USER:=postgres}"
# Check if a custom password has been set, otherwise default to 'password'
DB_PASSWORD="${POSTGRES_PASSWORD:=postgres}"
# Check if custom database name has ben set, otherwise default to 'newsletter'
DB_NAME="${POSTGRES_DB:=newsletter}"
# Check if a custom port has been set otherwise default ot '5432'
DB_PORT=${POSTGRES_PORT:=5432}

CONTAINER_PORT=5432
# 
IMAGE_NAME="${POSTGRES_IMAGE_NAME:=db_newsletter_pg}"

# Pull the latest PostgreSQL image
echo "Pulling PostgreSQL image..."
docker pull $POSTGRES_IMAGE

# Check if a container with the same name is already running
if [ "$(docker ps -q -f name=$IMAGE_NAME)" ]; then
    echo "A running container with the name $IMAGE_NAME already exists. Stopping and removing it..."
    docker stop $IMAGE_NAME
    docker rm $IMAGE_NAME
elif [ "$(docker ps -aq -f name=$IMAGE_NAME)" ]; then
    echo "A container with the name $IMAGE_NAME exists but is not running. Removing it..."
    docker rm $IMAGE_NAME
fi

# docker run --name some-postgres -e POSTGRES_PASSWORD=mysecretpassword -d postgres

# Launch postgres using Docker
docker run \
	--name "${IMAGE_NAME}" \
	-e "POSTGRES_USER=${DB_USER}" \
	-e "POSTGRES_PASSWORD=${DB_PASSWORD}" \
	-e "POSTGRES_DB=${DB_NAME}" \
	-p "${DB_PORT}:${CONTAINER_PORT}" \
	-d "${POSTGRES_IMAGE}" \
	-N 1000 
# Increased maximum number of connections for testing purposes

# Keep pinging postgres until it's ready to accept commands
export PGPASSWORD="${DB_PASSWORD}"
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do 
	>&2 echo "Postgres is still unavailable - sleeping"
	sleep 1 
done


>&2 echo "Postgres is up and running on port ${DB_PORT}"

docker ps -q -f name=$IMAGE_NAME 

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
sqlx database create

CREATE_SUBSCRIPTIONS_TABLE="create_subscriptions_table"
MIGRATIONS="migrations"

if [[ ! -d ${MIGRATIONS} ]]; then 
	sqlx migrate add ${CREATE_SUBSCRIPTIONS_TABLE}

	# Find files ending with the specific string
	FILES=$(find "$MIGRATIONS" -type f -name "*${CREATE_SUBSCRIPTIONS_TABLE}.sql")

	# Print the found files
	for FILE in $FILES; {
		printf "CREATE TABLE subscriptions(\n\tid uuid NOT NULL,\n\tPRIMARY KEY (id),\n\temail TEXT NOT NULL UNIQUE,\n\tname TEXT NOT NULL,\n\tsubscribed_at timestamp NOT NULL\n);\n" >> ${FILE}
	
	}
fi

sqlx migrate run

# rm -rf "migrations"
