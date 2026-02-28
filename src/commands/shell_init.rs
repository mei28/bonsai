use clap_complete::Shell;

use crate::error::Result;

pub fn exec(shell: Shell) -> Result<()> {
    let script = match shell {
        Shell::Bash => BASH_INIT,
        Shell::Zsh => ZSH_INIT,
        Shell::Fish => FISH_INIT,
        _ => {
            eprintln!("Unsupported shell for shell-init");
            return Ok(());
        }
    };
    println!("{script}");
    Ok(())
}

const BASH_INIT: &str = r#"
bonsai() {
    if [ "$1" = "cd" ]; then
        shift
        local dir
        dir="$(command bonsai cd "$@")"
        if [ $? -eq 0 ] && [ -n "$dir" ]; then
            builtin cd "$dir"
        fi
    else
        command bonsai "$@"
    fi
}

bn() {
    bonsai "$@"
}

_bonsai_completions() {
    local cur prev subcmds wt_cmds
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    subcmds="init add remove list cd status prune rename move lock unlock completion shell-init"
    wt_cmds="cd remove status lock unlock rename move"

    if [ "$COMP_CWORD" -eq 1 ]; then
        COMPREPLY=($(compgen -W "$subcmds" -- "$cur"))
    elif [ "$COMP_CWORD" -eq 2 ]; then
        for cmd in $wt_cmds; do
            if [ "$prev" = "$cmd" ]; then
                local names
                names="$(command bonsai list --names-only 2>/dev/null)"
                COMPREPLY=($(compgen -W "$names" -- "$cur"))
                return
            fi
        done
    fi
}
complete -F _bonsai_completions bonsai bn
"#;

const ZSH_INIT: &str = r#"
bonsai() {
    if [[ "$1" == "cd" ]]; then
        shift
        local dir
        dir="$(command bonsai cd "$@")"
        if [[ $? -eq 0 ]] && [[ -n "$dir" ]]; then
            builtin cd "$dir"
        fi
    else
        command bonsai "$@"
    fi
}

bn() {
    bonsai "$@"
}

_bonsai() {
    local -a subcmds wt_cmds
    subcmds=(init add remove list cd status prune rename move lock unlock completion shell-init)
    wt_cmds=(cd remove status lock unlock rename move)

    if (( CURRENT == 2 )); then
        _describe 'subcommand' subcmds
    elif (( CURRENT == 3 )); then
        if (( ${wt_cmds[(Ie)${words[2]}]} )); then
            local -a names
            names=("${(@f)$(command bonsai list --names-only 2>/dev/null)}")
            _describe 'worktree' names
        fi
    fi
}
compdef _bonsai bonsai bn
"#;

const FISH_INIT: &str = r#"
function bonsai
    if test "$argv[1]" = "cd"
        set -l dir (command bonsai cd $argv[2..])
        if test $status -eq 0; and test -n "$dir"
            builtin cd $dir
        end
    else
        command bonsai $argv
    end
end

function bn
    bonsai $argv
end

set -l __bonsai_subcmds init add remove list cd status prune rename move lock unlock completion shell-init
set -l __bonsai_wt_cmds cd remove status lock unlock rename move

complete -c bonsai -f
complete -c bn -f
complete -c bonsai -n "not __fish_seen_subcommand_from $__bonsai_subcmds" -a "$__bonsai_subcmds"
complete -c bn -n "not __fish_seen_subcommand_from $__bonsai_subcmds" -a "$__bonsai_subcmds"

for cmd in $__bonsai_wt_cmds
    complete -c bonsai -n "__fish_seen_subcommand_from $cmd" -a "(command bonsai list --names-only 2>/dev/null)"
    complete -c bn -n "__fish_seen_subcommand_from $cmd" -a "(command bonsai list --names-only 2>/dev/null)"
end
"#;
