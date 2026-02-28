# bonsai

Intuitive Git worktree management CLI.

Bonsai organizes your Git worktrees under a single directory, provides hooks for automating setup, and offers a short alias (`bn`) for quick access.

## Features

- Worktrees stored in `.bonsai/` by default, keeping your repo tidy
- Post-create hooks: copy files, create symlinks, or run commands automatically
- Shell integration for `bonsai cd` (actually changes your directory)
- Colored, table-formatted output with `bonsai list`
- Prune merged or stale worktrees in bulk
- Lock/unlock worktrees to prevent accidental removal
- Rename branches and move worktree directories
- Shell completions for bash, zsh, and fish
- Short alias: `bn` works identically to `bonsai`

## Installation

### cargo

```sh
cargo install bonsai-wt
```

### Nix (flake)

```nix
# flake.nix
{
  inputs.bonsai.url = "github:mei28/bonsai";

  outputs = { bonsai, ... }: {
    # Add to your packages or environment
    environment.systemPackages = [ bonsai.packages.${system}.default ];
  };
}
```

Or try it directly:

```sh
nix run github:mei28/bonsai -- --help
```

### Build from source

```sh
git clone https://github.com/mei28/bonsai.git
cd bonsai
cargo install --path .
```

## Quick Start

```sh
# Initialize bonsai in your repository
bonsai init

# Create a worktree for a new branch
bonsai add -c feature/login

# List all worktrees
bonsai list

# Switch to a worktree (requires shell integration)
bonsai cd feature/login

# Remove a worktree when done
bonsai remove feature/login --with-branch
```

## Commands

| Command | Description |
|---------|-------------|
| `init` | Initialize bonsai in the current repository |
| `add <branch>` | Add a new worktree (`-c` to create branch, `--base` to set base) |
| `remove <worktree>` | Remove a worktree (`--with-branch` to delete branch, `--force`) |
| `list` | List worktrees (`--status`, `--porcelain`, `--names-only`) |
| `cd <worktree>` | Print worktree path (use `@` for main worktree) |
| `status [worktree]` | Show git status for worktrees |
| `prune` | Remove stale/merged worktrees (`--merged`, `--stale <days>`) |
| `rename <old> <new>` | Rename a worktree branch and move its directory |
| `move <worktree> <path>` | Move a worktree to a new path |
| `lock <worktree>` | Lock a worktree (`--reason`) |
| `unlock <worktree>` | Unlock a worktree |
| `completion <shell>` | Generate shell completions |
| `shell-init <shell>` | Print shell integration script |

Global flags: `--dry-run`, `--verbose`, `--no-color`

## Configuration

Bonsai stores its configuration in `.bonsai.toml` at the repository root:

```toml
version = "1"

[defaults]
worktree_dir = ".bonsai"

[[hooks.post_create]]
type = "copy"
from = ".env"
to = ".env"

[[hooks.post_create]]
type = "symlink"
from = "node_modules"
to = "node_modules"

[[hooks.post_create]]
type = "command"
command = "npm install"
```

Hook types:
- `copy` — copies a file from the main worktree
- `symlink` — creates a symlink to a file in the main worktree
- `command` — runs a shell command in the new worktree (supports `env` table)

## Shell Integration

Shell integration enables `bonsai cd` to change your working directory. Add one of the following to your shell config:

### bash (`~/.bashrc`)

```sh
eval "$(bonsai shell-init bash)"
```

### zsh (`~/.zshrc`)

```sh
eval "$(bonsai shell-init zsh)"
```

### fish (`~/.config/fish/config.fish`)

```fish
bonsai shell-init fish | source
```

This also defines `bn` as a shell alias for `bonsai`.

## License

[MIT](LICENSE)
