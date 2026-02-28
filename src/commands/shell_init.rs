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
"#;
