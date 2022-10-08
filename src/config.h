// alex-laycalvert
// https://github.com/alex-laycalvert/l1t

#ifndef CONFIG_H_
#define CONFIG_H_

#define LINE_BUFFER_SIZE 256
#define L1T_CONFIG_FILE "/.config/l1t.conf"

#define MOVE_UP_KEY_CONFIG "MoveUpKey"
#define MOVE_DOWN_KEY_CONFIG "MoveDownKey"
#define MOVE_LEFT_KEY_CONFIG "MoveLeftKey"
#define MOVE_RIGHT_KEY_CONFIG "MoveRightKey"
#define INTERACTION_KEY_CONFIG "InteractionKey"
#define RESTART_KEY_CONFIG "RestartKey"
#define QUIT_KEY_CONFIG "QuitKey"

typedef struct {
    int move_up_key;
    int move_down_key;
    int move_left_key;
    int move_right_key;
    int interaction_key;
    int restart_key;
    int quit_key;
} Configuration;

Configuration read_configuration(char *filename);

#endif // CONFIG_H_
