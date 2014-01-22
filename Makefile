.phony: all docs check clean

all: docs
	rustpkg build papi

check:
	rustpkg test papi

docs:
	rustdoc src/papi/lib.rs

clean:
	rustpkg clean papi
