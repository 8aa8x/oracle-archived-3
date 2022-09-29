# Load environment variables from .env
ifneq (,$(wildcard ./.env))
	include .env
	export
endif

# Export default environment variables
# Please note that you should only change these for you local environment. For other
# environments like beta or prod, set the DATABASE_URL in your .env file.
DB_USER ?= postgres
DB_PASS ?= password
DB_NAME ?= oracle
DB_HOST ?= localhost
DB_PORT ?= 5432
DATABASE_URL ?= postgres://${DB_USER}:${DB_PASS}@${DB_HOST}:${DB_PORT}/${DB_NAME}
DOCKER_CONTAINER_NAME ?= oracle-db

# Colors for echo
_YELLOW := "\033[1;33m" # Yellow text for echo
_RED := "\033[0;31m" # Red text for echo
_RESET := "\033[0m" # Reset text for echo

# Hook to check if command exists
cmd-exists-%:
	@hash $(*) > /dev/null 2>&1 || \
		(echo "${_RED}ERROR: '$(*)' must be installed and available on your PATH.${_RESET}"; exit 1)

# Colors for printf
_ERROR := "\033[31m[%s]\033[0m %s\n" # Red text for printf
_SUCCESS := "\033[32m[%s]\033[0m %s\n" # Green text for printf

# If the CI variable is set, it automatically continues
.PHONY: confirm
confirm:
	@if [[ -z "$(CI)" ]]; then \
		REPLY="" ; \
		read -p "⚠ Are you sure? [y/n] > " -r ; \
		if [[ ! $$REPLY =~ ^[Yy]$$ ]]; then \
			printf $(_ERROR) "NO" "Stopping" ; \
			exit 1 ; \
		else \
			printf $(_SUCCESS) "OK" "Continuing" ; \
			exit 0; \
		fi \
	fi


# Hook to wait for postgres
wait-for-postgres: cmd-exists-psql
	@until psql ${DATABASE_URL} -c '\q'; do \
		echo "Postgres is unavailable - trying again"; \
		sleep 1; \
	done
	@echo "Postgres is up and running on port ${DB_PORT}!"

# Hook to create the database docker image
db-init: cmd-exists-docker
	docker run \
		--name="${DOCKER_CONTAINER_NAME}" \
		-e POSTGRES_USER="${DB_USER}" \
		-e POSTGRES_PASSWORD="${DB_PASS}" \
		-e POSTGRES_DB="${DB_NAME}" \
		-p "${DB_PORT}":5432 \
		-d postgres:14-alpine \
		postgres -N 1000

# Create the local database
.PHONY: db-create
db-create: db-init wait-for-postgres cmd-exists-sqlx
	@sqlx database create -D "${DATABASE_URL}"

# Remove the local database container
.PHONY: db-cleanup
db-cleanup: cmd-exists-docker
	@docker kill ${DOCKER_CONTAINER_NAME}
	@docker rm ${DOCKER_CONTAINER_NAME}

# Run migrations
.PHONY: db-migrate
db-migrate: cmd-exists-sqlx
	@sqlx migrate run --dry-run
	@echo Are you sure you want to run this on ${_YELLOW}${DB_HOST}${_RESET}?
	@if $(MAKE) -s confirm ; then \
		sqlx migrate run; \
	fi

# Revert migrations
.PHONY: db-revert
db-revert: cmd-exists-sqlx
	@sqlx migrate revert --dry-run
	@echo "Are you sure you want to run this on ${_YELLOW}${DB_HOST}${_RESET}?"
	@if $(MAKE) -s confirm ; then \
		sqlx migrate revert; \
	fi

# Hard reset the database
.PHONY: db-reset
db-reset: db-cleanup db-create db-migrate
