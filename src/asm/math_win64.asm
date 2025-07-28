; INPUTS:
; xmm0 = radius      (f64)
; xmm1 = angle       (f64, radians)
; ecx  = quality     (u32)
; rdx  = pointer to output buffer (each point = 2x f64)

global gen_arc_vertices
section .text

gen_arc_vertices:
    ; Check if quality < 2
    cmp ecx, 2
    jl .done

    ; Setup loop counters
    xor r8d, r8           ; r8d = i (step index)
    mov r9d, ecx          ; r9d = quality

    ; Compute angle_step = angle / (quality - 1)
    cvtsi2sd xmm2, ecx    ; xmm2 = (double)quality
    mov eax, ecx
    sub eax, 1
    cvtsi2sd xmm3, eax    ; xmm3 = (double)(quality - 1)
    divsd xmm1, xmm3      ; xmm1 = angle_step (reusing xmm1)

.loop:
    ; Compute theta = i * angle_step
    cvtsi2sd xmm4, r8d     ; xmm4 = (double)i
    mulsd xmm4, xmm1       ; xmm4 = theta

    ; Save theta
    movq xmm5, xmm4        ; xmm5 = theta

    ; x = radius * cos(theta)
    call cos
    mulsd xmm0, xmm0       ; reuse radius in xmm0
    mulsd xmm6, xmm0       ; xmm6 = x

    ; z = radius * sin(theta)
    movq xmm4, xmm5        ; reload theta
    call sin
    mulsd xmm7, xmm0       ; xmm7 = z

    ; Store [x, z] into buffer
    mov rax, rdx
    mov r10, r8
    imul r10, r10, 16      ; each vertex = 2 * 8 bytes
    add rax, r10

    movsd [rax], xmm6      ; store x
    movsd [rax + 8], xmm7  ; store z

    ; Increment i
    inc r8d
    cmp r8d, r9d
    jl .loop

.done:
    ret
