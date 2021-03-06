# Marslang
**Stack-based interpreted language implemented in rust**

*This repository has been archived*

Syntax heavily inspired by [porth](https://gitlab.com/tsoding/porth)

## To-Do
- [x] Implement all basic programming uses (if/else, loops, variables, etc)
- [x] Implement string literals
- [x] Better error handling
- [x] Implement import/export system (either simple C-like 'include' or a module system)
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

*Switching pushes the pointer to the stack onto the current working stack*

**Push the pointer to a named stack using the "stack" keyword**

**Push the pointer of the curent stack using the "this" keyword**

**Switch to that stack pointer using the "switch" keyword**

**Close the stack pointer using the "close" keyword**
```
spawn <stack name>
switch

<operations>

stack <stack name>
close
```

**Tools:**
- You can list all existing stacks using the "stack" keyword.
- You can reverse the current stack using stack_rev
- You can get the size of the current stack using stack_size


### String literals

**Create a string literal using double quotes:**
```
"Hello, World!\n"
```

String literals don't get pushed to the stack. Rather, a new stack gets
generated containing the ascii representation of each character in the string literal.
The name of the stack is the first 3 available words in the string, with underscores in place
of whitepsace and one trailing underscore at the end.

```
"Hello, World!\n" switch

@counter 0 def
@size stack_size def
while counter size < do
    print_ascii
    @counter counter 1 + def
end
```

*This whole thing will most likely be put in a procedure/macro in the future standard library*

### Procedures

**Create a procedure using the 'procedure' keyword**

```
procedure my_proc do
    // Do something
end
```

**Add parameters using the 'in' keyword followed by the names of the parameters you would like to add. The parameters are dynamically typed**

```
procedure print in my_str do
    my_str puts
end
```

**Call procedures by using the procedure name. Arguments will be popped off the stack.

```
procedure print in my_str do
    my_str puts
end

"Hello, World!\n" print
```

### Import

**Import files using the 'using' keyword followed by the relative path to the file you'd like to import**

```
using std.mars // contains the 'puts' procedure

"Hello, World!\n" puts
```
