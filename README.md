# Espanso Snippets for Alfred

Fuzzy-search your [espanso](https://espanso.org) snippets from Alfred and expand the selected one into the frontmost app.

Useful when you have more snippets than you can remember triggers for: find one by its trigger, label, or contents, and let espanso insert it — including forms, dates, shell output, and other dynamic variables.

## Usage

- Press <kbd>⌘</kbd><kbd>;</kbd> or type `es` in Alfred, then search.
- <kbd>↩</kbd> expands the snippet via `espanso match exec`; snippets with forms open espanso's form window.
- <kbd>⌘</kbd><kbd>L</kbd> previews the full snippet in Large Type.

Search uses [skim](https://github.com/skim-rs/skim)'s extended syntax (fzf-compatible), e.g. `'exact`, `^:prefix`, `!inverse`, or multiple space-separated terms.

## Requirements

- [Alfred](https://www.alfredapp.com) with the Powerpack
- [espanso](https://espanso.org) 2.x, running

## Install

Download `Espanso.alfredworkflow` from the latest release, or build it from source (requires a [Rust toolchain](https://rustup.rs)):

```sh
rustup target add aarch64-apple-darwin x86_64-apple-darwin
make
open Espanso.alfredworkflow
```
