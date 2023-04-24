.intel_syntax noprefix
.text
.global aisum

aisum:
        add rax, rdi
        add rax, rsi
        ret
