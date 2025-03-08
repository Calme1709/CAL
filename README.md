# CAL (Callum's Assembly Language)
This is a custom computer project from the ground up, including:
 - A custom ISA
 - An emulator for the above
 - A custom assembly language
 - An assembler for the above
 - Potentially the computer itself (either as a physical breadboard type implementation or within some simulator software)

## Overall design
The word size of this computer is 16 bits and memory is word addressable.

There are 8 registers R0-R7.

## Instructions

<table>
    <tr>
        <th>Name</th>
        <th>Description</th>
        <th>Syntax</th>
        <th>0</th>
        <th>1</th>
        <th>2</th>
        <th>3</th>
        <th>4</th>
        <th>5</th>
        <th>6</th>
        <th>7</th>
        <th>8</th>
        <th>9</th>
        <th>10</th>
        <th>11</th>
        <th>12</th>
        <th>13</th>
        <th>14</th>
        <th>15</th>
        <th>Pseudocode</th>
    </tr>
    <tr>
        <td>ADD</td>
        <td>Addition</td>
        <td>ADD DR SR0 SR1</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td colspan="3" style="text-align: center">DR</td>
        <td colspan="3" style="text-align: center">SR0</td>
        <td>1</td>
        <td colspan="3" style="text-align: center">SR1</td>
        <td>0</td>
        <td>0</td>
        <td>DR = SR0 + SR1</td>
    </tr>
    <tr>
        <td>ADD</td>
        <td>Immediate addition</td>
        <td>ADD DR SR0 U5</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td colspan="3" style="text-align: center">DR</td>
        <td colspan="3" style="text-align: center">SR0</td>
        <td>0</td>
        <td colspan="5" style="text-align: center">U5</td>
        <td>DR = SR0 + U5</td>
    </tr>
    <tr>
        <td>SUB</td>
        <td>Immediate subtraction</td>
        <td>SUB DR SR0 U5</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>1
        <td colspan="3" style="text-align: center">DR</td>
        <td colspan="3" style="text-align: center">SR0</td>
        <td>0
        <td colspan="5" style="text-align: center">U5</td>
        <td>DR = SR0 - U5</td>
    </tr>
    <tr>
        <td>SUB</td>
        <td>Subtraction</td>
        <td>SUB DR SR0 SR1</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>1
        <td colspan="3" style="text-align: center">DR</td>
        <td colspan="3" style="text-align: center">SR0</td>
        <td>1
        <td colspan="3" style="text-align: center">SR1</td>
        <td>0</td>
        <td>0</td>
        <td>DR = SR0 - SR1</td>
    </tr>
    <tr>
        <td>AND</td>
        <td>Immediate bitwise AND</td>
        <td>AND DR SR0 U5</td>
        <td>0</td>
        <td>0</td>
        <td>1</td>
        <td>0
        <td colspan="3" style="text-align: center">DR</td>
        <td colspan="3" style="text-align: center">SR0</td>
        <td>0
        <td colspan="5" style="text-align: center">U5</td>
        <td>DR = SR0 & U5</td>
    </tr>
    <tr>
        <td>AND</td>
        <td>Bitwise AND</td>
        <td>AND DR SR0 SR1</td>
        <td>0</td>
        <td>0</td>
        <td>1</td>
        <td>0
        <td colspan="3" style="text-align: center">DR</td>
        <td colspan="3" style="text-align: center">SR0</td>
        <td>1
        <td colspan="3" style="text-align: center">SR1</td>
        <td>0</td>
        <td>0</td>
        <td>DR = SR0 & SR1</td>
    </tr>
    <tr>
        <td>NOT</td>
        <td>Bitwise NOT</td>
        <td>NOT DR SR0</td>
        <td>0</td>
        <td>0</td>
        <td>1</td>
        <td>1
        <td colspan="3" style="text-align: center">DR</td>
        <td colspan="3" style="text-align: center">SR0</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>DR = ~SR0</td>
    </tr>
    <tr>
        <td>LSHF</td>
        <td>Left shift</td>
        <td>LSHF DR SR0 U4</td>
        <td>0</td>
        <td>1</td>
        <td>0</td>
        <td>0
        <td colspan="3" style="text-align: center">DR</td>
        <td colspan="3" style="text-align: center">SR0</td>
        <td>0
        <td colspan="4" style="text-align: center">U4</td>
        <td>0</td>
        <td>DR = SR0 << U4</td>
    </tr>
    <tr>
        <td>RSHF</td>
        <td>Right shift</td>
        <td>RSHF DR SR0 U4</td>
        <td>0</td>
        <td>1</td>
        <td>0</td>
        <td>0
        <td colspan="3" style="text-align: center">DR</td>
        <td colspan="3" style="text-align: center">SR0</td>
        <td>1
        <td colspan="4" style="text-align: center">U4</td>
        <td>0</td>
        <td>DR = SR0 >> U4</td>
    </tr>
    <tr>
        <td>LEA</td>
        <td>Load effective address</td>
        <td>LEA I9</td>
        <td>0</td>
        <td>1</td>
        <td>0</td>
        <td>1
        <td colspan="3" style="text-align: center">DR</td>
        <td colspan="9" style="text-align: center">I9</td>
        <td>DR = PC + I9</td>
    </tr>
    <tr>
        <td>LD</td>
        <td>Load memory</td>
        <td>LD DR SR0 I6</td>
        <td>0</td>
        <td>1</td>
        <td>1</td>
        <td>0
        <td colspan="3" style="text-align: center">DR</td>
        <td colspan="3" style="text-align: center">SR0</td>
        <td colspan="6" style="text-align: center">I6</td>
        <td>DR = MEM[SR0 + I6]</td>
    </tr>
    <tr>
        <td>LDI</td>
        <td>Load immediate</td>
        <td>LDI DR U9</td>
        <td>0</td>
        <td>1</td>
        <td>1</td>
        <td>1
        <td colspan="3" style="text-align: center">DR</td>
        <td colspan="9" style="text-align: center">U9</td>
        <td>DR = U9</td>
    </tr>
    <tr>
        <td>ST</td>
        <td>Store in memory</td>
        <td>ST SR0 I6 SR1</td>
        <td>1</td>
        <td>0</td>
        <td>0</td>
        <td>0
        <td colspan="3" style="text-align: center">SR0</td>
        <td colspan="6" style="text-align: center">I6</td>
        <td colspan="3" style="text-align: center">SR1</td>
        <td>MEM[SR0 + I6] = SR1</td>
    </tr>
    <tr>
        <td>BR</td>
        <td>Branch</td>
        <td>BR [nzp] I9</td>
        <td>1</td>
        <td>0</td>
        <td>0</td>
        <td>1</td>
        <td>n</td>
        <td>z</td>
        <td>p
        <td colspan="9" style="text-align: center">I9</td>
        <td>PC = Cond ? (PC + I9) : PC</td>
    </tr>
    <tr>
        <td>CALL</td>
        <td>Call the subroutine at a specified index in the subroutine lookup table</td>
        <td>CALL I12</td>
        <td>1</td>
        <td>0</td>
        <td>1</td>
        <td>0</td>
        <td colspan="12" style="text-align: center">I12</td>
        <td>PC = SLT[I12]</td>
    </tr>
    <tr>
        <td>RET</td>
        <td>Return from subroutine</td>
        <td>RET</td>
        <td>1</td>
        <td>0</td>
        <td>1</td>
        <td>1</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>PC = pop(call stack)</td>
    </tr>
    <tr>
        <td>HLT</td>
        <td>Stop execution</td>
        <td>HLT</td>
        <td>1</td>
        <td>1</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>0</td>
        <td>Stop execution</td>
    </tr>
    <tr>
        <td>SLP</td>
        <td>Sleep</td>
        <td>SLP U12</td>
        <td>1</td>
        <td>1</td>
        <td>0</td>
        <td>1</td>
        <td colspan="12" style="text-align: center">U12</td>
        <td>Sleep for the time specified in ms</td>
    </tr>
