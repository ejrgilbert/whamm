# Conditionals # 
Conditional statements are useful for controlling the way a program executes

## Syntax ## 
`Whamm` supports 3 different signifiers for conditional statements:
All `if`/`else` blocks must begin with an `if` statement, which is then followed by any number of `elif` statments and either 0 or 1 `else` statments. Finally, the whole chain must be closed with a `;`. 

### Formal Syntax ### 
"if" ~ "(" ~ expr ~ ")" ~ "{" ~ statement* ~ "}" ~ (else | elif) ? ~ ";"

Where elif is: "elif" ~ "(" ~ expr ~ ")" ~ "{" ~ statement* ~ "}" ~ (else| elif) ?

And else is: "else" ~  "{" ~ statement* ~ "}"

### Examples of Conditional Statements
```
var a: i32 = 5; 
if (a == 5) {
    a = 3;
};
```
```
var a: i32 = 5; 
if (a == 4){
    a = 3;
}elif (a == 3){
    a = 2;
}elif (a == 5){
    a = 1;
}; 
```
```
//This is an example in a function that returns a bool
my_fn (param: i32) -> bool {
    return (param/10 == 0);    
}
var a: i32 = 5; 
if(my_fn(a)) {
    a = 3;
}else{
    a = 2;
};
```
## Short-circuit evaluation ##

Conditional expressions will only evaluate the branch corresponding to the value of the condition.
In other words, it short-circuits.
