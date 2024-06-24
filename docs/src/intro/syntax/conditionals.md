# Conditionals # 
Conditional statements are useful for controlling the way a program executes

## Syntax ## 
`whamm!` supports 3 different signifiers for conditional statements:
All if/else blocks must begin with and if statement, which is then followed by any number of "elif" statments and either 0 or 1 "else" statment. Finally, the whole chain must be closed with a ";". 

### Formal Syntax ### 
"if" ~ "(" ~ expr ~ ")" ~ "{" ~ statement* ~ "}" ~ (else | elif) ? ~ ";"

Where elif is: "elif" ~ "(" ~ expr ~ ")" ~ "{" ~ statement* ~ "}" ~ (else| elif) ?

And else is: "else" ~  "{" ~ statement* ~ "}"

### Examples of Conditional Statements
```
i32 a = 5; 
if (a == 5) {
    a = 3;
};
```
```
i32 a = 5;
if (a == 4){
    a = 3;
}elif (a == 3){
    a = 2;
}elif (a == 5){
    a = 1;
}; 
```
## Short-circuit evaluation ##

Conditional expressions will only evaluate the branch corresponding to the value of the condition.
In other words, it short-circuits.

