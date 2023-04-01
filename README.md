# hedge

wrap text in an ASCII box

## usage

1. `cargo build --release && cd target/release`
2. `hedge --help`

```
Wrap text in ASCII boxes

Usage: -t TYPE -w INT -h INT

Available options:
    -t, --type <TYPE>   Type of box (one of solid, solid_round)
    -w, --width <INT>   Width of box (defaults to auto)
    -h, --height <INT>  Height of box (defaults to auto)
    -h, --help          Prints help information
```

## spec

> note: the current state of the code cannot do all the things this outlines
>
> <i><small>(is spec the right word?)</small></i>

Let's say we want this result:

```
┌──────────────────────┐
│     lorem ipsum      │
│    dolor sit amet    │
└──────────────────────┘
```

Ideally, you'd get this using `hedge --width 24 --padding 4 "lorem ipsum dolor sit amet"`.

#### how the code would do it

1. **Wrap the text.** Using `textwrap` to wrap the text with a width of `width - padding - 2.`
   > _We subtract 2 to account for the edges of the box_

This gives a result of:

```rs
&[
  "lorem ipsum",
  "dolor sit amet"
]
```

2. **Center the text.** For each row, determine how many spaces to pad the row with, and pad it.
   For `lorem ipsum`, this'll look like: - its length is 11, and `width` (specified width) is 24; therefore its "negative space" is 13 - since 13 does not divide into two equal parts, `ceil` 13 / 2 (more spaces than necessary) - pad the left with 7 and the right with 7
3. Go row by row and column by column through `0..=height` and `0..=width`. Add a character to the box, depending on one of these cases (evaluated in the order shown).

   #### Corners

   - If the row is 0 (top), and the column is 0 (left), `┌`
   - If the row is 0 (top), and the column is `width` (right), `┐`
   - If the row is `height` (bottom), and the column is 0 (left), `└`
   - If the row is `height` (bottom), and the column is `width` (right), `┘`

   #### Edges

   - If the row is 0 (top) or `height` (bottom), `─`
   - If the column is 0 (left) or `width` (right), `│`

   #### Text & spaces

   - Get the current character in the current row of the wrapped text
   - Use that character, or a blank space if there isn't one
