[bits 16]
[org 0x7c00]
start:
    mov si, msg
    call print
    jmp $
print:
    mov ah, 0x0e
.l: lodsb
    test al, al
    jz .d
    int 0x10
    jmp .l
.d: ret
msg db 'ALONENURI_SOVEREIGN_CORE_ACTIVE', 0
times 510-($-$$) db 0
dw 0xaa55
