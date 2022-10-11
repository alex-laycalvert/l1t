# l1t

![l1t-logo](https://user-images.githubusercontent.com/45835678/194675329-027fd0d9-e1ee-4149-980b-e2fc7099206e.png)

A terminal game where you position mirrors in a grid to light
up statues. Light up all the statues in a grid to complete
the level.

## Progress

***NOTE***: This project is still in development.

Number of Levels: `3`

## Installation and Running

Follow the steps below to install `l1t` for your OS:

<details>
<summary>
<h3>Linux</h3>
</summary>

- Clone this repository

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
</details>
<details>
<summary>
<h3>macOS</h3>
</summary>

For macOS users, you will need to make sure that the proper
`ncurses` library is installed on your machine. You can install
[Homebrew](https://brew.sh/) by following the steps on their
website then run `brew install ncurses`.

- Clone this repository

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
</details>

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

Shooting a laser beam into a laser will turn that laser off
and it cannot be turned back on. Off lasers appear as a
dimmed red.

### Statues

Statues are the dimmed yellow blocks when off and turn a bright
yellow when turned on by a laser beam. Turn on all lasers in a
level to win.

### Mirrors

Mirrors are the `/` and `\` characters that reflect laser beams.
Mirrors can have their direction toggled but cannot be moved.

### Zappers

Zappers are the `Z` characters that **cannot be hit by lasers**.
If a laser hits a zapper, it is game over!

### Other Items

Some items on the grid can be moved around:

- `BLOCKS`: Blocks are the black boxes on the grid and can
            be used to block a laser beam.

## Configuration

See the options for configurations [here](docs/CONFIGURATION.md).

## Contributing

See the guidelines for contributing [here](docs/CONTRIBUTING.md).
