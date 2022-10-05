// alex-laycalvert
// https://github.com/alex-laycalvert/l1t

#include "levels.h"
#include "l1t.h"
#include "node.h"

void init_level_000(const int trows, const int tcolumns, Node **grid) {
    init_walls();
    (&grid[trows / 2][tcolumns / 2])->type = PLAYER;
    (&grid[trows / 2][tcolumns / 2])->ch = PLAYER_CH;
}

void init_level_001(const int trows, const int tcolumns, Node **grid) {
    init_walls();
    (&grid[trows / 2][tcolumns / 2])->type = PLAYER;
    (&grid[trows / 2][tcolumns / 2])->ch = PLAYER_CH;
}