</table>

## Subroutine Lookup Table
This system uses a subroutine lookup table for "CALL" execution. The first word of an assembled binary `n` indicates an SLT of length `n` occuping words 1..`n`. The emulator is configured to skip the SLT before starting execution.

## Directives
|Directive|Description|Example|
|--|--|--|
|WORD|Output a single word based on the passed numeric literal|WORD 0xFFFF|
|ASCII|Output a null terminated ascii string|ASCII "Hello, World!"|
|BLK|Reserve a block of memory of length N words|BLK #8|
|INCLUDE|Parse the contents of another file as though it's contents were in place of this directive|INCLUDE "./file.asm"|
|INCLUDE_ONCE|Same as the above INCLUDE directive if we have not yet included this file, otherwise do nothing|INCLUDE_ONCE "./file.asm"|

## Macros
Macros can be defined and invoked as below - the numeric literal is the number of arguments:

```asm
MACRO INC #1
    ADD $0 $0 #1
ENDMACRO

INC R0
```

## I/O
This system employs memory mapped I/O according to the following design.

|Device|Address|Contains|On Load|On Store|
|--|--|--|--|--|
|0xFFFE|STDIN|Char from stdin in bottom half of word or 0x80 if there are no more buffered characters|Load character from stdin and shift buffer||
|0xFFFF|STDOUT|||Push character stored in bottom half of word to stdout|