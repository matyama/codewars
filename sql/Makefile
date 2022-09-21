.PHONY: lint fmt

SCRIPTS := scripts

# Hack: lint only specific scripts
lint:
	@sqlfluff lint $(SCRIPTS)

fmt:
	@sqlfluff fix $(SCRIPTS)
