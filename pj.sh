#!/usr/bin/env bash
# pj - Project Jump for Bash and Zsh
#
# Installation:
#   Add to ~/.bashrc or ~/.zshrc:
#     source /path/to/pj.sh

pj() {
    # If --help or --version is passed, just run the binary directly
    if [[ "$1" == "--help" ]] || [[ "$1" == "--version" ]] || [[ "$1" == "-h" ]] || [[ "$1" == "-V" ]]; then
        command pj "$@"
        return
    fi

    # If --init-config is passed, run and return
    if [[ "$1" == "--init-config" ]]; then
        command pj "$@"
        return
    fi

    # Run pj and capture output
    local result
    result=$(command pj "$@")
    local exit_code=$?

    # If pj succeeded and returned a path, cd to it
    if [[ $exit_code -eq 0 ]] && [[ -n "$result" ]] && [[ -d "$result" ]]; then
        # Save current directory as previous before jumping
        if [[ "$result" != "$PWD" ]]; then
            command pj --set-prev "$PWD" 2>/dev/null
        fi
        cd "$result" || return 1
    elif [[ $exit_code -eq 0 ]] && [[ -n "$result" ]]; then
        # If result is not empty but not a directory, print it
        echo "$result"
    fi

    return $exit_code
}
