# Marslang
**Stack-based interpreted language implemented in rust**

Syntax heavily inspired by [porth](https://gitlab.com/tsoding/porth)

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
*(Prints '3' and '5' to the standard output)*

*Note: printing also pops the value off the stack*

**Duplicate value on top of stack by writing "dup"**
```
5 dup . .
```

*(Prints '5' twice to the standard output)*

### Arithmetics

**Use arithmetic operators (+, -, *, /) after pushing two values to the stack**

```
1 2 + .
```

*(Prints '3' to the standard output)*

*Note: operations as such pop the evaluated values from the stack*

### Conditionals

**Check equality by pusing two values to the stack, followed by an '=' sign**
```
5 5 = .
```

*(Prints '1`, represeting 'true' to the standard output)*

*Note: checking for equality also pops the two values checked*

**Check for less-than/greater-than by pusing two values to the stack, followed by a '<' or '>' sign respectively**
```
6 5 > .
```
*(Prints '1`, represeting 'true' to the standard output)*

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

### Loops

**Make a while loop by writing the 'while' keyword, followed by the wanted condition, followed by the 'do' keyword, followed by the code that you want to repeat**

```
while 1 1 = do
    8 .
end
```

*(prints 8 to the standard output repeatedly)*

### Variables NOT YET IMPLEMENTED

*These are all future plans to be implemented in the future*

**Variables are declared with the '@' token, followed by the name of the variable, followed by a set of instructions that will be evaluated similarly to if they were independent of a variable. Variable definitions end with the 'def' keyword which assigns the top stack value to the variable name**

```
@my_var 42 def
my_var .
```

*(prints 12 to the standard ouptut)*