# C64 BASIC V2 Command Reference

## Currently Implemented Commands

### File Operations
| Command | Syntax | Description | Example |
|---------|--------|-------------|---------|
| **LOAD** | `LOAD "filename"` | Load a .PRG file into memory | `LOAD "SOCCER"` |
| **SAVE** | `SAVE "filename"` | Save current program to .PRG file | `SAVE "MYPRG"` |

### Program Management
| Command | Syntax | Description | Example |
|---------|--------|-------------|---------|
| **NEW** | `NEW` | Clear program and variables | `NEW` |
| **LIST** | `LIST` | Display program in memory | `LIST` |
| **RUN** | `RUN` | Execute program from beginning | `RUN` |

### Output
| Command | Syntax | Description | Example |
|---------|--------|-------------|---------|
| **PRINT** | `PRINT expression` | Display text or values | `PRINT "HELLO"` |
| **REM** | `REM comment` | Add comment (ignored) | `10 REM THIS IS A COMMENT` |

### Program Control
| Command | Syntax | Description | Example |
|---------|--------|-------------|---------|
| **END** | `END` | Terminate program | `100 END` |

## Token Reference (Hex Values)

### Commands (0x80-0xA2)
```
0x80 END        0x81 FOR        0x82 NEXT       0x83 DATA
0x84 INPUT#     0x85 INPUT      0x86 DIM        0x87 READ
0x88 LET        0x89 GOTO       0x8A RUN        0x8B IF
0x8C RESTORE    0x8D GOSUB      0x8E RETURN     0x8F REM
0x90 STOP       0x91 ON         0x92 WAIT       0x93 LOAD
0x94 SAVE       0x95 VERIFY     0x96 DEF        0x97 POKE
0x98 PRINT#     0x99 PRINT      0x9A CONT       0x9B LIST
0x9C CLR        0x9D CMD        0x9E SYS        0x9F OPEN
0xA0 CLOSE      0xA1 GET        0xA2 NEW
```

### Operators (0xAA-0xB3)
```
0xAA TAB(       0xAB TO         0xAC FN         0xAD SPC(
0xAE THEN       0xAF NOT        0xB0 STEP       0xB1 +
0xB2 -          0xB3 *
```

### Functions (0xB4-0xD1)
```
0xB4 /          0xB5 ^          0xB6 AND        0xB7 OR
0xB8 >          0xB9 =          0xBA <          0xBB SGN
0xBC INT        0xBD ABS        0xBE USR        0xBF FRE
0xC0 POS        0xC1 SQR        0xC2 RND        0xC3 LOG
0xC4 EXP        0xC5 COS        0xC6 SIN        0xC7 TAN
0xC8 ATN        0xC9 PEEK       0xCA LEN        0xCB STR$
0xCC VAL        0xCD ASC        0xCE CHR$       0xCF LEFT$
0xD0 RIGHT$     0xD1 MID$
```

## Commands Not Yet Implemented

