
edit:completion:arg-completer[code-minimap] = [@words]{
    fn spaces [n]{
        repeat $n ' ' | joins ''
    }
    fn cand [text desc]{
        edit:complex-candidate $text &display-suffix=' '(spaces (- 14 (wcswidth $text)))$desc
    }
    command = 'code-minimap'
    for word $words[1:-1] {
        if (has-prefix $word '-') {
            break
        }
        command = $command';'$word
    }
    completions = [
        &'code-minimap'= {
            cand -H 'Specify horizontal scale factor'
            cand --horizontal-scale 'Specify horizontal scale factor'
            cand -V 'Specify vertical scale factor'
            cand --vertical-scale 'Specify vertical scale factor'
            cand --padding 'Specify padding width'
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand --version 'Prints version information'
            cand completion 'Generate shell completion file'
            cand help 'Prints this message or the help of the given subcommand(s)'
        }
        &'code-minimap;completion'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'code-minimap;help'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
    ]
    $completions[$command]
}
