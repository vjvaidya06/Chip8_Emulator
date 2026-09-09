# C8DB
A CHIP 8 debugger inspired by gdb.
<!--
TODO: Make a proper README
For now this is just the help dialogue
-->

Usage:
Below are a list of all possible commands

load/l [FILEPATH]                              Loads the binary file located at the selected path.

print/p [NUM]/all                              Prints a register from 0-15 (or I), or type all to print them all

disassemble/dis [NUM LINES=10] [ADDRESS=PC]    Prints the assembly instructions starting at the selected memory address.

step/s                                         Steps one instruction.

screen/sc                                      Displays the Chip 8 screen.

jump/jmp/j [ADDRESS]                           Jumps to the selected memory address.

break/b [ADDRESS]                              Sets a break point at the selected memory address.

delete/d [ADDRESS]                             Deletes the break point at the selected memory address.

continue/c                                     Runs the program until it encounters a break point, or ends.

legacy/leg                                     Toggle legacy mode.

stack/stk                                      Prints the call stack.

key/k [KEY NUMBER]                             Toggles the key at key number (0-15)

keypad/kp                                      Prints the current state of the keypad

Any memory address can be input as decimal or hexadecimal. If using hex, prefix with 0x.

