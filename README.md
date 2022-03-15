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

### Stack Manipulation

**Push integers to the stack by simply writing them as such:**

```
1 2 3 4 5
```

**Pop values from the stack by using "pop"**
```
1 2 3 4 5 pop print
```

**Print value on top of stack using "print"**

```
1 2 3 print 4 5 print
```

*Note: printing also pops the value off the stack*

**Duplicate value on top of stack using "dup"**
```
5 dup print print
```
**Swap top and second values using "swap"**
```
5 10 swap print
```

### Arithmetics

**Use arithmetic operators (+, -, *, /) after pushing two values to the stack**

```
1 2 + print
```

*Note: operations as such pop the evaluated values from the stack*

### Conditionals

**Check equality by pusing two values to the stack, followed by an '=' sign**
```
5 5 = print
```

*Note: checking for equality also pops the two values checked*

**Check for less-than/greater-than by pusing two values to the stack, followed by a '<' or '>' sign respectively**
```
6 5 > print
```
*Note: checking for lt/gt also pops the two values checked*

### Control Flow

**If/Else**

```
<condition> if
  <some>
else
  <some>
end
```
*Note: 'if' pops top of stack, regardless of whether or not it is a binary boolean*

### Loops

**While loops**

```
while <condition> do
    <some>
end
```


### Variables

**Define variables using the @ token**

```
@<name> <value> def
<name> print
```

ex:

```
@my_var 42 def
my_var print
```

### Spawnable Stacks

**Generate a new stack using the "spawn" keyword**

**Switch to that stack using the "switch" keyword**

**Close the stack using the "close" keyword**
```
spawn <stack name>
switch <stack name>

<operations>

close <stack name>
```

You can list all existing stacks using the "stack" keyword.