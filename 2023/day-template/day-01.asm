.globl	_add_numbers
.p2align   4
_add_numbers:
    // Prologue
    stp x29, x30, [sp, #-16]!  ; store pair x29 and x30 to the stack and decrement the stack pointer
    mov x29, sp

    // Function logic
    add x0, x0, x1  ; Add x0 and x1, store the result in x0

    // Epilogue
    ldp x29, x30, [sp], #16  ; load pair x29 and x30 from the stack and increment the stack pointer
    ret


.globl	_add_array
.p2align   4
_add_array:
    // Prologue
    stp x29, x30, [sp, #-16]!  ; store pair x29 and x30 to the stack and decrement the stack pointer
    mov x29, sp

    // Function parameters
    // x0 is base address
    // x1 is count of items

    // Function logic
    // x0 is running sum
    // x1 is number of items
    // x2 is memory address
    // x3 is the current index
    // x4 is the temp index address or
    // x4 is the value from the current index
    mov x2, x0
    mov x0, #0
    mov x3, #0

    // Check we have at least 1 item
    cmp x1, #0
    b.eq finish

loop:
    // Next index is base + (counter * 4)
    // 4 for 4 bytes in a uint32_t
    lsl x4, x3, #2
    ldr x4, [x2, x4]

    // Update the running total
    add x0, x0, x4

    // Increment and check for loop condition
    add x3, x3, #1
    cmp x1, x3
    b.eq finish
    b loop
finish:
    // Epilogue
    ldp x29, x30, [sp], #16  ; load pair x29 and x30 from the stack and increment the stack pointer
    ret

