INCLUDE_ONCE "./stack.asm"
INCLUDE_ONCE "./math.asm"
INCLUDE_ONCE "./utils.asm"

.ITOA_OUT BLK #8

// Convert an integer into a null terminated ASCII string
// Params:
//   R0: The integer to convert
// Returns:
//   The converted string in the ITOA_OUT block
.ITOA
    PUSH R1
    PUSH R2
    PUSH R3
    PUSH R4
    PUSH R5
    PUSH R6

    MOV R4 R0
    LDI R3 #0
    LEA R2 .ITOA_OUT
    LDI R5 #48     // ASCII '0'

.ITOA_LEN_LOOP
    INC R3
    LDI R0 #10
    MOV R1 R3
    CALL .POW

    SUB R0 R4 R0
    BR zp .ITOA_LEN_LOOP

.ITOA_LOOP
    DEC R3
    BR n .ITOA_END

    // R6 = R4 % 10^(R3 + 1)
    ADD R1 R3 #1
    LDI R0 #10
    CALL .POW
    MOV R1 R0
    MOV R0 R4
    CALL .DIVIDE
    MOV R6 R1

    // R6 = R6 / (10^R3)
    MOV R1 R3
    LDI R0 #10
    CALL .POW
    MOV R1 R0
    MOV R0 R6
    CALL .DIVIDE

    ADD R0 R0 R5
    ST R2 #0 R0
    INC R2

    BR nzp .ITOA_LOOP

.ITOA_END
    // null-terminate
    LDI R0 #0
    ST R2 #0 R0

    POP R6
    POP R5
    POP R4
    POP R3
    POP R2
    POP R1

    RET

// Convert an ASCII string into an integer. Returns 0 if non-numeric characters are encountered
// Params:
//   R0: The address of the string
// Returns:
//   R0: The converted integer
.ATOI
    PUSH R1
    PUSH R2
    PUSH R3
    PUSH R4
    PUSH R5

    MOV R2 R0

    CALL .STRLEN

    MOV R3 R0

    LDI R5 #0

.ATOI_LOOP
    DEC R3
    BR n .ATOI_END

    LDI R0 #10
    MOV R1 R3
    CALL .POW

    LD R4 R2 #0
    INC R2

    // Ascii '9'
    LDI R1 #57
    SUB R1 R4 R1
    BR p .ATOI_INVALID_CHAR

    // Ascii '0'
    LDI R1 #48
    SUB R1 R4 R1
    BR n .ATOI_INVALID_CHAR

    CALL .MULTIPLY

    ADD R5 R5 R0
    BR nzp .ATOI_LOOP

.ATOI_INVALID_CHAR
    LDI R5 #0

.ATOI_END
    MOV R0 R5

    POP R5
    POP R4
    POP R3
    POP R2
    POP R1

    RET

// Get the length of a null terminated string.
// Params:
//   R0: The address of the string
// Returns:
//   R0: The length of the string
.STRLEN
    PUSH R1
    PUSH R2

    MOV R1 R0

.STRLEN_LOOP
    LD R2 R0 #0
    BR z .STRLEN_END
    INC R0
    BR nzp .STRLEN_LOOP

.STRLEN_END
    SUB R0 R0 R1

    POP R2
    POP R1

    RET