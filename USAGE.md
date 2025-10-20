# Usage Guide

## Quick Start

1. Build the project:
   ```bash
   cargo build --release
   ```

2. Run the interpreter:
   ```bash
   cargo run
   ```

3. You'll see the C64 BASIC startup screen:
   ```
    **** COMMODORE 64 BASIC V2 ****

 64K RAM SYSTEM  38911 BASIC BYTES FREE

READY.
_
   ```

## Testing the Interpreter

### Test 1: Direct PRINT Commands

Type these commands directly:
```
PRINT "HELLO WORLD"
PRINT 2+2
```

### Test 2: Create a Simple Program

```
10 PRINT "HELLO FROM C64!"
20 PRINT "LINE TWO"
30 END
LIST
RUN
```

### Test 3: Load a PRG File

First, generate the test files:
```bash
python3 create_prg.py
python3 create_soccer.py
```

Then in the interpreter:
```
LOAD "test"
LIST
RUN
```

Or load the soccer demo:
```
NEW
LOAD "soccer"
LIST
RUN
```

### Test 4: Save Your Own Program

```
NEW
10 PRINT "MY PROGRAM"
20 PRINT "SAVED TO DISK"
30 END
SAVE "mytest"
```

This creates `mytest.prg` in the current directory.

### Test 5: Modify Programs

```
LIST
15 PRINT "INSERTED LINE"
LIST
25
LIST
```

## Keyboard Controls

- **Type characters**: Enter BASIC commands or program lines (automatically converted to uppercase)
- **Enter**: Execute command or add program line
- **Backspace**: Delete last character
- **Ctrl+C**: Exit the interpreter

## C64 BASIC Commands Currently Supported

### Essential Commands
- `LIST` - Display program in memory
- `RUN` - Execute the program
- `NEW` - Clear program and variables
- `LOAD "filename"` - Load .PRG file (adds .prg extension automatically)
- `SAVE "filename"` - Save program to .PRG file
- `PRINT expression` - Print text or evaluate expressions

### Program Commands
- `END` - End program execution
- `REM text` - Comment (ignored during execution)

## PRG File Format Details

The .PRG files created and loaded by this interpreter follow the authentic C64 format:

**Structure:**
```
Byte 0-1:   Load address (0x01 0x08 = $0801)
Then for each line:
  Byte 0-1: Address of next line (little-endian)
  Byte 2-3: Line number (little-endian)
  Byte 4+:  Tokenized BASIC code
  Last:     0x00 (end of line)
End marker: 0x00 0x00
```

**Example (test.prg):**
```
01 08          ; Load at $0801
17 08          ; Next line at $0817
0A 00          ; Line 10
99             ; PRINT token
20 22 48...22  ; " "HELLO, WORLD!""
00             ; End of line
...
00 00          ; End of program
```

## Tokenization

Keywords are automatically converted to single-byte tokens:
- `PRINT` → 0x99
- `END` → 0x80
- `FOR` → 0x81
- `NEXT` → 0x82
- etc.

String literals (in quotes) are preserved as-is.

## Known Limitations (Current Version)

The following are planned but not yet implemented:
- [ ] Math expressions (only simple numbers work)
- [ ] Variables (LET, assignment)
- [ ] Flow control (GOTO, GOSUB, IF/THEN)
- [ ] Loops (FOR/NEXT)
- [ ] INPUT command
- [ ] READ/DATA
- [ ] Arrays (DIM)
- [ ] Functions (SIN, COS, RND, etc.)

## Tips

1. **All input is converted to uppercase** - Just like the real C64!

2. **Line numbers can be any value from 0-65535** - Common practice is to use increments of 10 (10, 20, 30...) to leave room for insertions.

3. **Delete a line by typing just its number:**
   ```
   20
   ```

4. **The screen shows the last 23 lines** - Older output scrolls off the top.

5. **File extensions are automatic** - Both `LOAD "test"` and `LOAD "test.prg"` work.

## Example Programs

### Hello World
```
10 PRINT "HELLO, WORLD!"
20 END
```

### Multiple Lines
```
10 PRINT "*** COMMODORE 64 ***"
20 PRINT ""
30 PRINT "READY FOR BASIC PROGRAMMING"
40 END
```

### Save and Load Workflow
```
NEW
10 PRINT "MY PROGRAM"
20 PRINT "VERSION 1.0"
30 END
SAVE "demo"
NEW
LOAD "demo"
LIST
RUN
```

## Troubleshooting

**"FILE NOT FOUND" error:**
- Check that the .prg file exists in the current directory
- Try adding .prg extension explicitly: `LOAD "test.prg"`

**"SYNTAX ERROR":**
- Check your command syntax
- Remember: Direct commands like LIST, RUN don't need line numbers
- Program lines MUST start with a line number

**Program won't run:**
- Make sure you typed `RUN` (not just the line numbers)
- Use `LIST` to verify your program was entered correctly

**Can't exit:**
- Press `Ctrl+C` to quit
