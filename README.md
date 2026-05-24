# APS — Aether Package System

APS is a modern experimental Linux package manager written in Rust.

It is designed around:

* simple package manifests
* portable package formats
* decentralized repositories
* future container integration
* lightweight package management

APS is part of the broader Aether ecosystem.

---

# Current Features

* `.aps` package format
* package build command
* package install command
* package removal
* SQLite package database
* local repository support
* package search
* TOML manifests (`aps.toml`)
* installed file tracking

---

# Project Goals

APS aims to provide:

* a clean package management experience
* distro-independent package workflows
* modern package manifests
* lightweight repository architecture
* future ACS integration

Long-term goals include:

* dependency resolution
* package signatures
* Git-based packages
* remote repositories
* containerized applications
* ACS integration

---

# Example Commands

## Build Package

```bash
aps build ./hello
```

---

## Install Package

```bash
aps install hello
```

---

## Remove Package

```bash
aps remove hello
```

---

## Search Repository

```bash
aps search hello
```

---

## List Installed Packages

```bash
aps list
```

---

# Example Package Layout

```text
hello/
├── aps.toml
└── files/
    └── usr/
        └── bin/
            └── hello
```

---

# Example Manifest

```toml
[package]
name = "hello"
version = "1.0.0"

[dependencies]
aps = []
```

---

# Repository Layout

```text
repo/
├── index.json
└── packages/
    ├── hello.aps
    └── foo.aps
```

---

# Build From Source

## Requirements

* Rust
* Cargo

---

## Clone Repository

```bash
git clone https://github.com/ronu34/aps.git
cd aps
```

---

## Build APS

```bash
cargo build
```

---

## Run APS

```bash
cargo run -- --help
```

---

# Development Status

APS is currently an experimental early-stage project.

The package format and APIs may change frequently.

---

# Roadmap

## Core Package Manager

* [x] Build packages
* [x] Install packages
* [x] Remove packages
* [x] Local repository support
* [x] Package search
* [ ] Dependency handling
* [ ] Package upgrades
* [ ] Package verification
* [ ] Package signatures

---

## Repository System

* [ ] Remote HTTP repositories
* [ ] Mirrors
* [ ] Package metadata cache
* [ ] Maintainer metadata

---

## ACS Integration

* [ ] ACS manifests
* [ ] Permissions system
* [ ] Container runtimes
* [ ] Sandboxed applications

---

# Philosophy

APS focuses on:

* simplicity
* portability
* modularity
* clean architecture
* modern packaging workflows

---

# License

GNU License
