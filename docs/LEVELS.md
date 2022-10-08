# l1t Levels

Levels in `l1t` are defined as files with the `.l1t` extension
and are just normal text files with characters representing
items on the grid.

## Making a Level

To make a new level, add a new `.l1t` file to the `src/levels`
directory in this repo with the name of the level and add
your items as text in that file.

The following is the key for which items correspond to which
characters:

- `I`: `WALL`
- `X`: `PLAYER`
- `/`: `MIRROR_FORWARD`
- `\`: `MIRROR_BACKWARD`
- `K`: `BLOCK`
- `S`: `STATUE`
- `R`: `REVERSE_STATUE`
- `1`: `LASER` `UP` `ON`
- `2`: `LASER` `DOWN` `ON`
- `3`: `LASER` `LEFT` `ON`
- `4`: `LASER` `RIGHT` `ON`
- `5`: `LASER` `UP` `OFF`
- `6`: `LASER` `DOWN` `OFF`
- `7`: `LASER` `LEFT` `OFF`
- `8`: `LASER` `RIGHT` `OFF`

Your level ***MUST*** be surrounded by a wall of `WALL`s as seen
in any of the numbered level files. Your top left wall should be
the first character of the first line and the surrounding walls
should be a continuous square around your grid.

You must also make sure your level has exactly 1 `PLAYER`, at least
one `LASER`, and the same amount or less `STATUE`s as you have `LASER`s.

This is to allow the level to actually be playable and completable.
