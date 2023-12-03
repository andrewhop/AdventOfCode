#define NULL 0
#define NEWLINE #10
#define ASCII_0 #48
#define ASCII_9 #57
#define FIRST_FOUND #1

// Function parameters
#define STRING_PTR x0

// Algorithm variables
#define TEMP x1
#define SUM x2
#define FIRST x3
#define LAST x4
#define FLAGS x5
#define CURRENT_BYTE w6
#define CURRENT x6


.globl	_puzzle1
.p2align   4
_puzzle1:
    // Prologue
    stp x29, x30, [sp, #-16]!  ; store pair x29 and x30 to the stack and decrement the stack pointer
    mov x29, sp

    // Setup
    mov SUM, #0
    mov FLAGS, #0

loop:
    ldrb CURRENT_BYTE, [STRING_PTR]
    cmp CURRENT, NULL
    b.eq end_loop

    cmp CURRENT, NEWLINE
    b.ne search
    // This is a newline run the calculation for the previous line
    mov TEMP, #10
    mul FIRST, FIRST, TEMP
    add SUM, SUM, FIRST
    add SUM, SUM, LAST
    mov FLAGS, #0
    b next

search:
    cmp CURRENT, ASCII_0
    b.lt next

    cmp CURRENT, ASCII_9
    b.gt next

    // This is an ascii number get the actual value
    sub CURRENT, CURRENT, ASCII_0

    and TEMP, FLAGS, FIRST_FOUND
    cmp TEMP, 0
    b.gt found_last
    mov FIRST, CURRENT
    mov LAST, CURRENT
    orr FLAGS, FLAGS, FIRST_FOUND
    b next

found_last:
    mov LAST, CURRENT
    b next

next:
    add STRING_PTR, STRING_PTR, #1
    b loop

end_loop:
    mov x0, SUM

    // Epilogue
    ldp x29, x30, [sp], #16  ; load pair x29 and x30 from the stack and increment the stack pointer
    ret

