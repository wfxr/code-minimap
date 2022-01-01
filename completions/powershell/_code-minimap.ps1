
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'code-minimap' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'code-minimap'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-')) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'code-minimap' {
            [CompletionResult]::new('-H', 'H', [CompletionResultType]::ParameterName, 'Specify horizontal scale factor')
            [CompletionResult]::new('--horizontal-scale', 'horizontal-scale', [CompletionResultType]::ParameterName, 'Specify horizontal scale factor')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Specify vertical scale factor')
            [CompletionResult]::new('--vertical-scale', 'vertical-scale', [CompletionResultType]::ParameterName, 'Specify vertical scale factor')
            [CompletionResult]::new('--padding', 'padding', [CompletionResultType]::ParameterName, 'Specify padding width')
            [CompletionResult]::new('--encoding', 'encoding', [CompletionResultType]::ParameterName, 'Specify input encoding')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('completion', 'completion', [CompletionResultType]::ParameterValue, 'Generate shell completion file')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'code-minimap;completion' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'code-minimap;help' {
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
