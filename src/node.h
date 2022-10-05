// alex-laycalvert
// https://github.com/alex-laycalvert/l1t

#ifndef NODE_H_
#define NODE_H_

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

#endif // NODE_H_
