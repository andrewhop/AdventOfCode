#define NULL 0
#define newline #10
#define ASCII_0 #48
#define ASCII_9 #57
#define first_flag #1

// Function parameters
#define string_ptr x0

// Algorithm variables
#define temp x1
#define sum x2
#define first x3
#define last x4
#define flags x5
#define current_byte w6
#define current x6


.globl	_puzzle1
.p2align   4
_puzzle1:
    // Prologue
    stp x29, x30, [sp, #-16]!
    mov x29, sp

    // Setup
    mov sum, #0
    mov flags, #0

loop:
    ldrb current_byte, [string_ptr]
    cmp current, NULL
    b.eq end_loop

    cmp current, newline
    b.ne search
    // This is a newline run the calculation for the previous line
    mov temp, #10
    mul first, first, temp
    add sum, sum, first
    add sum, sum, last
    mov flags, #0
    b next

search:
    cmp current, ASCII_0
    b.lt next

    cmp current, ASCII_9
    b.gt next

    // This is an ascii number get the actual value
    sub current, current, ASCII_0

    and temp, flags, first_flag
    cmp temp, 0
    b.gt found_last
    mov first, current
    mov last, current
    orr flags, flags, first_flag
    b next

found_last:
    mov last, current
    b next

next:
    add string_ptr, string_ptr, #1
    b loop

end_loop:
    mov x0, sum

    // Epilogue
    ldp x29, x30, [sp], #16  ; load pair x29 and x30 from the stack and increment the stack pointer
    ret

