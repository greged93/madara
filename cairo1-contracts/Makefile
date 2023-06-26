# cairo-files
CAIRO_FILES=$(shell find . -name "*.cairo" | sed 's|./||')
COMPILED_FILES=$(patsubst %.cairo, %.sierra.json, $(CAIRO_FILES))

# Specify the required version
REQUIRED_VERSION = cairo-compile 2.0.0-rc2

# Command to retrieve the version of `mybinary`
VERSION = $(shell cairo-compile --version)

.PHONY: all
check:
	@if [ "$(VERSION)" != "$(REQUIRED_VERSION)" ]; then \
		echo "Incorrect 'cairo-compile' version. Expected $(REQUIRED_VERSION), but got $(VERSION)."; \
		exit 1; \
	fi

all: check compile

compile: $(COMPILED_FILES)

%.sierra.json: %.cairo
	starknet-compile $< build/$(notdir $@) --allowed-libfuncs-list-name all && \
	starknet-sierra-compile build/$(notdir $@) build/$(patsubst %sierra.json,%casm.json,$(notdir $@)) --allowed-libfuncs-list-name all

