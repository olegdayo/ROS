.intel_syntax noprefix
.text
.global _aisum

_aisum:
        add rax, rdi
        add rax, rsi
        ret
