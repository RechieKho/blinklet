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