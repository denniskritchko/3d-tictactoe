# 3D Tic-Tac-Toe

A 3D tic-tac-toe game built with Rust and Bevy, featuring a rotating cube interface and AI opponent powered by Monte Carlo Tree Search.

## Features

- **3D Visualization**: A 3x3x3 grid of cubes displayed in 3D space
- **Interactive Camera**: Freely rotate around the cube to see all angles
- **AI Opponent**: Smart AI using Monte Carlo Tree Search algorithm
- **Winning Detection**: Comprehensive 3D win condition checking including:
  - Lines along X, Y, and Z axes
  - Face diagonals on all planes
  - 3D space diagonals (corner to corner)
- **Modern UI**: Clean interface with game status display

![172EF63E-552D-420C-A1EB-6A514BEACA55_1_105_c](https://github.com/user-attachments/assets/a1cc20fc-4ce1-43a7-a827-6e193d88ac7a)

## How to Play

### Controls
- **Left Mouse Click**: Select and place your move on a cube
- **WASD Keys**: Rotate the camera around the cube
- **Right Mouse + Drag**: Alternative camera rotation (mouse look)
- **R Key**: Reset the game

### Gameplay
1. You play as the green cubes, AI plays as red cubes
2. Click on any empty cube to make your move
3. The AI will automatically make its move after a short delay
4. Win by getting 3 cubes in a row in any direction:
   - Straight lines along any axis
   - Diagonals on any face
   - 3D diagonals through the center

## Technical Details

### Architecture
- **Game Logic**: Pure Rust implementation with 3D position tracking
- **AI**: Monte Carlo Tree Search with random game simulation
- **Graphics**: Bevy engine for 3D rendering and input handling
- **Materials**: Color-coded cubes with transparency for empty spaces

### AI Implementation
The AI uses a simplified Monte Carlo Tree Search approach:
- Evaluates each possible move by running random simulations
- Chooses the move with the highest win probability
- Runs 100 simulations per move for balanced performance

## Building and Running

### Prerequisites
- Rust (latest stable version)
- macOS, Windows, or Linux

### Installation
```bash
# Clone the repository
git clone <repository-url>
cd 3d-tictactoe

# Build and run
cargo run
```

### Development
```bash
# Check for compilation errors
cargo check

# Run with optimizations
cargo run --release
```

## Game Rules

In 3D tic-tac-toe, you can win by getting three of your cubes in a line in any of these ways:

1. **Axis Lines**: Three cubes in a straight line along X, Y, or Z axis
2. **Face Diagonals**: Three cubes diagonally across any face of the cube
3. **Space Diagonals**: Three cubes diagonally from corner to corner through the center

This creates 76 total winning combinations compared to 8 in traditional 2D tic-tac-toe!

## Future Enhancements

- [ ] Difficulty levels for AI
- [ ] Multiplayer support
- [ ] Game statistics and history
- [ ] Sound effects and animations
- [ ] Different board sizes (4x4x4, 5x5x5)
- [ ] Undo/redo functionality

## Contributing

Feel free to submit issues, suggestions, or pull requests to improve the game!

## License

This project is licensed under the MIT License - see the LICENSE file for details.
