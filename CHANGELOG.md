# Changelog

## [Unreleased] - 2025-10-20

### Added
- **QUIT/EXIT/BYE commands** - Graceful exit from the interpreter
- Complete expression parser with operator precedence
- All arithmetic operators: +, -, *, /, ^ (exponentiation)
- All comparison operators: =, <>, <, >, <=, >=
- Logical operators: AND, OR, NOT (bitwise like C64)
- Math functions: ABS, INT, SGN, SQR, SIN, COS, TAN, ATN, LOG, EXP, RND
- String functions: LEFT$, RIGHT$, MID$, LEN, ASC, CHR$, VAL, STR$
- LET command (explicit and implicit variable assignment)
- GOTO command (unconditional jumps)
- IF/THEN conditionals (with line numbers or statements)
- GOSUB/RETURN (subroutine calls with return stack)
- FOR/NEXT loops with optional STEP
- STOP command to halt execution
- CLR command to clear variables
- ? as shorthand for PRINT
- Parentheses support in expressions
- String concatenation with +
- Proper operator precedence matching BASIC standard

### Fixed
- **Critical**: FOR/NEXT loops now execute correctly - NEXT jumps to line after FOR, not to FOR itself
- Expression parser handles uninitialized variables (default to 0 or "")
- Proper C64-style boolean values (-1 for true, 0 for false)

### Changed
- **Removed step limit on program execution** - Infinite loops will run forever (use Ctrl+C to stop)
- WAIT command repurposed as QUIT (C64 WAIT not needed in modern context)

### Known Limitations
- Output from RUN appears all at once when program completes (not line-by-line during execution)
  - This is a TUI architecture limitation
  - All output IS generated correctly, just batched for display
- No INPUT command yet (user input during program execution)
- No PRINT formatting (semicolon/comma separators)
- No statement separator (:) for multiple statements per line

## [0.1.0] - Initial Release

### Added
- C64-style TUI with 40x25 character display
- Authentic C64 colors (blue background, light blue text)
- Complete C64 BASIC V2 token table (68 keywords)
- Tokenizer and detokenizer
- PRG file format support (load/save)
- Commands: LIST, RUN, NEW, LOAD, SAVE, PRINT, REM, END
- Line-based program editing
- REPL interface with keyboard input
