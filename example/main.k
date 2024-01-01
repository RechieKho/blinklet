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

var 7th-element
    list-get numbers 6

println '7th element of the numbers: `7th-element`'

var numbers-iter
    make-list-iter numbers

while number
    numbers-iter

    println 'number: `number`'

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
    duplicate person

better-person
    set name 'Better Mr. Krab'
    set age 30

    var be-good
        closure
            println 'I am definitely a good guy!'

better-person
    be-good

# Execute console command
var console_output 
    console
        echo 'Hello, this is from a console.'

println 'console output: `console_output`'