# Example Programs

## Test the Interpreter

Try these examples to see all the new features in action!

## 1. Direct Commands

Type these directly at the READY prompt:

```basic
PRINT 2+2
PRINT 5*3
PRINT 2^8
PRINT SQR(16)
PRINT "HELLO"+" "+"WORLD"
```

## 2. Variables

```basic
LET A=10
LET B=20
PRINT A+B
A=5
B=3
PRINT A*B
```

## 3. String Variables

```basic
N$="COMMODORE"
M$="64"
PRINT N$+" "+M$
PRINT LEN(N$)
PRINT LEFT$(N$,4)
```

## 4. Math Functions

```basic
10 PRINT "MATH DEMO"
20 PRINT "SIN(1)=";SIN(1)
30 PRINT "COS(0)=";COS(0)
40 PRINT "SQR(25)=";SQR(25)
50 PRINT "ABS(-5)=";ABS(-5)
60 PRINT "INT(3.7)=";INT(3.7)
```

## 5. IF/THEN

```basic
10 A=5
20 B=3
30 IF A>B THEN PRINT "A IS BIGGER"
40 IF A<B THEN PRINT "B IS BIGGER"
50 IF A=5 THEN PRINT "A EQUALS 5"
```

## 6. GOTO Loop

```basic
10 LET I=1
20 PRINT I
30 LET I=I+1
40 IF I<=10 THEN GOTO 20
50 PRINT "DONE!"
```

## 7. FOR/NEXT Loop

```basic
10 FOR I=1 TO 10
20 PRINT I
30 NEXT I
```

## 8. FOR/NEXT with STEP

```basic
10 FOR I=0 TO 20 STEP 2
20 PRINT I
30 NEXT I
```

## 9. Countdown

```basic
10 FOR I=10 TO 1 STEP -1
20 PRINT I
30 NEXT I
40 PRINT "BLASTOFF!"
```

## 10. GOSUB/RETURN (Subroutines)

```basic
10 GOSUB 1000
20 GOSUB 1000
30 GOSUB 1000
40 END
1000 PRINT "HELLO FROM SUBROUTINE"
1010 RETURN
```

## 11. Multiplication Table

```basic
10 PRINT "MULTIPLICATION TABLE"
20 FOR I=1 TO 10
30 PRINT I;" X 5 = ";I*5
40 NEXT I
```

## 12. Fibonacci Sequence

```basic
10 A=0
20 B=1
30 FOR I=1 TO 10
40 PRINT A
50 C=A+B
60 A=B
70 B=C
80 NEXT I
```

## 13. Comparison Operators

```basic
10 A=5
20 B=3
30 IF A=B THEN PRINT "EQUAL"
40 IF A<>B THEN PRINT "NOT EQUAL"
50 IF A>B THEN PRINT "A GREATER"
60 IF A<B THEN PRINT "B GREATER"
70 IF A>=5 THEN PRINT "A>=5"
80 IF B<=3 THEN PRINT "B<=3"
```

## 14. Logical Operators

```basic
10 A=5
20 IF A>0 AND A<10 THEN PRINT "BETWEEN 0 AND 10"
30 IF A<0 OR A>10 THEN PRINT "OUTSIDE RANGE"
40 IF NOT A=0 THEN PRINT "NOT ZERO"
```

## 15. String Functions Demo

```basic
10 A$="HELLO WORLD"
20 PRINT "LENGTH: ";LEN(A$)
30 PRINT "LEFT 5: ";LEFT$(A$,5)
40 PRINT "RIGHT 5: ";RIGHT$(A$,5)
50 PRINT "MID 7,5: ";MID$(A$,7,5)
60 PRINT "ASC 'A': ";ASC("A")
70 PRINT "CHR$ 65: ";CHR$(65)
```

## 16. Number Guessing Game

```basic
10 PRINT "GUESS THE NUMBER (1-10)"
20 N=INT(RND(1)*10)+1
30 PRINT "ENTER YOUR GUESS:"
40 REM INPUT G
50 IF G=N THEN PRINT "CORRECT!"
60 IF G<N THEN PRINT "TOO LOW"
70 IF G>N THEN PRINT "TOO HIGH"
80 IF G<>N THEN GOTO 30
```

## 17. Nested Loops

```basic
10 FOR I=1 TO 5
20 FOR J=1 TO 3
30 PRINT I;",";J
40 NEXT J
50 NEXT I
```

## 18. Calculate Circle Area

```basic
10 PRINT "CIRCLE AREA CALCULATOR"
20 R=5
30 PI=3.14159
40 A=PI*R^2
50 PRINT "RADIUS: ";R
60 PRINT "AREA: ";A
```

## 19. Temperature Conversion

```basic
10 PRINT "CELSIUS TO FAHRENHEIT"
20 FOR C=0 TO 100 STEP 10
30 F=C*9/5+32
40 PRINT C;"C = ";F;"F"
50 NEXT C
```

## 20. Prime Number Check

```basic
10 N=17
20 PRINT "CHECKING IF ";N;" IS PRIME"
30 P=1
40 FOR I=2 TO SQR(N)
50 IF N/I=INT(N/I) THEN P=0
60 NEXT I
70 IF P=1 THEN PRINT "PRIME!"
80 IF P=0 THEN PRINT "NOT PRIME"
```

## Running the Examples

### Method 1: Type Directly
Just type the lines at the READY prompt, then type RUN.

### Method 2: Load from Files
Save examples to .PRG files and load them:
```
LOAD "MYPROGRAM"
LIST
RUN
```

### Method 3: Immediate Execution
For single line tests, just type without a line number:
```
PRINT 2+2
? 5*3
LET X=10:PRINT X
```

## Tips

1. **Use line numbers in increments of 10** - Leaves room to insert lines later
2. **? is shorthand for PRINT** - Faster to type!
3. **Variable names** - Single letter or letter+digit (A, X, A1, Z9)
4. **String variables** - End with $ (A$, NAME$)
5. **Comments** - Use REM to add notes
6. **Test expressions directly** - No line number needed: `PRINT SQR(25)`

## Debugging

If you get an error:
- **SYNTAX ERROR** - Check your syntax, missing operators
- **TYPE MISMATCH** - Mixing strings and numbers
- **DIVISION BY ZERO** - Check your math
- **UNDEF'D STATEMENT** - GOTO/GOSUB to non-existent line
- **ILLEGAL QUANTITY** - Bad argument to function (like SQR(-1))

## What Works Now

✅ All arithmetic operators (+, -, *, /, ^)
✅ All comparison operators (=, <>, <, >, <=, >=)
✅ Logical operators (AND, OR, NOT)
✅ Math functions (SIN, COS, TAN, ATN, SQR, ABS, INT, SGN, LOG, EXP)
✅ String functions (LEFT$, RIGHT$, MID$, LEN, ASC, CHR$, VAL, STR$)
✅ Variables (numeric and string)
✅ IF/THEN conditionals
✅ FOR/NEXT loops with STEP
✅ GOTO/GOSUB/RETURN
✅ LET assignments
✅ Parentheses in expressions
✅ String concatenation with +

## What's Still TODO

❌ INPUT command
❌ READ/DATA
❌ DIM arrays
❌ ON...GOTO/GOSUB
❌ PRINT formatting (;, ,)
❌ Multiple statements per line (:)
❌ File I/O (OPEN, CLOSE, etc.)
❌ POKE/PEEK
❌ More...

---

**Enjoy coding in BASIC!**
