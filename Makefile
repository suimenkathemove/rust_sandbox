ifndef VERBOSE
.SILENT:
endif

.PHONY: run
run:
	PROJECT=$(PROJECT) docker compose up --build
