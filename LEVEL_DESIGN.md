# Level Design

To design a level, create a level file calle `<your_filename>.l1t`. An example has been provided in this repo called `test_level.l1t`.

## Structure

Every level must start with the following three lines:

- Level Name
- Author
- Description

They can be left empty if desired.

Example:

```
// level.l1t
Test Level
alex-laycalvert
A test level description
// rest of level...
```

The following lines represent the level grid.

## Grid

The grid of the level must be at least one playable space large and be surrounded by an even box of `I` characters representing walls:

Example:

```
// ... file info
IIIII
I   I
I   I
I   I
IIIII
```

Inside the whitespace, you can place any characters representing the level items that you want.

## Items/Characters

+------------------------------+
| Ascii Character | Level Item |
| --------------- | ---------- |
| `X`             | Player |
| `I`             | Wall |
| `S`             | Statue |
| `R`             | Reverse Statue |
| `Z`             | Zapper |
| `1`             | Laser facing UP turned ON |
| `2`             | Laser facing DOWN turned ON |
| `3`             | Laser facing LEFT turned ON |
| `4`             | Laser facing RIGHT turned ON |
| `5`             | Laser facing UP turned OFF |
| `6`             | Laser facing DOWN turned OFF |
| `7`             | Laser facing LEFT turned OFF |
| `8`             | Laser facing RIGHT turned OFF |
| `B`             | Block |
| `T`             | Toggle Block |
| `b`             | Button |
| `s`             | Switch |
| `/`             | Mirror facing FORWARD |
| `\`             | Mirror facing BACKWARD |
| `?`             | Moveable Mirror facing FORWARD |
| `|`             | Moveable Mirror facing BACKWARD |
+------------------------------+

Any other characters not listed above inside the level grid will be translated into walls.
