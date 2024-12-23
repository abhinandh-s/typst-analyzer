# Symbols

symbols represent programming constructs or entities within source code.
These constructs are typically identifiers or named elements in the code
that can be defined and referenced.

## A few Types of Symbols ( for example in Rust )

### Variables:

Local variables, global variables, or member variables of a class/struct.
Example: let x = 42; → x is a symbol.

### Functions/Methods:

Named blocks of code that perform a specific task.
Example: fn my_function() {} → my_function is a symbol.

### Modules/Namespaces:

Logical groupings of code.
Example: mod my_module {} → my_module is a symbol.

## How Symbols Are Used in lsp

### Definition Tracking:

LSP allows a client (like a code editor) to locate where a symbol
(e.g., a variable or function) is defined in the code.

### Reference Finding:

LSP servers can find all places in the code where a symbol is referenced.

### Symbol Renaming:

Tools can rename a symbol and automatically update all references in the project.

### Outline View:

Editors provide an outline of all symbols in a file
(e.g., classes, methods) for quick navigation.

### Code Navigation:

Features like "Go to Definition" or "Peek Definition" use symbols
to locate code constructs.

### Code Completion:

Symbols are used to suggest possible completions in the editor.
