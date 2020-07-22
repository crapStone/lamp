_lamp() {
    local i cur prev opts cmds
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    for i in ${COMP_WORDS[@]}
    do
        case "${i}" in
            lamp)
                cmd="lamp"
                ;;
            
            *)
                ;;
        esac
    done

    case "${cmd}" in
        lamp)
            opts=" -g -z -f -l -h -V -s -i -d -t  --get --zero --full --list --help --version --set --increase --decrease --type  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                --set)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -s)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --increase)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -i)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --decrease)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --type)
                    COMPREPLY=($(compgen -W "raw lin log" -- "${cur}"))
                    return 0
                    ;;
                    -t)
                    COMPREPLY=($(compgen -W "raw lin log" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        
    esac
}

complete -F _lamp -o bashdefault -o default lamp
