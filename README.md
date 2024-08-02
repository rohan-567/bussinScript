# bussinScript
A programming language based off of Gen z slang. Yes it is supposed to be this cringe.

UPDATE 02.08.2024: A formal grammar in the EBNF standard is now available as a pdf document. Please note that this will very likely change as I begin to add more
complex features like if statements and loops.

UPDATE 01.08.2024: The lexer is now up and running, supporting basic arithmetic operations, 
as well as variable assignments which get tokenized. 


<u><b><h2>Basic Concept:</h2></b></u>

Keep in mind that even though the rough concept is outlined, new ideas and concepts will be introduced as I come up with them.

Every new line starts with the keyword "bro". Furthermore, a variable assignment doesn't use
"=" , instead it uses the "be" keyword. So an assignment where x = 5, would look like this:

```
bro x be 5;
```

Variable are mutable by default. If immutability is desired, the "noCap" modifier can be employed to achieve that. Here is an example:

```
bro y noCap be "Hello World";
```

