# Rust Shell

A minimal Unix-like shell written in Rust.

This project is a practical exploration of Rust through systems programming. The goal is straightforward: understand the language by building something real — a working shell that can parse commands, handle builtins, search `PATH`, and execute external programs.

No frameworks. No magic. Just systems programming fundamentals.

---

## Features

### Builtin Commands

- `exit` — exits the shell
- `echo` — prints arguments to stdout
- `type` — identifies whether a command is a builtin or an executable found in `PATH`
- `pwd` — prints the current working directory
- `cd` — changes the current working directory
  - Supports absolute paths
  - Supports relative paths (`./`, `../`, etc.)
  - Supports `~` (home directory expansion)

### External Command Execution

- Searches for executables across all directories listed in `PATH`
- Verifies execute permissions before running
- Spawns processes using `std::process::Command`
- Passes command-line arguments correctly to child processes

**Example:**

```
$ grep foo file.txt
```

The shell resolves this by:

1. Iterating through each directory in `PATH`
2. Locating `/usr/bin/grep`
3. Executing it with the provided arguments `foo file.txt`

---

## Getting Started

Clone the repository:

```bash
git clone <repo-url>
cd <repo-folder>
```

Build and run:

```bash
cargo run
```

You will see a `$` prompt indicating the shell is ready.

---

## Usage

```
$ pwd
/home/user

$ cd /usr/local
$ pwd
/usr/local

$ type echo
echo is a shell builtin

$ type grep
grep is /usr/bin/grep

$ ls
# executes the external ls binary normally
```

---

## What This Project Covers

Building this shell touches on a broad range of Rust and systems programming concepts:

- Ownership and borrowing in real-world, stateful code
- The distinction between `String` and `&str`
- Working with `Path` and `PathBuf` for cross-platform path handling
- Reading and using environment variables (`PATH`, `HOME`)
- Querying filesystem metadata and checking permissions
- Spawning and managing child processes
- Error handling with `Result` and `Option`
- Mutating process state (`cd`, `pwd`)

---

## Project Goals

This project is not intended to be a production-ready shell. It exists to:

- Learn Rust by building something concrete and non-trivial
- Understand how a shell interacts with the operating system at a low level
- Build intuition for systems programming in a memory-safe language

Every feature implemented required a genuine understanding of how Unix works under the hood.

---

## Planned Improvements

- More robust error handling and user-facing error messages
- Handling of quoted and escaped arguments
- Pipe support (`|`)
- I/O redirection (`>`, `<`, `>>`)
- Job control and background processes (`&`)

---

## Motivation

The best way to learn a systems language is to build a system.

Writing this shell is an exercise in understanding how the OS manages processes, how the filesystem resolves paths, how environment variables shape runtime behavior, and how Rust enforces correctness at compile time rather than at runtime.

If you are learning Rust, consider building your own shell. It forces you to engage with both the language and the operating system in a way that tutorials rarely achieve.
