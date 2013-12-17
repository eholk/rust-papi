.phony: all docs check

all: docs
	rustpkg build papi

check:
	rustpkg test papi

docs:
	rustdoc src/papi/lib.rs
