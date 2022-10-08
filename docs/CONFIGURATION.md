# l1t Configuration

You can configure your `l1t` setup in the `$HOME/.config/l1t.conf` file.

You can set values to configuration options like this:

```conf
Key=Value
```

Example: `MoveUpKey=w`

## Valid keys

| Key              | Default | Description                                    |
|------------------|---------|------------------------------------------------|
| `MoveUpKey`      | `k`     | Key press to move the player up                |
| `MoveDownKey`    | `j`     | Key press to move the player down              |
| `MoveLeftKey`    | `h`     | Key press to move the player left              |
| `MoveRightKey`   | `l`     | Key press to move the player right             |
| `InteractionKey` | `space` | Key press to interact with surrounding objects |
| `RestartKey`     | `r`     | Key press to restart the level                 |
| `QuitKey`        | `q`     | Key press to quit the game                     |

Capitalization matters for values. Currently there is no way to set arrow keys
to keybindings.
