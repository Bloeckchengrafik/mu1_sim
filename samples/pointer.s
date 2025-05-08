!eq Total 183

Loop:
    LDR TablePtr
    ADD Total
    STO Total
    LDA TablePtr
    ADD One
    STO TablePtr
    LDA Count
    SUB One
    STO Count
    JGE Loop
    STP

Total:
    DEFW 0
One:
    DEFW 1
Count:
    DEFW 4
TablePtr:
    DEFW Table
Table:
    DEFW 39
    DEFW 25
    DEFW 4
    DEFW 98
    DEFW 17
