.intel_syntax noprefix
.text
.global aisum
.global _aisum

aisum:
        add rax, rdi
        add rax, rsi
        ret

_aisum:
        jmp aisum
