# pj - Project Jump for xonsh
#
# Installation:
#   Add to ~/.xonshrc:
#     source /path/to/pj.xsh

import shutil
import subprocess
import os
import sys

def _pj(args):
    # Find the pj binary to avoid recursive alias calls
    pj_bin = shutil.which('pj')
    if not pj_bin:
        print("pj: command not found", file=sys.stderr)
        return 1

    # If --help or --version is passed, just run the binary directly
    # We only check the first argument to match bash/fish scripts behavior
    if args and args[0] in ['--help', '--version', '-h', '-V', '--init-config']:
        # Run directly, let it inherit stdin/out/err
        return subprocess.run([pj_bin] + args).returncode

    # Run pj and capture stdout, but let stderr pass through (for interactive pickers like fzf)
    try:
        # text=True requires Python 3.7+, universal_newlines is the older alias
        proc = subprocess.run(
            [pj_bin] + args, 
            stdout=subprocess.PIPE, 
            stderr=None, # Inherit stderr
            universal_newlines=True
        )
    except Exception as e:
        print(f"pj: error running command: {e}", file=sys.stderr)
        return 1

    result = proc.stdout.strip() if proc.stdout else ""
    exit_code = proc.returncode

    # If pj succeeded and returned a path, cd to it
    if exit_code == 0 and result:
        if os.path.isdir(result):
            # Save current directory as previous before jumping
            cwd = os.getcwd()
            if result != cwd:
                # run pj --set-prev "$PWD" silently
                subprocess.run(
                    [pj_bin, '--set-prev', cwd],
                    stdout=subprocess.DEVNULL,
                    stderr=subprocess.DEVNULL
                )
            
            # Change directory
            # In xonsh, os.chdir updates the shell's working directory
            os.chdir(result)
        else:
            # If result is not empty but not a directory, print it
            print(result)

    return exit_code

aliases['pj'] = _pj
