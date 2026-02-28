use clap::{Parser, Subcommand};
use clap_complete::Shell;

#[derive(Parser)]
#[command(name = "bonsai", version, about = "Intuitive Git worktree management")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Show git commands without executing them
    #[arg(long, global = true)]
    pub dry_run: bool,

    /// Show git commands being executed
    #[arg(long, global = true)]
    pub verbose: bool,

    /// Disable colored output
    #[arg(long, global = true)]
    pub no_color: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize bonsai in the current repository
    Init {
        /// Overwrite existing configuration
        #[arg(long)]
        force: bool,
    },

    /// Add a new worktree
    Add {
        /// Branch name
        branch: String,

        /// Create a new branch
        #[arg(short = 'c', long = "create")]
        create: bool,

        /// Base branch for new branch creation (used with -c)
        #[arg(long)]
        base: Option<String>,

        /// Custom worktree path
        #[arg(long)]
        path: Option<String>,

        /// Worktree name
        #[arg(long)]
        name: Option<String>,

        /// Create in detached HEAD state
        #[arg(long)]
        detach: bool,

        /// Skip post_create hooks
        #[arg(long)]
        no_hooks: bool,
    },

    /// Remove a worktree
    Remove {
        /// Worktree name or branch
        worktree: String,

        /// Also delete the branch
        #[arg(long)]
        with_branch: bool,

        /// Force removal even with uncommitted changes
        #[arg(long)]
        force: bool,
    },

    /// List worktrees
    List {
        /// Machine-readable output
        #[arg(long)]
        porcelain: bool,

        /// Show git status for each worktree
        #[arg(long)]
        status: bool,

        /// Only print worktree names
        #[arg(long)]
        names_only: bool,
    },

    /// Print worktree path (use @ for main worktree)
    Cd {
        /// Worktree name or @ for main
        worktree: String,
    },

    /// Show worktree status
    Status {
        /// Specific worktree (all if omitted)
        worktree: Option<String>,
    },

    /// Remove stale or merged worktrees
    Prune {
        /// Remove worktrees merged into base branch
        #[arg(long)]
        merged: Option<Option<String>>,

        /// Remove worktrees with no commits in N days
        #[arg(long)]
        stale: Option<u64>,

        /// Also delete branches
        #[arg(long)]
        with_branch: bool,

        /// Interactively select worktrees to remove
        #[arg(short, long)]
        interactive: bool,

        /// Skip confirmation prompt
        #[arg(short, long)]
        yes: bool,
    },

    /// Rename a worktree branch and move its directory
    Rename {
        /// Current branch name
        old: String,
        /// New branch name
        new: String,
    },

    /// Move a worktree to a new path
    Move {
        /// Worktree name or branch
        worktree: String,
        /// New path
        new_path: String,
    },

    /// Lock a worktree
    Lock {
        /// Worktree name or branch
        worktree: String,
        /// Reason for locking
        #[arg(long)]
        reason: Option<String>,
    },

    /// Unlock a worktree
    Unlock {
        /// Worktree name or branch
        worktree: String,
    },

    /// Generate shell completions
    Completion {
        /// Shell type
        shell: Shell,
    },

    /// Print shell integration script
    ShellInit {
        /// Shell type
        shell: Shell,
    },
}
