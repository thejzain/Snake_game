## Snake Game

This is a simple Piston snake game.

### Installation

To install and run the game, clone this repository and run the following command:

```
cargo run
```

### How to play

The goal of the game is to control the snake and eat as many apples as possible without hitting itself or the edge of the screen. Use the arrow keys to move the snake.

### Documentation

Here is a brief overview of the game's classes:

#### Game

The `Game` struct represents the overall game state. It contains the following attributes:

* `gl`: A GlGraphics object used to render the game.
* `snake`: A Snake object representing the snake.
* `apple`: An Apple object representing the apple.
* `game_update`: A bool value indicating whether the game is currently being updated.

#### Snake

The `Snake` struct represents the snake in the game. It contains the following attributes:

* `body`: A LinkedList containing the coordinates of the snake's body segments.
* `dir`: A Direction enum value indicating the direction in which the snake is moving.

#### Apple

The `Apple` struct represents the apple in the game. It contains the following attributes:

* `x`: The x-coordinate of the apple.
* `y`: The y-coordinate of the apple.

### Example usage

Here is an example of how to use the game:

```rust
use snake_game::Game;

fn main() {
    let mut game = Game::new();

    while game.game_update {
        // Update the game state.
        game.update();

        // Render the game.
        game.render();

        // Check for input.
        let mut events = Events::new(EventSettings::new()).ups(8);
        while let Some(e) = events.next(&mut game.window) {
            if let Some(args) = e.render_args() {
                game.render(&args);
            }

            if app.game_update {
                if let Some(_u) = e.update_args() {
                    game.update();
                }
            }

            if let Some(k) = e.button_args() {
                if k.state == ButtonState::Press {
                    game.pressed(&k.button);
                }
            }
        }
    }
}
```

## Conclusion

This is a simple Piston snake game. You can use the arrow keys to move the snake and try to eat as many apples as possible before the game is over.