### Flow Control
- **GOTO** `line` - Jump to line number
- **GOSUB** `line` - Call subroutine
- **RETURN** - Return from subroutine
- **IF** `condition` **THEN** `statement` - Conditional execution
- **FOR** `var` = `start` **TO** `end` - Begin loop
- **NEXT** `var` - End loop
- **ON** `expr` **GOTO**/**GOSUB** `line1,line2,...` - Computed branch

### Input
- **INPUT** `var1,var2,...` - Read user input
- **INPUT#** `file,var` - Read from file
- **GET** `var` - Get single character
- **READ** `var1,var2,...` - Read from DATA

### Variables & Arrays
- **LET** `var=expr` - Assign value (LET is optional)
- **DIM** `array(size)` - Declare array
- **DATA** `val1,val2,...` - Define data
- **RESTORE** - Reset DATA pointer

### Screen Control
- **CLR** - Clear variables
- **CMD** `channel` - Redirect output
- **SYS** `address` - Call machine language

### File Operations
- **OPEN** `file,device,secondary,name` - Open file
- **CLOSE** `file` - Close file
- **VERIFY** `"filename"` - Verify file

### Other
- **STOP** - Pause program
- **CONT** - Continue after STOP
- **WAIT** `addr,mask,value` - Wait for memory change
- **POKE** `addr,value` - Write to memory
- **DEF FN** `name(var)=expr` - Define function

## String Functions
- **LEFT$**(string, n) - Leftmost n characters
- **RIGHT$**(string, n) - Rightmost n characters
- **MID$**(string, start, length) - Substring
- **CHR$**(code) - Character from ASCII code
- **ASC**(string) - ASCII code of first character
- **LEN**(string) - String length
- **STR$**(number) - Number to string
- **VAL**(string) - String to number

## Math Functions
- **ABS**(x) - Absolute value
- **INT**(x) - Integer part
- **SGN**(x) - Sign (-1, 0, 1)
- **SQR**(x) - Square root
- **SIN**(x) - Sine
- **COS**(x) - Cosine
- **TAN**(x) - Tangent
- **ATN**(x) - Arctangent
- **LOG**(x) - Natural logarithm
- **EXP**(x) - e to the power of x
- **RND**(x) - Random number

## Other Functions
- **PEEK**(addr) - Read memory byte
- **FRE**(x) - Free memory
- **POS**(x) - Cursor position
- **USR**(addr) - Call ML function

## Operators

### Arithmetic
- `+` Addition
- `-` Subtraction
- `*` Multiplication
- `/` Division
- `^` Exponentiation

### Comparison
- `=` Equal
- `<>` Not equal
- `<` Less than
- `>` Greater than
- `<=` Less than or equal
- `>=` Greater than or equal

### Logical
- **AND** - Logical and
- **OR** - Logical or
- **NOT** - Logical not

## Special Characters

- `"` - String delimiter
- `:` - Statement separator
- `;` - PRINT separator (suppress newline/space)
- `,` - PRINT tab separator / list separator
- `?` - Shorthand for PRINT

## Programming Tips

### Line Numbers
- Use increments of 10: `10, 20, 30...`
- Leaves room to insert lines later
- Max line number: 65535

### Variables
- Numeric: `A`, `B`, `X1`, `Y2`, etc. (letter + optional digit)
- String: `A$`, `B$`, `NAME$`, etc. (letter(s) + $)
- Default value: 0 for numbers, "" for strings

### PRINT Formatting
- `PRINT "TEXT"` - Print and newline
- `PRINT "TEXT";` - Print, no newline
- `PRINT A;B;C` - Print with spaces
- `PRINT A,B,C` - Print in columns
- `PRINT TAB(10);"TEXT"` - Position cursor

### Common Patterns

#### Loop
```basic
10 FOR I=1 TO 10
20 PRINT I
30 NEXT I
```

#### Conditional
```basic
10 IF X>5 THEN PRINT "BIG"
20 IF X<=5 THEN 100
```

#### Subroutine
```basic
10 GOSUB 1000
20 END
1000 PRINT "SUBROUTINE"
1010 RETURN
```

#### Input
```basic
10 INPUT "NAME"; N$
20 PRINT "HELLO "; N$
```

## Quick Examples

### Hello World
```basic
10 PRINT "HELLO, WORLD!"
```

### Countdown
```basic
10 FOR I=10 TO 1 STEP -1
20 PRINT I
30 NEXT I
40 PRINT "BLASTOFF!"
```

### Menu
```basic
10 PRINT "1. START"
20 PRINT "2. QUIT"
30 INPUT "CHOICE"; C
40 IF C=1 THEN GOSUB 1000
50 IF C=2 THEN END
60 GOTO 10
1000 PRINT "STARTING..."
1010 RETURN
```

---

**Note**: Items marked as "Not Yet Implemented" are planned for future versions of nano-basic.
