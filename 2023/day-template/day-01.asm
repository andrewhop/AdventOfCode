.text
.globl	_add_numbers
_add_numbers:
    // Prologue
    stp x29, x30, [sp, #-16]!  ; store pair x29 and x30 to the stack and decrement the stack pointer
    mov x29, sp

    // Logic, add x1 to x0
    add x0, x0, x1  // Add r0 and r1, store the result in r0

    // Epilogue
    ldp x29, x30, [sp], #16  ; load pair x29 and x30 from the stack and increment the stack pointer
    ret