# Espanso Snippets for Alfred

Fuzzy-search your [espanso](https://espanso.org) snippets from Alfred and expand the selected one into the frontmost app.

Useful when you have more snippets than you can remember triggers for: find one by its trigger, label, or contents, and let espanso insert it — including forms, dates, shell output, and other dynamic variables.

## Usage

- Press <kbd>⌘</kbd><kbd>;</kbd> or type `es` in Alfred, then search.
- <kbd>↩</kbd> expands the snippet via `espanso match exec`; snippets with forms open espanso's form window.
- <kbd>⌘</kbd><kbd>L</kbd> previews the full snippet in Large Type.

Search uses [fzf syntax](https://github.com/junegunn/fzf#search-syntax), e.g. `'exact`, `^:prefix`, or multiple space-separated terms.

## Requirements

- [Alfred](https://www.alfredapp.com) with the Powerpack
- [espanso](https://espanso.org) 2.x, running
- [fzf](https://github.com/junegunn/fzf) — `brew install fzf`
- [jq](https://jqlang.org) — built into macOS 15+, otherwise `brew install jq`

## Install

```sh
make
open Espanso.alfredworkflow
```
