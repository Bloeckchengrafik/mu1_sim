!eq Total 183
%Count

Loop:
    LDA Total1
Add_instr:
    ADD Table
    STO Total
    LDA Add_instr
    ADD One
    STO Add_instr
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
Table:
    DEFW 39
    DEFW 25
    DEFW 4
    DEFW 98
    DEFW 17
