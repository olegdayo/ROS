.att_syntax
.text
.globl aasum
.globl _aasum

aasum:
        add %rdi, %rax
        add %rsi, %rax
        ret

_aasum:
        jmp aasum
