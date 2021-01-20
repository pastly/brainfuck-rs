[-][Two byte adder
The two values are next to each other and the cursor points to the low byte of
the right one.
The "return value" is stored in the left one.
Author: Matt Traudt
This work is released to the public domain "under" the unlicense. See LICENSE.]
-->--
>
>++
Pointing at low byte of right value

Move to high byte of right value
<
While nonzero add it to high byte of left value
[<<+>>-]
Move to low byte of right value
>
Loop till it's empty
[
    Add one to low byte of left value
    <<+

    
    @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
    And here is where the fun begins; this is a fucking "if { code1 } else {
    code2 }" code block and WE ONLY NEED CODE2!!!! We want to increment the
    high byte of the left value if its low byte is now zero; this requires two
    temp cells that we position just to the right of the right input value

    Zero out two temp values temp0 and temp1
    >>>[-] >[-]
    Consider the current value of the low byte of the left value as "x";
    Copy x into the two temp values; into temp0 just so we can put it back in x's
    location after the copy; into temp1 so we can enter a loop on x's value then
    zero out the temp1 cell so the loop is only done once
    This does the move from x's location into temp0 and temp1
    <<<<[>>>+>+<<<<-]
    This move's temp0's value (x) back to x's location then sets temp0 to 1
    >>>[<<<+>>>-]+
    Move to temp1 (copy of x) and if nonzero execute "true side" code
    >[
        (Put "true side" code of "if" statement here but there is none)
        Move to temp0 and make zero
        <-
        Move to temp1 and make zero
        >[-]
        At this point both temps are zero
    ]
    Move to temp0; it is 1 from *before* the previous loop if the loop wasn't
    executed; it is 0 if the loop was executed
    <[
        (Put "false side" code of "if" statement here)
        ("false side" code begins)
        Move from temp0 to high byte of left value
        <<<<
        increment it
        +
        Move back to temp0
        >>>>
        ("false side" code ends)
        Make temp0 zero thus we exit the loop
        -
    ]
    Since we started this craziness positioned at the low byte of the left
    value we now return there from temp0
    <<<
    
    THE FUN IS OVER NOW
    @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

    Move to low byte of right value and decrement now that we've finally added
    one to the low byte of the left value with carry
    >>-
]

Output first value left to right
<<<.>.
