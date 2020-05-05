# librollenspielsache

![Crates.io](https://img.shields.io/crates/d/librollenspielsache)
![Docs](https://docs.rs/librollenspielsache/badge.svg)
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/deciduously/librollenspielsache/Rust)

WIP - there's like nothing actually here yet.  This is more a guiding plan.

The Rollenspielsache is a set of tools for managing tabletop RPG games.  The goal is to handle the mechanics seamlessly, allowing you to participate or run a tabletop encounter without getting distracted by rule clarifications.

## Overview

This repo contains the core library.  Use `cargo build --release` to produce `target/release/librollenspiel.so`.  Then, add this directory to your `$LD_LIBRARY_PATH` to create a binding.  Use `cargo test` to run the tests.

### Features

This library aims to cover as much territory as possible, reducing automating as many interactions as possible for a seamless experience while running (or playing) a game, whether that's in person or digitally via another system like roll20.

#### Types

* Party, either player or NPC (for encounters) - APL, CR
* Character - class, race
* Item
* Skill/Ability/Spell (given by items)

Systems - make everything as agnostic as possible:

* D&D 5E
* Starfinder

TODO roll20 integration?

Data-ready

* C-compatible - see [librollenspielsache-rb](https://github.com/deciduously/librollenspielsache-rb) for an example of how to use from another language.
* Redis-ready - each type has a method to build an appropriate Redis command for insertion.

## Usage

Control the build with `cargo`.  Use `make all` as a shortcut to put the compiled shared object file for FFI usage in `dist/`.  Point your linker to this file to use it.
