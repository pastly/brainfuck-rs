[-][Strip all non-brainfuck bytes and wrap output to 80-column lines.
This program uses the following cells (left to right):

    0: desired line width
    1: stores read-in byte
    2: temp for if/else
    3: temp for if/else

If you want to change the line width, look for all instances of "LINEWIDTH" and
edit the number of '+' chars.

Author: Matt Traudt
This work is released to the public domain "under" the unlicense. See LICENSE.]

LINEWIDTH: Init the cell to the desired output line width
++++++++++++++++++++
++++++++++++++++++++
++++++++++++++++++++
++++++++++++++++++++

>+[
    ,

    Look for plus sign (43)
    --------------------
    --------------------
    ---
    if/else statement; uses two temp cells to the right
    and returns pointer to original position
    >[-]>[-]
    <<[>+>+<<-]>[<+>-]+
    >[<<
        Convert cell back to original byte
        ++++++++++++++++++++
        ++++++++++++++++++++
        +++
    >->[-]]<[<
        Convert cell back to original byte
        ++++++++++++++++++++
        ++++++++++++++++++++
        +++
        .
        Decrement counter
        <->
    >-]<

    Look for minus sign (45)
    --------------------
    --------------------
    -----
    if/else statement; uses two temp cells to the right
    and returns pointer to original position
    >[-]>[-]
    <<[>+>+<<-]>[<+>-]+
    >[<<
        Convert cell back to original byte
        ++++++++++++++++++++
        ++++++++++++++++++++
        +++++
    >->[-]]<[<
        Convert cell back to original byte
        ++++++++++++++++++++
        ++++++++++++++++++++
        +++++
        .
        Decrement counter
        <->
    >-]<

    Look for comma (44)
    --------------------
    --------------------
    ----
    if/else statement; uses two temp cells to the right
    and returns pointer to original position
    >[-]>[-]
    <<[>+>+<<-]>[<+>-]+
    >[<<
        Convert cell back to original byte
        ++++++++++++++++++++
        ++++++++++++++++++++
        ++++
    >->[-]]<[<
        Convert cell back to original byte
        ++++++++++++++++++++
        ++++++++++++++++++++
        ++++
        .
        Decrement counter
        <->
    >-]<

    Look for period (46)
    --------------------
    --------------------
    ------
    if/else statement; uses two temp cells to the right
    and returns pointer to original position
    >[-]>[-]
    <<[>+>+<<-]>[<+>-]+
    >[<<
        Convert cell back to original byte
        ++++++++++++++++++++
        ++++++++++++++++++++
        ++++++
    >->[-]]<[<
        Convert cell back to original byte
        ++++++++++++++++++++
        ++++++++++++++++++++
        ++++++
        .
        Decrement counter
        <->
    >-]<

    Look for left square bracket (91)
    --------------------
    --------------------
    --------------------
    --------------------
    ----------
    -
    if/else statement; uses two temp cells to the right
    and returns pointer to original position
    >[-]>[-]
    <<[>+>+<<-]>[<+>-]+
    >[<<
        Convert cell back to original byte
        ++++++++++++++++++++
        ++++++++++++++++++++
        ++++++++++++++++++++
        ++++++++++++++++++++
        ++++++++++
        +
    >->[-]]<[<
        Convert cell back to original byte
        ++++++++++++++++++++
        ++++++++++++++++++++
        ++++++++++++++++++++
        ++++++++++++++++++++
        ++++++++++
        +
        .
        Decrement counter
        <->
    >-]<

    Look for right square bracket (93)
    --------------------
    --------------------
    --------------------
    --------------------
    ----------
    ---
    if/else statement; uses two temp cells to the right
    and returns pointer to original position
    >[-]>[-]
    <<[>+>+<<-]>[<+>-]+
    >[<<
        Convert cell back to original byte
        ++++++++++++++++++++
        ++++++++++++++++++++
        ++++++++++++++++++++
        ++++++++++++++++++++
        ++++++++++
        +++
    >->[-]]<[<
        Convert cell back to original byte
        ++++++++++++++++++++
        ++++++++++++++++++++
        ++++++++++++++++++++
        ++++++++++++++++++++
        ++++++++++
        +++
        .
        Decrement counter
        <->
    >-]<

    Look for left angle bracket (60)
    --------------------
    --------------------
    --------------------
    if/else statement; uses two temp cells to the right
    and returns pointer to original position
    >[-]>[-]
    <<[>+>+<<-]>[<+>-]+
    >[<<
        Convert cell back to original byte
        ++++++++++++++++++++
        ++++++++++++++++++++
        ++++++++++++++++++++
    >->[-]]<[<
        Convert cell back to original byte
        ++++++++++++++++++++
        ++++++++++++++++++++
        ++++++++++++++++++++
        .
        Decrement counter
        <->
    >-]<

    Look for right angle bracket (62)
    --------------------
    --------------------
    --------------------
    --
    if/else statement; uses two temp cells to the right
    and returns pointer to original position
    >[-]>[-]
    <<[>+>+<<-]>[<+>-]+
    >[<<
        Convert cell back to original byte
        ++++++++++++++++++++
        ++++++++++++++++++++
        ++++++++++++++++++++
        ++
    >->[-]]<[<
        Convert cell back to original byte
        ++++++++++++++++++++
        ++++++++++++++++++++
        ++++++++++++++++++++
        ++
        .
        Decrement counter
        <->
    >-]<
    
    Move to linewidth cell; if we've output enough chars then output newline
    and reset linewidth cell
    <
    if/else statement; uses two temp cells to the right
    and returns pointer to original position
    >>[-]>[-]
    <<<[>>+>+<<<-]>>[<<+>>-]+
    >[<<<
        true side
    >>->[-]]<[<<
        ++++++++++.----------
        LINEWIDTH: reinit the linewidth cell
        ++++++++++++++++++++
        ++++++++++++++++++++
        ++++++++++++++++++++
        ++++++++++++++++++++
    >>-]<<
    Move back to cell for reading
    >
    
]
++++++++++.----------
