// alex-laycalvert
// https://github.com/alex-laycalvert/l1t

#ifndef L1T_H_
#define L1T_H_

#define QUIT_KEY 'q'
#define MOVE_UP_KEY 'k'
#define MOVE_DOWN_KEY 'j'
#define MOVE_LEFT_KEY 'h'
#define MOVE_RIGHT_KEY 'l'

#define EMPTY_CH ' '
#define PLAYER_CH 'X'
#define WALL_CH 'I'
#define MIRROR_FORWARD_CH '/'
#define MIRROR_BACKWARD_CH '\\'
#define BLOCK_CH 'K'
#define STATUE_CH 'S'
#define TOGGLE_BLOCK_CH 'T'
#define BUTTON_CH 'B'
#define SWITCH_CH 'W'
#define LASER_CH 'L'

typedef enum node_type {
    EMPTY,
    PLAYER,
    WALL,
    MIRROR_FORWARD,
    MIRROR_BACKWARD,
    BLOCK,
    STATUE,
    TOGGLE_BLOCK,
    BUTTON,
    SWITCH,
    LASER,
} NodeType;

typedef enum direction { UP, DOWN, LEFT, RIGHT } Direction;

typedef struct node {
    int row;
    int column;
    NodeType type;
    char ch;
    Direction dir;
} Node;

#endif // L1T_H_
