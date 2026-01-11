# jjkk

> A fast, beautiful terminal UI for [Jujutsu](https://martinvonz.github.io/jj/) version control

**jjkk** brings the power of [lazygit](https://github.com/jesseduffield/lazygit)-style interfaces to Jujutsu VCS. Navigate your repository, stage changes, create commits, and manage bookmarks—all from a sleek, keyboard-driven TUI.

![jjkk screenshot placeholder]

## Features

✨ **Intuitive Interface**
- Split-pane view with file list and colorized diff viewer
- Vim-style navigation (hjkl)
- Tab-based workflow: Working Copy, Bookmarks, Log

🚀 **Essential Operations**
- View and navigate file changes with live diffs
- Describe and commit changes with popup prompts
- Create new commits, set bookmarks, and rebase
- Fetch from and push to remote repositories
- Checkout bookmarks interactively

🎨 **Beautiful Design**
- Catppuccin Mocha color theme
- Syntax-highlighted diffs (additions, deletions, hunks)
- Clean, minimal interface built with [ratatui](https://ratatui.rs/)

## Installation

### Prerequisites
- [Jujutsu](https://github.com/martinvonz/jj) must be installed and in your PATH
- Rust 1.70+ (for building from source)

### From Source
```bash
git clone https://github.com/yourusername/jjkk.git
cd jjkk
cargo build --release
./target/release/jjkk
```

## Usage

Run `jjkk` from any directory within a Jujutsu repository:

```bash
jjkk
```

### Keybindings

#### Global
- `q` - Quit
- `1` / `2` / `3` - Switch to Working Copy / Bookmarks / Log tab
- `Tab` / `Shift+Tab` - Cycle through tabs
- `R` - Refresh status

#### Working Copy Tab
- `j` / `k` (or `↓` / `↑`) - Navigate files
- `Shift+J` / `Shift+K` - Scroll diff view
- `d` - Describe current commit
- `c` - Commit changes
- `n` - Create new empty commit
- `b` - Set bookmark on current commit
- `r` - Rebase current commit
- `f` - Git fetch
- `p` - Git push (auto-detects current bookmark)

#### Bookmarks Tab
- `j` / `k` (or `↓` / `↑`) - Navigate bookmarks
- `Enter` - Checkout selected bookmark

#### Log Tab
- `j` / `k` (or `↓` / `↑`) - Navigate commits

#### Popups
- `Enter` - Submit
- `Esc` - Cancel
- Type to enter text, `Backspace` to delete

## Configuration

Configuration file location: `~/.config/jjkk/config.toml`

```toml
[ui]
log_commits_count = 20  # Number of commits to show in Log tab
```

## Roadmap

- [ ] Full syntax highlighting using syntect
- [ ] Help screen (`?` key)
- [ ] Commit details view
- [ ] Bookmark management (delete, rename)
- [ ] Direct jj-lib integration (currently uses subprocess)
- [ ] Customizable themes
- [ ] Split/squash commits
- [ ] Conflict resolution UI

## Why "jjkk"?

`jj` for Jujutsu + `kk` for the Vim motion keys you'll be using constantly. Also, it's fun to type!

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## License

MIT License - See [LICENSE](LICENSE) for details

## Acknowledgments

- [Jujutsu](https://github.com/martinvonz/jj) - The amazing VCS that makes this possible
- [lazygit](https://github.com/jesseduffield/lazygit) - Inspiration for the TUI design
- [ratatui](https://ratatui.rs/) - The excellent Rust TUI framework
- [Catppuccin](https://github.com/catppuccin/catppuccin) - Beautiful color palette
