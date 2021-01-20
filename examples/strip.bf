[-][Strip all non-brainfuck bytes.
This program uses a total of 3 cells: one to store the most recently read byte,
and two temporary ones for if/else statements.
Author: Matt Traudt
This work is released to the public domain "under" the unlicense. See LICENSE.]
+[
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
    >-]<

]
