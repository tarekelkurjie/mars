# Marslang
**Stack-based interpereted language implemented in rust**

## To-Do
- [ ] Implement all basic programming uses (if/else, loops, variables, functions, etc)
- [ ] Better error handling
- [ ] Implement string literals
- [ ] Implement import/export system (either simple C-like 'include' or a module system)
- [ ] Write a compiler
- [ ] Write a standard library


## Documentation
(It's very basic for now, more features to be added in the future)

### The Basics

**Push integers to the stack by simply writing them as such:**

```
1 2 3 4 5
```

**Print value on top of stack by writing a '.'**

```
1 2 3 . 4 5 .
```
(Prints '3' and '5' to the standard output)

*Note: printing also pops the value off the stack*

### Arithmetics

**Use arithmetic operators (+, -, *, /) after pushing two values to the stack**

```
1 2 + .
```

(Prints '3' to the standard output)

*Note: for order-sensitive operations such as - or /, expressions are evaluated in the order they sit on the stack. In the case of the above example, if the operation were subtraction, the program would subtracrt '1' from '2'*

*Note: operations as such pop the evaluated values from the stack*

### Conditionals

**Check equality by pusing two values to the stack, followed by an '=' sign**
```
5 5 = .
```

(Prints '1`, represeting 'true' to the standard output)

*Note: checking for equality also pops the two values checked*

**Check for less-than/greater-than by pusing two values to the stack, followed by a '<' or '>' sign respectively**
```
6 5 < .
```
(Prints '1`, represeting 'true' to the standard output)

*Note: checks values in order that they sit on the stack, in this case checking if '5' is less than '6'*

*Note: checking for lt/gt also pops the two values checked*

### If/Else

**Use if by adding the 'if' keyword after a conditional. End the statement using the 'end' keyword**\

**To use else, add the 'else' keyword instead of 'end' after an if statement. End the statement using the 'end' keyword**

```
5 5 = if
  8 .
else
  9 .
end
```
*Note: 'if' pops top of stack, regardless of whether or not it is a binary boolean*

### Variables NOT YET IMPLEMENTED

*These are all future plans to be implemented in the future*

**Variables are declared with the 'var' keyword, followed by the name of the variable, followed by a set of instructions that will be evaluated similarly to if they were independent of a variable. Variable definitions end with the 'end' keyword**

```
var my_var 1 2 * 6 + end
my_var .
```

*(prints 12 to the standard ouptut)*