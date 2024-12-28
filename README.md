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
|Name|Description|Syntax|0|1|2|3|4|5|6|7|8|9|10|11|12|13|14|15|Pseudocode
|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|
|ADD|Addition|ADD DR SR0 SR1|0|0|0|0<td colspan="3" style="text-align: center">DR</td><td colspan="3" style="text-align: center">SR0</td>|1<td colspan="3" style="text-align: center">SR1<td>0|0|DR = SR0 + SR1
|ADD|Immediate addition|ADD DR SR0 IMM5|0|0|0|0<td colspan="3" style="text-align: center">DR</td><td colspan="3" style="text-align: center">SR0</td>|0<td colspan="5" style="text-align: center">IMM5</td>|DR = SR0 + IMM5
|SUB|Immediate subtraction|SUB DR SR0 IMM5|0|0|0|1<td colspan="3" style="text-align: center">DR</td><td colspan="3" style="text-align: center">SR0</td>|0<td colspan="5" style="text-align: center">IMM5</td>|DR = SR0 - IMM5
|SUB|Subtraction|SUB DR SR0 SR1|0|0|0|1<td colspan="3" style="text-align: center">DR</td><td colspan="3" style="text-align: center">SR0</td>|1<td colspan="3" style="text-align: center">SR1</td>|0|0|DR = SR0 - SR1
|AND|Immediate bitwise AND|AND DR SR0 IMM5|0|0|1|0<td colspan="3" style="text-align: center">DR</td><td colspan="3" style="text-align: center">SR0</td>|0<td colspan="5" style="text-align: center">IMM5</td>|DR = SR0 & IMM5
|AND|Bitwise AND|AND DR SR0 SR1|0|0|1|0<td colspan="3" style="text-align: center">DR</td><td colspan="3" style="text-align: center">SR0</td>|1<td colspan="3" style="text-align: center">SR1<td>0|0|DR = SR0 & SR1
|NOT|Bitwise NOT|NOT DR SR0|0|0|1|1<td colspan="3" style="text-align: center">DR</td><td colspan="3" style="text-align: center">SR0|0|0|0|0|0|0|DR = ~SR0
|LSHF|Left shift|LSHF DR SR0 IMM4|0|1|0|0<td colspan="3" style="text-align: center">DR</td><td colspan="3" style="text-align: center">SR0</td>|0<td colspan="4" style="text-align: center">IMM4</td>|0|DR = SR0 << IMM4
|RSHF|Right shift|RSHF DR SR0 IMM4|0|1|0|0<td colspan="3" style="text-align: center">DR</td><td colspan="3" style="text-align: center">SR0</td>|1<td colspan="4" style="text-align: center">IMM4</td>|0|DR = SR0 >> IMM4
|LEA|Load effective address|LEA OFFSET9|0|1|0|1<td colspan="3" style="text-align: center">DR</td><td colspan="9" style="text-align: center">OFFSET9</td>|DR = PC + OFFSET9
|LD|Load memory|LD DR SR1 OFFSET6|0|1|1|0<td colspan="3" style="text-align: center">DR</td><td colspan="3" style="text-align: center">SR0</td><td colspan="6" style="text-align: center">OFFSET6|DR = MEM[SR0 + OFFSET6]
|LDI|Load immediate|LDI DR IMM9|0|1|1|1<td colspan="3" style="text-align: center">DR</td><td colspan="9" style="text-align: center">IMM9</td>|DR = IMM9
|ST|Store in memory|ST SR0 SR1 OFFSET6|1|0|0|0<td colspan="3" style="text-align: center">SR0</td><td colspan="6" style="text-align: center">OFFSET6</td><td colspan="3" style="text-align: center">SR1</td>|MEM[SR0 + OFFSET6] = SR1
|BR|Branch|BR [nzp] IMM9|1|0|0|1|n|z|p<td colspan="9" style="text-align: center">OFFSET9</td>|PC = Cond ? (PC + OFFSET9) : PC
|CALL|Call subroutine|CALL SR0 OFFSET9|1|0|1|0<td colspan="3" style="text-align: center">SR0</td><td colspan="9" style="text-align: center">OFFSET9</td>|PC = SR0 + OFFSET9 (and push PC onto the call stack)
|RET|Return from subroutine|RET|1|0|1|1|0|0|0|0|0|0|0|0|0|0|0|0|PC = pop(call stack)
|HLT|Stop execution|HLT|1|1|0|0|0|0|0|0|0|0|0|0|0|0|0|0|

## Pseudoinstructions
|Name|Description|Syntax|Implementation|Pseudocode|
|-|-|-|-|-|
|NOP|No operation|NOP|ADD R0 R0 0| No operation