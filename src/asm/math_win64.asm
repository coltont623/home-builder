; math_win64.asm
; Windows x64 calling convention
; Args: RCX = a, RDX = b
; Returns: RAX

global multiply_and_add_ten

section .text

multiply_and_add_ten:
    mov rax, rcx       ; rax = a
    imul rax, rdx      ; rax = a * b
    add rax, 10        ; rax = a * b + 10
    ret
