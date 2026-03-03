# Journey Terminal

Journey Terminal helps you customize your terminal in a clean, safe, and reversible way.

It’s for people who want their terminal to look and behave exactly how they like — without breaking things, digging through random dotfiles, or pasting mystery config from the internet.

Journey Terminal is currently CLI-first.  
A GUI will be built on top of the same core engine.
---

## What Is It?

Journey Terminal is a tool that:

- Is built to manage terminal “profiles” safely and transparently
- Keeps your changes organized and easy to undo
- Works across Linux, macOS, and Windows (eventually)
- Avoids messy, hard-to-maintain dotfile edits

Think of it as a control panel for your shell.

---

## Why It Exists

Customizing your terminal usually means:

- Editing `.bashrc` or `.zshrc`
- Copying snippets from Reddit
- Installing tools without tracking what changed
- Breaking something and not knowing how to fix it

Journey Terminal aims to solve that.

You should always know:
- What you can change
- What that change looks like before install
- What changed
- Where it changed
- How to undo it

---

## What It Does (So Far)

Right now:

- Detects your platform and shows where Journey Terminal stores its configuration directory and file.
- Sets up a clean configuration directory
- Lays the groundwork for profiles and safe configuration

Next up:

- Initialize and manage configuration
- Apply named profiles
- Backup and restore modified files
- “Dry run” mode to preview changes before applying

---

## What It Does NOT Do (Yet)

- It does not replace your terminal emulator.
- It does not manage your entire desktop.
- It does not silently edit your files without telling you.

The first two will change over time.

The last one will not.

## Philosophy

Journey Terminal is built around:

**Clarity**  
No hidden changes.

**Reversibility**  
Everything can be undone.

**Portability**  
It should work the same way on different systems.

**Growth**  
Start simple. Add power over time.

---

## Quick Start

You need Rust installed (via rustup).

Run:

```bash
cargo run -- doctor

### `config init`

Creates a versioned configuration file safely.

```bash
cargo run -- config init