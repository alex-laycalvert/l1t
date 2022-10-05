# Contributing to l1t

First, I'd like to thank you for taking the time to
contribute to `l1t`. I'm not perfect and make mistakes
and I appreciate any that I can get.

## How to Contribute

You can follow these guidelines to contribute.

### Running l1t for Development

To contribute to `l1t`, you can start by forking an
instance of the repo.

- Fork this repository

- Clone your fork

```bash
git clone https://github.com/<your_fork>/l1t
cd ./l1t
```
- Build the project for development:

```bash
make dev
```

- Run and test the development instance:

```bash
./build/l1t.dev
```

The developmenet instance as all warnings turned on. PR's
that compile with warnings will be asked to be modified
so that no warnings/errors are generated. If, for some reason,
the warnings ***must*** be present, the you most likely need
to attempt the feature or fix you are trying to make a
different way.

### Creating a Level

Here's how you can propose a new level to `l1t`:

- Add this code snippet to the bottom of `src/levels.h`:

```c
void init_user_level_YOURUSERNAME(const int rows, const int columns, Node **grid);
```

- Add this code snippet to the bottom of `src/levels.c`:

```c
void init_user_level_YOURUSERNAME(const int rows, const int columns, Node **grid) {
    init_walls(rows, columns, grid);
    
    // place your items here...
}
```

Replace `YOURUSERNAME` in both snippets with your GitHub username.

To add items to a level, you will need to call the `place_item()` function
with the desired item and grid location.

The `place_item()` function takes the following 6 arguments in order:

- `item`: The type of item you are placing (see [Node Types](#NodeTypes) below) 
- `dir`: The direction the item is pointing (only applies to `LASER`s, see [Directions](#Directions) below)
- `on`: A boolean representing whether the item is on or not (only applies to `LASER`s, set to false for others)
- `row`: The row the item is on (must be greater than `0` and less than `TERMINAL_ROWS`)
- `column`: The column the item is on (must be greater than `0` and less than `TERMINAL_COLUMNS`)
- `grid`: The grid the game is played on, ALWAYS set this to `grid`

#### Node Types

These are the following types of nodes you can add to your grid:

- `PLAYER`
- `WALL` 
  - Can also be used as an immoveable block.
- `MIRROR_FORWARD`
  - Not implemented yet
- `MIRROR_BACKWARD`
  - Not implemented yet
- `BLOCK`
  - A block that can be moved by the player. Will get stuck if moved into
    a corner or against a wall.
- `STATUE`
- `TOGGLE_BLOCK`
  - Not implemented yet
- `BUTTON`
  - Not implemented yet
- `SWITCH`
  - Not implemented yet
- `LASER`
- `EMPTY`
  - You do not need to set this item manually, every grid space is empty
    by default after calling `init_walls()`

#### Directions

These are the following directions that a node can face:

- `UP`
- `DOWN`
- `LEFT`
- `RIGHT`

I recommend setting the `dir` of any item that does not need a direction to `UP`.

#### Adding the player

You can start by placing the `PLAYER` item to your level. This will let you play
the level.

```c
place_item(PLAYER, UP, false, <DESIRED_ROW>, <DESIRED_COLUMN>, grid);
```

Replace `<DESIRED_ROW>` and `<DESIRED_COLUMN>` with the row and column you
want the player to start at. This row 

Adding more than one `PLAYER` to the grid will result in only one of them
being playeable. The other player(s) will be treated as a `WALL` item.

#### Adding other items

You can use the same function call for any other `NodeType` you want to add. Make
sure to set the appropriate `dir` and `on` values for all items you add.

### Submitting a PR

Once you have made your changes and everything is good,
open a PR and select the fork you have modified to merge
into `main`. Make sure to include the required information
from the PR template.
