_code-minimap() {
    local i cur prev opts cmds
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    for i in ${COMP_WORDS[@]}
    do
        case "${i}" in
            code-minimap)
                cmd="code-minimap"
                ;;
            
            completion)
                cmd+="__completion"
                ;;
            help)
                cmd+="__help"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        code-minimap)
            opts=" -h -H -V  --help --version --horizontal-scale --vertical-scale  <FILE>  completion help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                --horizontal-scale)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -H)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --vertical-scale)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -V)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        
        code__minimap__completion)
            opts=" -h -V  --help --version  <shell> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        code__minimap__help)
            opts=" -h -V  --help --version  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
    esac
}

complete -F _code-minimap -o bashdefault -o default code-minimap
