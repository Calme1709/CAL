BR nzp .MAIN

INCLUDE_ONCE "../_shared/utils.asm"
INCLUDE_ONCE "../_shared/stack.asm"
INCLUDE_ONCE "../_shared/strings.asm"
INCLUDE_ONCE "../_shared/stdio.asm"

.MAIN
    CALL .INIT_STACK

    LEA R0 .STRING_1
    CALL .STDOUT_WRITE

    LEA R0 .INPUT_NUMBER
    LDI R1 #8 
    CALL .STDIN_READ

    // Remove trailing newline
    LEA R0 .INPUT_NUMBER
    LDI R1 #10
    CALL .REMOVE_TRAILING_CHAR

    LEA R0 .INPUT_NUMBER
    CALL .ATOI

    MOV R1 R0

    CALL .FIB

    MOV R2 R0

    LEA R0 .STRING_2
    CALL .STDOUT_WRITE

    MOV R0 R1
    CALL .ITOA
    LEA R0 .ITOA_OUT
    CALL .STDOUT_WRITE

    MOV R0 R1
    CALL .GET_NUMBER_SUFFIX
    CALL .STDOUT_WRITE

    LEA R0 .STRING_3
    CALL .STDOUT_WRITE

    MOV R0 R2
    CALL .ITOA
    LEA R0 .ITOA_OUT
    CALL .STDOUT_WRITE

    HLT

// Compute the nth fibonacci digit
// Params:
//   R0: n
// Return:
//   R0: nth fibonacci number
.FIB
    PUSH R1
    PUSH R2
    PUSH R3

    // If n < 2, return n
    SUB R3 R0 #2
    BR n .FIB_END

    // R2 = R0
    MOV R2 R0
    
    // R1 = fib(n - 1)
    SUB R0 R2 #1
    CALL .FIB
    MOV R1 R0

    // R0 = fib(n - 2)
    SUB R0 R2 #2
    CALL .FIB

    // R0 += R1
    ADD R0 R0 R1

.FIB_END
    POP R3
    POP R2
    POP R1

    RET

// Compute the suffix for an ordinal number (i.e. st, nd, rd, or th)
// Params:
//   R0: The number
// Return:
//   R0: An address pointing to the suffix
.GET_NUMBER_SUFFIX
    PUSH R1

    SUB R1 R0 #10
    BR n .GET_NUMBER_SUFFIX_NOT_BETWEEN_10_AND_20

    SUB R1 R0 #20
    BR p .GET_NUMBER_SUFFIX_NOT_BETWEEN_10_AND_20

    LEA R0 .NUMBER_SUFFIX_TH
    BR nzp .GET_NUMBER_SUFFIX_END

.GET_NUMBER_SUFFIX_NOT_BETWEEN_10_AND_20
    LDI R1 #10
    CALL .DIVIDE

    SUB R0 R1 #1
    BR np .GET_NUMBER_SUFFIX_MOD_10_NOT_1

    LEA R0 .NUMBER_SUFFIX_ST
    BR nzp .GET_NUMBER_SUFFIX_END

.GET_NUMBER_SUFFIX_MOD_10_NOT_1
    SUB R0 R1 #2
    BR np .GET_NUMBER_SUFFIX_MOD_10_NOT_2

    LEA R0 .NUMBER_SUFFIX_ND
    BR nzp .GET_NUMBER_SUFFIX_END

.GET_NUMBER_SUFFIX_MOD_10_NOT_2
    SUB R0 R1 #3
    BR np .GET_NUMBER_SUFFIX_MOD_10_NOT_3

    LEA R0 .NUMBER_SUFFIX_RD
    BR nzp .GET_NUMBER_SUFFIX_END

.GET_NUMBER_SUFFIX_MOD_10_NOT_3
    LEA R0 .NUMBER_SUFFIX_TH

.GET_NUMBER_SUFFIX_END
    POP R1
    RET

// Remove trailing char from string if present
// Params:
//   R0: The address of the string
//   R1: The char to remove
.REMOVE_TRAILING_CHAR
    PUSH R2

    LEA R0 .INPUT_NUMBER
    CALL .STRLEN
    DEC R0
    // If strlen = 0, skip removing trailing newline
    BR n .REMOVE_TRAILING_CHAR_END

    // If last char is not newline - skip removing trailing newline
    LEA R2 .INPUT_NUMBER
    ADD R0 R0 R2
    LD R2 R0 #0
    SUB R2 R2 R1
    BR np .REMOVE_TRAILING_CHAR_END

    LDI R2 #0

    ST R0 #0 R2
.REMOVE_TRAILING_CHAR_END
    POP R2

    RET

.STRING_1 ASCII "Input the index for the fibonacci number that you would like computed: "
.STRING_2 ASCII "The "
.STRING_3 ASCII " fibonacci number is: "

.NUMBER_SUFFIX_ST ASCII "st"
.NUMBER_SUFFIX_ND ASCII "nd"
.NUMBER_SUFFIX_RD ASCII "rd"
.NUMBER_SUFFIX_TH ASCII "th"

.INPUT_NUMBER BLK #8