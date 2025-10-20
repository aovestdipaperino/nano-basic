# Nano-BASIC: Commodore 64 BASIC Interpreter

A Commodore 64 BASIC V2 interpreter with an authentic 40x25 character TUI that resembles the original C64 screen.

## Features

- **Authentic C64 TUI**: 40x25 character display with classic blue background and light blue text
- **Tokenized .PRG Format**: Read and write C64 BASIC programs in native tokenized format
- **C64 BASIC V2 Commands**: Support for classic commands like LOAD, SAVE, LIST, RUN, NEW
- **REPL Interface**: Interactive command-line experience like the real C64

## Building

```bash
cargo build --release
```

## Running

```bash
cargo run
```

Type `QUIT` (or `EXIT` or `BYE`) to exit, or press `Ctrl+C`.

## Usage

### Immediate Commands

These commands execute immediately:

- `LIST` - Display the current program
- `RUN` - Execute the program in memory
- `NEW` - Clear program memory
- `CLR` - Clear variables only
- `LOAD "filename"` - Load a .PRG file (e.g., `LOAD "soccer"` loads `soccer.prg`)
- `SAVE "filename"` - Save current program as .PRG file
- `PRINT expression` - Print text or evaluate expressions (or use `?`)
- `QUIT` - Exit the interpreter (also `EXIT` or `BYE`)

### Program Lines

Type a line number followed by code to add it to the program:

```
10 PRINT "HELLO, WORLD!"
20 END
```

Type just a line number to delete that line:

```
10
```

## PRG File Format

.PRG files use the Commodore 64 tokenized BASIC format:

- First 2 bytes: Load address (0x0801 for BASIC programs)
- Each line: [next_addr_lo, next_addr_hi, line_num_lo, line_num_hi, tokens..., 0x00]
- End of program: [0x00, 0x00]

## Example Session

```
READY.
10 PRINT "HELLO FROM C64!"
20 PRINT "NANO-BASIC ROCKS"
30 END
LIST
10 PRINT "HELLO FROM C64!"
20 PRINT "NANO-BASIC ROCKS"
30 END
READY.
RUN
HELLO FROM C64!
NANO-BASIC ROCKS
READY.
SAVE "TEST"
SAVING TEST
READY.
NEW
LOAD "SOCCER"
LOADING SOCCER
READY.
LIST
10 PRINT "*** SOCCER GAME ***"
20 PRINT
30 PRINT "GOAL!"
40 PRINT "SCORE: 3-2"
50 END
READY.
RUN
*** SOCCER GAME ***

GOAL!
SCORE: 3-2
READY.
```

## Supported C64 BASIC V2 Tokens

Currently implemented:
- Commands: END, FOR, NEXT, DATA, INPUT, DIM, READ, LET, GOTO, RUN, IF, RESTORE, GOSUB, RETURN, REM, STOP, ON, WAIT, LOAD, SAVE, VERIFY, DEF, POKE, PRINT, CONT, LIST, CLR, CMD, SYS, OPEN, CLOSE, GET, NEW
- Operators: TAB(, TO, FN, SPC(, THEN, NOT, STEP, +, -, *, /, ^, AND, OR, >, =, <
- Functions: SGN, INT, ABS, USR, FRE, POS, SQR, RND, LOG, EXP, COS, SIN, TAN, ATN, PEEK, LEN, STR$, VAL, ASC, CHR$, LEFT$, RIGHT$, MID$

## Test Files

Generate test .PRG files:

```bash
python3 create_prg.py      # Creates test.prg
python3 create_soccer.py   # Creates soccer.prg
```

## TODO

- [ ] Full BASIC statement execution (LET, IF, GOTO, GOSUB, FOR/NEXT)
- [ ] Expression evaluator with math operations
- [ ] Variable storage (numeric and string)
- [ ] Flow control (GOTO, GOSUB, RETURN, FOR, NEXT)
- [ ] More BASIC commands (INPUT, READ/DATA, etc.)
- [ ] Keyboard shortcuts (cursor movement, history)

## Technical Details

- Built with Rust
- TUI: ratatui + crossterm
- Authentic C64 colors (RGB values matching the original palette)
- Token-based parsing matching C64 BASIC V2 specification

## License

MIT
