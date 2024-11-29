##
# Simple resturant API
#
# @file
# @version 0.1

PROJECT_DIR := $(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
SHELL := /bin/sh
.DEFAULT_GOAL := help
args?=$(filter-out $@,$(MAKECMDGOALS))
compose := docker compose --file $(PROJECT_DIR)/docker-compose.yml
CARGO-exists: ; @which cargo > /dev/null 2>&1
DIESEL-exists: ; @which diesel > /dev/null 2>&1
DOCKER-exists: ; @which docker > /dev/null 2>&1

.PHONY: task
## Run a specific task inside the cargo makefile, modules/Makefile.toml
task: CARGO-exists DIESEL-exists check-args
	@if [ -z "$(args)" ]; then \
		echo "Please provide arguments to this rule." >&2; \
		exit 1; \
	fi

	@if [ "$(args)" = "run" ]; then \
		$(compose) down --volumes; \
		$(compose) up db -d; \
		echo "Waiting for the database to be ready..."; \
		until docker compose logs db | grep -q "PostgreSQL init process complete"; do \
			echo "Database is not ready yet. Retrying in 2 seconds..."; \
			sleep 2; \
		done; \
	fi
	@cargo make --makefile $(PROJECT_DIR)/modules/Makefile.toml -t $(args) --cwd $(PROJECT_DIR)/modules

.PHONY: run
## Run Application using docker.
## To run Specific Module in docker, use args=<MODULE_NAME> (e.g., server/client)
run: DOCKER-exists
	@if [ -n "$(args)" ]; then \
		$(compose) run --build --rm $(args); \
	else \
		$(compose) up --build -d; \
	fi

.PHONY: test
## Run test with test db.
test: DOCKER-exists DIESEL-exists
	$(compose) down --volumes
	$(compose) up db -d
	@until docker compose logs db | grep -q "PostgreSQL init process complete"; do \
		echo "Database is not ready yet. Retrying in 2 seconds..."; \
		sleep 2; \
	done
	@cargo make --makefile $(PROJECT_DIR)/modules/Makefile.toml -t test --cwd $(PROJECT_DIR)/modules
	$(compose) down --volumes

.PHONY: run-release
## Run release images
run-release: DOCKER-exists
	@OVERRIDE_COMPOSE=release.yml RELEASE=$(or $(args),latest) $(compose) up --detach

.PHONY: help
help:
	@echo "$$(tput setaf 2)Rules:$$(tput sgr0)";sed -ne"/^## /{h;s/.*//;:d" -e"H;n;s/^## /---/;td" -e"s/:.*//;G;s/\\n## /===/;s/\\n//g;p;}" ${MAKEFILE_LIST}|awk -F === -v n=$$(tput cols) -v i=4 -v a="$$(tput setaf 6)" -v z="$$(tput sgr0)" '{printf"- %s%s%s\n",a,$$1,z;m=split($$2,w,"---");l=n-i;for(j=1;j<=m;j++){l-=length(w[j])+1;if(l<= 0){l=n-i-length(w[j])-1;}printf"%*s%s\n",-i," ",w[j];}}'

%:
	@:

# end
