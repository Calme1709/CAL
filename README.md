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
        <td>Call subroutine</td>
        <td>CALL SR0 I9</td>
        <td>1</td>
        <td>0</td>
        <td>1</td>
        <td>0
        <td colspan="3" style="text-align: center">SR0</td>
        <td colspan="9" style="text-align: center">I9</td>
        <td>PC = SR0 + I9 (and push PC onto the call stack)</td>
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
</table>

## Pseudoinstructions
|Name|Description|Syntax|Implementation|Pseudocode|
|-|-|-|-|-|
|NOP|No operation|NOP|ADD R0 R0 0| No operation