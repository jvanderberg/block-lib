# Block-lib

BLocks-lib is a game-logic only version of a simple tetromino based falling blocks game.

The game sticks roughly to 'official' piece dynamics but does not implement 'kicking' off the walls. It is entirely single threaded, with no async.

## Installing

`cargo add blocks-lib`

## Creating and manipulating a game

To create a game, you call the gamestate constructor, add an event handler, and then call start().

```rust
    let gs = GameState::new(
        width,
        height,
        hide_next_piece,
        difficulty
    );

    let ev = |ge: &GameEvent, gs: &GameState| match ge {
        ... Do something with the GameEvent to update the presentation ...
    };
    gs.add_event_handler(&ev);
    gs.start();
```

The GameState structure has a number of methods to move the game forward and control pieces:

-   GameState::move_left() moves the current piece left
-   GameState::move_right() moves the current piece right
-   GameState::move_down() moves the current piece down
-   GameState::rotate_right() rotates the current piece to the right
-   GameState::drop() drops the current piece
-   GameState::advance() advances the game a step by dropping the current piece, and performing other housekeeping activities. Implementations will typically call advanced() on a timer. Use GameState::get_piece_interval() to get the suggested interval in milliseconds, though there's nothing stopping an implementation from using its own interval.

## Game Event

The event handler gets called with significant game events:

-   GameEvent::GameStarted when the game has started
-   GameEvent::ScoreChanged when the score changes
-   GameEvent::LevelChanged when the level changes
-   GameEvent::PieceMoved when a piece has moved
-   GameEvent::PieceChanged when the current piece (and next piece) have changed
-   GameEvent::GameOver when the game is over
-   GameEvent::GameReset when the game is reset

A visual implementation will respond to these events and interrogate the GameState object passed to the event handler to update the presentation layer.

## Game State methods

-   GameState::get_board() returns the current pieces on the board. Each entry in the board.cells nested vector holds a PieceColor enum variant: Wall,
    Empty,
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
    Orange,
    Tracer. The color of Wall and Tracer are up to the implementation, but it's suggested to use the actual color names for the rest. Empty is just an empty space in the board.

    Here's an example drawing loop:

```rust
    for y in 0..board.height {
        for x in 0..board.width {
            draw_square(x, y, board.cells[x as usize][y as usize]);
        }
    }
```

-   GameState::get_next_piece() returns the next piece that will be put onto the board
-   GameState::get_score() returns the tuple (score, lines, level)
-   GameState::get_status() returns the current game status.
-   GameState::start() start a game, emitting the GameStarted event.
-   GameState::add_event_handler(Fn(&GameEvent, &GameState)) Takes a closure or function and calls it every time a GameEvent is emitted. The handling function should not attempt to mutate GameState (in fact it cannot), this is purely to update the presentation of the board to the user, not to implement additonal game logic.
