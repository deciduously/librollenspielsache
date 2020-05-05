##
# rollenspielsache
#
# @file
# @version 0.1
NAME=rollenspielsache
LIBNAME=lib$(NAME)
LIBPATH=target/release
RUSTSRC=src

ifeq ($(shell uname),Darwin)
    EXT := dylib
else
    EXT := so
endif

LIBFILE=$(LIBNAME).$(EXT)
OUTPATH=$(LIBPATH)/$(LIBFILE)

all: $(OUTPATH)

test:
	cargo test

$(OUTPATH): $(RUSTSRC)/lib.rs Cargo.toml
	cargo build --release

clean:
	cargo clean

docs: all
	cargo doc
	cp -r target/doc ./docs

.PHONY: all test clean docs

# end
