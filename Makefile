##
# rollenspielsache
#
# @file
# @version 0.1
NAME=rollenspielsache
VERSION=0.2.0
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
DIST=dist
DISTPATH=$(DIST)/$(LIBFILE)

all: clean $(OUTPATH)
	mkdir -p $(DIST)
	cp $(OUTPATH) $(DISTPATH)
	strip $(DISTPATH)

test:
	cargo test

$(OUTPATH): $(RUSTSRC)/lib.rs Cargo.toml
	cargo build --release

clean:
	rm -f $(OUTPATH)
	rm -rf $(DIST)

.PHONY: all test clean

# end
