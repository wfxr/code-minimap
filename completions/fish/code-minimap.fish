# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_code_minimap_global_optspecs
	string join \n H/horizontal-scale= V/vertical-scale= padding= encoding= version h/help
end

function __fish_code_minimap_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_code_minimap_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_code_minimap_using_subcommand
	set -l cmd (__fish_code_minimap_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c code-minimap -n "__fish_code_minimap_needs_command" -s H -l horizontal-scale -d 'Specify horizontal scale factor' -r
complete -c code-minimap -n "__fish_code_minimap_needs_command" -s V -l vertical-scale -d 'Specify vertical scale factor' -r
complete -c code-minimap -n "__fish_code_minimap_needs_command" -l padding -d 'Specify padding width' -r
complete -c code-minimap -n "__fish_code_minimap_needs_command" -l encoding -d 'Specify input encoding' -r -f -a "{utf8-lossy\t'',utf8\t''}"
complete -c code-minimap -n "__fish_code_minimap_needs_command" -l version -d 'Print version'
complete -c code-minimap -n "__fish_code_minimap_needs_command" -s h -l help -d 'Print help'
complete -c code-minimap -n "__fish_code_minimap_needs_command" -a "completion" -d 'Generate shell completion file'
complete -c code-minimap -n "__fish_code_minimap_needs_command" -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c code-minimap -n "__fish_code_minimap_using_subcommand completion" -s h -l help -d 'Print help'
complete -c code-minimap -n "__fish_code_minimap_using_subcommand help; and not __fish_seen_subcommand_from completion help" -f -a "completion" -d 'Generate shell completion file'
complete -c code-minimap -n "__fish_code_minimap_using_subcommand help; and not __fish_seen_subcommand_from completion help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
