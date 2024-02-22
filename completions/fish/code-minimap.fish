complete -c code-minimap -n "__fish_use_subcommand" -s H -l horizontal-scale -d 'Specify horizontal scale factor' -r
complete -c code-minimap -n "__fish_use_subcommand" -s V -l vertical-scale -d 'Specify vertical scale factor' -r
complete -c code-minimap -n "__fish_use_subcommand" -l padding -d 'Specify padding width' -r
complete -c code-minimap -n "__fish_use_subcommand" -l encoding -d 'Specify input encoding' -r -f -a "{utf8-lossy	'',utf8	''}"
complete -c code-minimap -n "__fish_use_subcommand" -l version -d 'Print version'
complete -c code-minimap -n "__fish_use_subcommand" -s h -l help -d 'Print help'
complete -c code-minimap -n "__fish_use_subcommand" -f -a "completion" -d 'Generate shell completion file'
complete -c code-minimap -n "__fish_use_subcommand" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c code-minimap -n "__fish_seen_subcommand_from completion" -s h -l help -d 'Print help'
complete -c code-minimap -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from completion; and not __fish_seen_subcommand_from help" -f -a "completion" -d 'Generate shell completion file'
complete -c code-minimap -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from completion; and not __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
