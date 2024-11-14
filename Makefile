##
# Simple resturant API
#
# @file
# @version 0.1

PROJECT_DIR := $(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
SHELL := /bin/sh
.DEFAULT_GOAL := help
args?=$(filter-out $@,$(MAKECMDGOALS))
CARGO-exists: ; @which cargo > /dev/null 2>&1

.PHONY: task
## Run a specific task inside the cargo makefile, modules/Makefile.toml
task: CARGO-exists check-args
	@if [ -z "$(args)" ]; then \
		echo "Please provide arguments to this rule." >&2; \
		exit 1; \
	fi
	@cargo make --makefile $(PROJECT_DIR)/modules/Makefile.toml -t $(args) --cwd $(PROJECT_DIR)/modules

.PHONY: help
help:
	@echo "$$(tput setaf 2)Rules:$$(tput sgr0)";sed -ne"/^## /{h;s/.*//;:d" -e"H;n;s/^## /---/;td" -e"s/:.*//;G;s/\\n## /===/;s/\\n//g;p;}" ${MAKEFILE_LIST}|awk -F === -v n=$$(tput cols) -v i=4 -v a="$$(tput setaf 6)" -v z="$$(tput sgr0)" '{printf"- %s%s%s\n",a,$$1,z;m=split($$2,w,"---");l=n-i;for(j=1;j<=m;j++){l-=length(w[j])+1;if(l<= 0){l=n-i-length(w[j])-1;}printf"%*s%s\n",-i," ",w[j];}}'

%:
	@:

# end
