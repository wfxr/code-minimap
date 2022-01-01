complete -c code-minimap -n "__fish_use_subcommand" -s H -l horizontal-scale -d 'Specify horizontal scale factor' -r
complete -c code-minimap -n "__fish_use_subcommand" -s V -l vertical-scale -d 'Specify vertical scale factor' -r
complete -c code-minimap -n "__fish_use_subcommand" -l padding -d 'Specify padding width' -r
complete -c code-minimap -n "__fish_use_subcommand" -l encoding -d 'Specify input encoding' -r -f -a "{UTF8Lossy	,UTF8	}"
complete -c code-minimap -n "__fish_use_subcommand" -s h -l help -d 'Print help information'
complete -c code-minimap -n "__fish_use_subcommand" -l version -d 'Print version information'
complete -c code-minimap -n "__fish_use_subcommand" -f -a "completion" -d 'Generate shell completion file'
complete -c code-minimap -n "__fish_use_subcommand" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c code-minimap -n "__fish_seen_subcommand_from completion" -s h -l help -d 'Print help information'
