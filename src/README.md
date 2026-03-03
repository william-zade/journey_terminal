Journey Terminal

Journey Terminal helps you customize your terminal in a clean, safe, and reversible way.

It’s for people who want their terminal to look and behave exactly how they like — without breaking things, digging through random dotfiles, or pasting mystery config from the internet.

Journey Terminal is currently CLI-first.
A GUI will be built on top of the same core engine.

What Is It?

Journey Terminal is a tool that:

Applies terminal “profiles” (themes, prompts, toolsets)

Manages configuration safely and transparently

Keeps changes organized and reversible

Uses structured, versioned TOML configuration

Avoids messy, hard-to-maintain dotfile edits

Is designed to be cross-platform (Linux first, macOS/Windows planned)

Think of it as a control plane for your shell.

Why It Exists

Customizing your terminal usually means:

Editing .bashrc or .zshrc

Copying snippets from Reddit

Installing tools without tracking what changed

Breaking something and not knowing how to fix it

Journey Terminal aims to solve that.

You should always know:

What you can change

What that change will look like before you apply it

What changed

Where it changed

How to undo it

Journey Terminal treats configuration as something to be managed — not hacked.

Current Features
doctor

Detects your platform and shows where Journey Terminal stores its configuration directory and file.

cargo run -- doctor

Example output:

Platform: Linux
Config dir: /home/user/.config/journey_terminal
Config file: /home/user/.config/journey_terminal/config.toml
config init

Creates a versioned configuration file safely.

cargo run -- config init

Creates ~/.config/journey_terminal/

Creates config.toml

Does not overwrite existing configuration

config show

Displays the current configuration in a human-readable format.

cargo run -- config show

Example output:

Journey Terminal config
- Version: 0.1.0
- Default profile: base
- Dry-run by default: true
- File: /home/user/.config/journey_terminal/config.toml
What It Does NOT Do (Yet)

It does not replace your terminal emulator.

It does not manage your entire desktop.

It does not silently edit your files without telling you.

The first two may expand over time.

The last one will not.

Roadmap

Near-term:

config set

Profile definition and listing

Profile apply with dry-run preview

File-level backup and restore

Raw and machine-readable output modes

Long-term:

Profile modules (themes, prompts, tool bundles)

Profile distribution system

Snapshot integration (Snapper / Timeshift awareness)

Cross-platform parity

GUI built on top of the CLI engine

Quick Start

Requirements:

Rust (via rustup)

Cargo

Clone the repository and run:

cargo run -- doctor
cargo run -- config init
cargo run -- config show

This is the beginning of the journey.

What we fixed

Removed duplicated CLI-first lines

Removed diff markers

Removed duplicate doctor command

Removed redundant bullets

Cleaned tone consistency

Reflected actual implemented features

Tightened roadmap

Cleaned structure
