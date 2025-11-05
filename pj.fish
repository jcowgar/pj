# pj - Project Jump for Fish shell
#
# Installation:
#   Copy this file to ~/.config/fish/functions/pj.fish
#   Or run: cp pj.fish ~/.config/fish/functions/

function pj --description "Project Jump - jump to project directories"
    # If --help or --version or --init-config is passed, just run the binary
    if contains -- $argv[1] --help --version -h -V --init-config
        command pj $argv
        return
    end

    # Run pj and capture output
    set -l result (command pj $argv)
    set -l exit_code $status

    # If pj succeeded and returned a path, cd to it
    if test $exit_code -eq 0; and test -n "$result"; and test -d "$result"
        # Save current directory as previous before jumping
        if test "$result" != "$PWD"
            command pj --set-prev "$PWD" 2>/dev/null
        end
        cd $result
    else if test $exit_code -eq 0; and test -n "$result"
        # If result is not empty but not a directory, print it
        echo $result
    end

    return $exit_code
end
