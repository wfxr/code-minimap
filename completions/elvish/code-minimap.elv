
use builtin;
use str;

set edit:completion:arg-completer[code-minimap] = [@words]{
    fn spaces [n]{
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand [text desc]{
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'code-minimap'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'code-minimap'= {
            cand -H 'Specify horizontal scale factor'
            cand --horizontal-scale 'Specify horizontal scale factor'
            cand -V 'Specify vertical scale factor'
            cand --vertical-scale 'Specify vertical scale factor'
            cand --padding 'Specify padding width'
            cand --encoding 'Specify input encoding'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand --version 'Print version information'
            cand completion 'Generate shell completion file'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'code-minimap;completion'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'code-minimap;help'= {
        }
    ]
    $completions[$command]
}
