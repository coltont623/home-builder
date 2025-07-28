; math_unix.asm
; System V AMD64 ABI (Linux/macOS)
; Args: RDI = a, RSI = b
; Returns: RAX

global multiply_and_add_ten

section .text

multiply_and_add_ten:
    mov rax, rdi       ; rax = a
    imul rax, rsi      ; rax = a * b
    add rax, 10        ; rax = a * b + 10
    ret
