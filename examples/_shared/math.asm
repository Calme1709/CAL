// Calculate R0 divided by R1 including the remainder
// Params:
//   R0: Numerator
//   R1: Denominator
// Returns:
//   R0: Result
//   R1: Remainder
.DIVIDE
    // push R2
    ST R7 #0 R2
    SUB R7 R7 #1

    // push R3
    ST R7 #0 R3
    SUB R7 R7 #1

    LDI R2 #0

.DIVIDE_LOOP
    SUB R3 R0 R1
    BR n .DIVIDE_END

    SUB R0 R0 R1
    ADD R2 R2 #1

    BR nzp .DIVIDE_LOOP
    
.DIVIDE_END
    // Set remainder
    ADD R1 R0 #0

    // Set result
    ADD R0 R2 #0

    // pop R3
    ADD R7 R7 #1
    LD R3 R7 #0

    // pop R2
    ADD R7 R7 #1
    LD R2 R7 #0

    RET

// Multiply R0 by R1
// Params:
//   R0: Multiplication operand 1
//   R1: Multiplication operand 2
// Returns:
//   R0: Result
.MULTIPLY
    PUSH R2
    PUSH R3
    PUSH R4

    LDI R2 #0
    LDI R3 #0

.MULTIPLY_LOOP
    SUB R4 R1 R2
    BR nz .MULTIPLY_END

    ADD R3 R3 R0
    INC R2

    BR nzp .MULTIPLY_LOOP

.MULTIPLY_END
    MOV R0 R3 // Return the result in R0
    POP R4
    POP R3
    POP R2    

    RET

// Raises R0 to the power of R1
// Params:
//   R0: The base
//   R1: The exponent
// Returns:
//   R0: The result
.POW
    PUSH R2
    PUSH R3

    // R2 = R1 - 1
    SUB R2 R1 #1

    // If exponent == 0, return 1
    BR zp .POW_NON_ZERO
    LDI R0 #1
    BR nzp .POW_END
    
.POW_NON_ZERO
    MOV R1 R0

.POW_LOOP
    // if R2 = 0, break
    ADD R2 R2 #0
    BR z .POW_END

    // R0 = R0 * R1
    CALL .MULTIPLY

    DEC R2

    BR nzp .POW_LOOP

.POW_END
    POP R3
    POP R2    

    RET
