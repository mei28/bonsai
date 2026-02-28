use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum BonsaiError {
    #[error("not in a git repository")]
    NotInRepository,

    #[error("bonsai is not initialized (run `bonsai init` first)")]
    NotInitialized,

    #[error("already initialized (use --force to reinitialize)")]
    AlreadyInitialized,

    #[error("worktree '{name}' already exists")]
    WorktreeExists { name: String },

    #[error("worktree '{name}' not found")]
    WorktreeNotFound { name: String },

    #[error("branch '{name}' not found")]
    BranchNotFound { name: String },

    #[error("branch '{name}' already exists")]
    BranchExists { name: String },

    #[error("worktree '{path}' has uncommitted changes (use --force to override)")]
    DirtyWorktree { path: PathBuf },

    #[error("git command failed: {command}\n{stderr}")]
    GitCommandFailed { command: String, stderr: String },

    #[error("config error: {0}")]
    Config(String),

    #[error("hook failed: {0}")]
    HookFailed(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, BonsaiError>;
