# Minky

![logo](graphics/logo.png)

> Nested paranthesis in lisp is too distracting! I should make one without it.
> -- RechieKho

A paranthesis-less indent-based object-oriented lisp.

```
println 'Hello world' # Print hello world!
```

## Concept
If you know lisp, then you already know pretty much how this language's syntax works.
To invoke a function in lisp, you write as bellow:

```lisp
(function arg1 arg2 arg3)
```

In Minky, we don't have the concept of operators (for real).
The above lisp code will translate into Minky as bellow:

```
function arg1 arg2 arg3
```

Let's say `arg1` is a function, and we want to invoke it and pass the returned value as argument.
In lisp, 

```lisp
(function 
    (arg1) 
    arg2 arg3
)
```

While in Minky, we just use indentation.

```
function 
    arg1 
    | arg2 arg3
```

In a sense, each indentation adds a paranthesis arounds the words, 
and if it is leading with `|`, it will not add paranthesis.

## Example
The example is in [`example`](example/) directory.

## Build from source
This is a pure rust project, so just:
```sh
git clone https://github.com/RechieKho/minky # Clone the directory.
cd minky # Enter the project.
cargo run -- example/example.mkx # Build and run the example.
```