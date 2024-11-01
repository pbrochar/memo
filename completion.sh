#!/bin/bash

_memo_completion() {
    local cur prev opts
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    opts="$(memo _complete "$cur")"

    case "${prev}" in
        "get" | "rm" | "set" | "cp")
            opts="$(memo _complete "$cur")"
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        *)
            return 0
            ;;
    esac
}

complete -F _memo_completion memo