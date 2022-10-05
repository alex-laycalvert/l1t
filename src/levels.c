// alex-laycalvert
// https://github.com/alex-laycalvert/l1t

#include "levels.h"
#include "l1t.h"
#include "node.h"

/*
 * Level 000: Lonely
 * Description: A playground/testing area for development of new features.
 */
void init_level_000(const int trows, const int tcolumns, Node **grid) {
    init_walls();
    (&grid[trows / 2][tcolumns / 2])->type = PLAYER;
    (&grid[trows / 2][tcolumns / 2])->ch = PLAYER_CH;
}

/*
 * Level 001: The Basics
 * Description: The first official level of the game. Get used to game mechanics.
 */
void init_level_001(const int trows, const int tcolumns, Node **grid) {
    init_walls();
    (&grid[trows / 2][tcolumns / 2])->type = PLAYER;
    (&grid[trows / 2][tcolumns / 2])->ch = PLAYER_CH;
}
