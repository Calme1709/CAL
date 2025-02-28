// Move a value from one register to another
MACRO MOV #2
    ADD $0 $1 #0
ENDMACRO

// Decrement a register
MACRO DEC #1
    SUB $0 $0 #1
ENDMACRO

// Increment a register
MACRO INC #1
    ADD $0 $0 #1
ENDMACRO

// Load a value from the address specified by a given label
MACRO LOAD_VALUE_FROM_LABEL #2
    LEA $0 $1
    LD $0 $0 #0
ENDMACRO