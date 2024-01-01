# Blinklet

<p align="center">
  <img width="500" height="500" src="graphics/logo.svg">
</p>

A simple interpreted language written in Rust.

## Example

Here is an example for Blinklet script, full example is in [`example/main.k`](./example/main.k):

```bash
# This is a comment.

# Let's start with a hello world.
println 'Hello world'

# Declaring variables
var text 'Some text.'
var number 41
var bool true
var nothing null

# Print out the variables
println 'number: `number`'

# Conditional statements
when true
    println 'It is true! The code executes.'

when false
    println 'It is false! The code will not execute.'

# List
var numbers
    list 1 2 3 4 5 6 7

var length-of-numbers
    list-length numbers

println 'Length of numbers: `length-of-numbers`'

# Table
var person
    table
        var name 'Mr. Krab'
        var age 29

println 'person: `person`'

var person-age
    person
        return age

println 'person age: `person-age`'

# Closure
var say-hello
    closure
        println 'Hi guys!'

say-hello

# Extending table
var better-person
    person
        set name 'Better Mr. Krab'
        set age 30

        var be-good
            closure
                println 'I am definitely a good guy!'

better-person
    be-good
```

## Commands

### Command notations

Command is essentially builtin function for Blinklet. Here demonstrates the notation for explaining command that will be used throughout the wiki.

Here is an example:

```
command <identifier> <...identifiers> {mandatory-value} {...mandatory-values} [optional-value] [...optional-values] @commands
```

| Entity                           | Description                                                                | In example              |
| -------------------------------- | -------------------------------------------------------------------------- | ----------------------- |
| Literal text                     | A literal text in which the coder _follows exactly_ as the notation given. | `command`               |
| Identifier                       | A name given by the coder.                                                 | `<identifier>`          |
| Multiple identifiers             | Names given by the coder.                                                  | `<...identifiers>`      |
| Mandatory value                  | A value in which it must be provided by the coder.                         | `{madatory-value}`      |
| Multiple mandatory values        | 1 or more values must be provided by the coder.                            | `{...mandatory-values}` |
| Optional value                   | A value that doesn't need to be provided by the coder.                     | `[optional-value]`      |
| Multiple optional values         | 0 or more values optionally provided by the coder.                         | `[...optional-values]`  |
| Series of commands (`@commands`) | A series of commands.                                                      | `@commands`             |

### Commands

#### `add`

```
add {left-hand-side} {...right-hand-sides}
```

Returns the summation of `left-hand-side` with `right-hand-sides`

#### `sub`

```
sub {left-hand-side} {...right-hand-sides}
```

Subtract `left-hand-side` with `right-hand-sides` and return the result.

#### `mul`

```
mul {left-hand-side} {...right-hand-sides}
```

Return the multiplication of `left-hand-side` with `right-hand-sides`

#### `div`

```
div {left-hand-side} {...right-hand-sides}
```

Divide `left-hand-side` with `right-hand-sides`, return the result.

#### `eq`

```
eq {left-hand-side} {...right-hand-sides}
```

Check if `left-hand-side` is equal with `right-hand-sides`, return `true` if true.

#### `g`

```
g {left-hand-side} {...right-hand-sides}
```

Check if `left-hand-side` is greater than `right-hand-sides`, return `true` if true.

#### `ge`

```
ge {left-hand-side} {...right-hand-sides}
```

Check if `left-hand-side` is greater or equal to `right-hand-sides`, return `true` if true.

#### `l`

```
l {left-hand-side} {...right-hand-sides}
```

Check if `left-hand-side` is less than `right-hand-sides`, return `true` if true.

#### `le`

```
le {left-hand-side} {...right-hand-sides}
```

Check if `left-hand-side` is less or equal to `right-hand-sides`, return `true` if true.

#### `list`

```
list [...elements]
```

Make a list out of the elements given.

#### `list-get`

```
list-get {list} {index}
```

Return the `index`th element of the `list`.

#### `list-length`

```
list-length {list}
```

Return the length of the `list`.

#### `list-pop`

```
list-pop {list}
```

Remove the last element of the `list` and return it.

#### `list-push`

```
list-push {list} {...elements}
```

Push the `elements` at the back of the `list`.

#### `make-list-iter`

```
make-list-iter {list}
```

Make an iterator for the `list`.

#### `import`

```
import {script-path}
```

Execute the script at `script-path`.
It will return the result of the execution.

#### `when`

```
when {value} @commands
```

Run the `@commands` if `value` is `true`.

#### `while`

```
while <identifier> {value} @commands
```

Assign `value` to variable `identifier`. Repeatedly run the `@commands` until `value` is `false`. It can be controlled by `continue` and `break`.

#### `break`

```
break
```

Break out loops.

#### `continue`

```
continue
```

Continue to the next iteration of the loop.

#### `closure`

```
closure @commands
```

Returns a callable closure.

#### `parameter`

```
parameter <...parameter-names>
```

Retreive parameters and assign it to `parameter-names`.

#### `return`

```
return [value]
```

Stop execution and return `value`.

#### `print`

```
print [...values]
```

Print `values`.

#### `println`

```
println [...values]
```

Print `values` with new line after print.

#### `var`

```
var <identifier> {value}
```

Declare a variable with the name `identifier` and initialize it with the value `value`.

#### `set`

```
set <variable> {value}
```

Set given `variable` with `value`. The `variable` must be declared lexically.

#### `table`

```
table @commands
```

Create a table and execute command on it, return the result of execution.

#### `console`

```
console @commands
```

Execute commands in system console. The values passed into the system commands are evaluated.

#### `duplicate`

```
duplicate {value}
```

Deep clone the value and return the clone.