INCLUDE_ONCE "./utils.asm"
INCLUDE_ONCE "./stack.asm"

.STDOUT_ADDR WORD #65535
.STDIN_ADDR WORD #65534

// Print a null terminated string stored in memory to stdout
// Params:
//   R0: The address of the string
.STDOUT_WRITE
    PUSH R1
    PUSH R6
    LOAD_VALUE_FROM_LABEL R6 .STDOUT_ADDR

.STDOUT_WRITE_LOOP
    // R1 = char
    LD R1 R0 #0
    
    // if R1 == 0, break
    BR z .STDOUT_WRITE_END

    // *stdout = R1
    ST R6 #0 R1

    INC R0
    BR nzp .STDOUT_WRITE_LOOP

.STDOUT_WRITE_END
    POP R6
    POP R1

    RET

// Read a maximum number of bytes into a specified address from STDIN
// Params:
//   R0: The address to read into
//   R1: The maximum number of bytes to read including null byte
.STDIN_READ
    PUSH R2
    PUSH R3
    PUSH R4
    PUSH R5

    // Null byte for comparison
    LDI R3 #128
    LOAD_VALUE_FROM_LABEL R4 .STDIN_ADDR

.STDIN_READ_WAIT_LOOP
    LD R2 R4 #0
    MOV R5 R2
    SUB R2 R2 R3
    BR z .STDIN_READ_WAIT_LOOP
    MOV R2 R5
    BR nzp .STDIN_READ_LOOP_INNER

.STDIN_READ_LOOP
    // Read word from STDIN
    LD R2 R4 #0

.STDIN_READ_LOOP_INNER
    // If we have read the max amount of bytes - break
    DEC R1
    BR nz .STDIN_READ_LOOP_BREAK

    // If end of stream - break
    SUB R5 R2 R3
    BR z .STDIN_READ_LOOP_BREAK

    // else - store at relevant address, increment address, and continue
    ST R0 #0 R2
    INC R0
    BR nzp .STDIN_READ_LOOP

.STDIN_READ_LOOP_BREAK
    // Insert null byte at end
    LDI R2 #0
    ST R0 #0 R2

.STDIN_READ_END
    POP R5
    POP R4
    POP R3
    POP R2

    RET