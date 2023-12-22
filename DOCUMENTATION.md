# Hello, World!
Execution starts in the `mn` function.

```
!!f:mn:{
    !con:"Hello, World!";
}
```

# I/O
## Output
There are 2 main commands for displaying an output to stdout. They are:
1. `con`: Console output, newline. Displays the text with a newline at the end.
2. `co`: Console output. Displays the text without newline at end.

Key-word arguments:
1. `p` (sng): Prefix that is added before the message is printed. Does not include '\n'.
2. `s` (sng): Suffix that is added to the end of the message. Is '\n' by default in `con`.
3. `j` (sng): The text that's used to join the positional arguments. It is ' ' (space character) by default in both `con` and `co`.

Thus, the default blueprints of the commands are:
```
!con(p:'',s:'\n',j:' '):positional_args;
!co(p:'',s:'',j:' '):positional_args;
```

## Input
The `ui` command is used to take user input through stdin. Key-word arguments:
1. `c` (typ): The type the input should cast to. Is `sng` by default.
2. `j` (sng): The character used to join the positional arguments. It is ' ' (space character) by default.
3. `e` (bol): Throw an error if the input is empty or any amount of spaces. Is `fls` by default

# Data Types
# Variables
# If-else-elif
# Loops
# Switch
# Try-Except
# Functions
The default return type is a none-type. If your function returns nothing, the return_type parameter may not be specified. However, it *can* be specified as a `\`

Function can take no arguments. In such a case, the brackets enclosing the arguments may be ommitted. However, empty brackets work fine too.

If you want to include a default but not type, you can use `arg1::default1` syntax.

```
!!f:function_name(arg1:type1:default1,arg2:type2:default2):return_type{
    --code
    !r:return_value
}
```

If there are no key-worded arguments, the brackets may be omitted or be empty.
The following syntax is used to call a function:
```
function_name(key-worded arguments):posarg1, posarg2;
```

# Comments
Single-line comments are defined through double minus signs, i.e., `--`
Block comments are defined through triple minus signs, i.e., `---`

# Importing
Libraries can be imported through the `u` command:
```
#u:library_name;
```
Key-word arguments:
1. `a` (sng): Alias for the library. It is the library's name by default.
2. `e` (lis\<sng>): List of the names of the modules to be excluded from the import. Everything else will be imported. By default, it is `-1`, i.e., nothing is excluded.
3. `i` (lis\<sng>): List of the names of the modules to be included in the import. Everything else will be excluded. By default, it is `1`, i.e., everything is included.

The default blueprints of the u command are:
```
!u(a:'library_name',e:-1,i:1):'library_name';
```

# OOP
# Libraries
