# l1t

A terminal strategy game about shooting lasers and lighting statues. Built in Rust.

![l1t-logo](https://user-images.githubusercontent.com/45835678/194675329-027fd0d9-e1ee-4149-980b-e2fc7099206e.png)

In l1t, your goal is to use the available lasers to light up all of the
statues in the level.

## CONTROLS

-   **W** - Move Up
-   **S**: Move Down
-   **A**: Move Left
-   **D**: Move Right
-   **Space**: Toggle surrounding blocks (if able)
-   **Shift-H**: Show this help menu
-   **Q**: Quit

Arrow keys can also be used to move around the level

## PLAYER

Hey, that's you!

## LASERS

Lasers shoot laser beams in their set direction (UP, DOWN, LEFT, RIGHT).
Laser beams are the key to winning the game and can affect various blocks.

Lasers cannot change directions but they can be toggled on and off.

If a laser beam hits you, you will die and have to restart the level.

If a laser is hit by a laser beam, it will turn off and must be toggled on by the player.

## STATUES

All statues in a level must be lit up by a laser beam to win the level.

Statues can not be moved or manually toggled.

## REVERSE STATUES

Same as statues except they must **NOT** be lit up to win the level.

## MIRRORS

Mirrors reflect laser beams in different directions.

Mirrors cannot be moved but their direction can be toggled by the player.

## MOVEABLE MIRRORS

Moveable Mirrors are the same as mirrors except they **CAN** be moved.

## ZAPPERS

If any Zappers are lit by a laser beam, you will lose the level.

## OTHER BLOCKS

-   **Walls**: Cannot be moved by player, will block laser beams.

-   **Blocks**: Can be moved around and will block laser beams.

-   **Toggle Blocks**: Cannot be moved. Switches and buttons can toggle these
    on and off.

-   **Switches**: When toggled, will turn toggle blocks on/off.

-   **Buttons**: When pressed, will turn toggle blocks on/off. Player must be
    next to button to press.
