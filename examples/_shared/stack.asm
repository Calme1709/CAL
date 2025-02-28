INCLUDE_ONCE "./utils.asm"

.STACK_ORIGIN WORD #16383

// Push a value onto the stack
MACRO PUSH #1
    ST R7 #0 $0
    DEC R7
ENDMACRO

// Pop a value from the stack
MACRO POP #1
    INC R7
    LD $0 R7 #0
ENDMACRO

// Initialize the stack pointer
.INIT_STACK
    LOAD_VALUE_FROM_LABEL R7 .STACK_ORIGIN
    RET
