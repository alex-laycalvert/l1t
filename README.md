# l1t

A terminal game where you move lasers around a grid to light up statues.
Light up all the statues in a level to complete the level.

## Progress

***NOTE***: This project is still in development and is not yet ready for
full gameplay. Thank you for being patient with me.

All current "gameplay" is done in `Level 0` right now which is the development
playground for level making. Any items in that level are purely for testing
and there is no way, right now, to "win" `Level 0`.

- [ ] Colors
- [x] Lasers
- [x] Moveable Blocks
- [x] Mirrors
- [ ] Statues
- [ ] Menu for picking levels
- [ ] A concept of "winning"
- [ ] Keeping track of completed levels
- [ ] Switches
- [ ] Buttons
- [ ] Toggle Blocks

- [ ] An official `Level 1`

## Installation and Running

Follow the steps below to install `l1t`:

- Close this repository

```bash
git clone https://github.com/alex-laycalvert/l1t
cd ./l1t
```

- Build the project:

```bash
make
```

- Run the game:

```bash
./build/l1t
```

- For development (optional):

```bash
make dev
./build/l1t.dev
```

## Gameplay


You can exit the game by pressing `q`.

***WORK IN PROGRESS***

The goal of every level in `l1t` is to turn all
of the statues on using the available laser beams.

### The Player

You can use the normal Vim keybindings to move around:

- `h`: LEFT
- `j`: DOWN
- `k`: UP
- `l`: RIGHT

You can also press `space` to interact with surrounding
items.

### Lasers

Lasers are the red blocks with red laser beams shooting out
of them and can be reflected off of mirrors.

You can toggle a laser by interacting with it and a laser will
turn off if a laser beam shoots into it. It can be turned back
on by the player.

Off lasers appear as a dimmed red.

### Statues

Statues are the dimmed yellow blocks when off and turn a bright
yellow when turned on by a laser beam. Turn on all lasers in a
level to win.

### Other Items

Some items on the grid can be moved around:

- `MIRRORS`: Mirrors are the `/` and `\` items on the grid
             and can be moved to redirect laser beams.
- `BLOCKS`: Blocks are the black boxes on the grid and can
            be used to block a laser beam.

## Contributing

See the guidelines for contributing [here](CONTRIBUTING.md)
