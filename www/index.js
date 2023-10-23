import { Universe, Cell } from "wasm-game-of-life"; // Import the Universe class
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

const CELL_SIZE = 5; // Size of each cell in pixels
const GRID_COLOR = "#CCCCCC"; // Color of the grid lines
const DEAD_COLOR = "#FFFFFF"; // Color of dead cells
const ALIVE_COLOR = "#000000"; // Color of alive cells

// Create a Universe instance
const universe = Universe.new();
const width = universe.width();
const height = universe.height();

// Construct the universe, and get its width and height.
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

// initialize cells
const bitIsSet = (n, arr) => {
  const byte = Math.floor(n / 8);
  const mask = 1 << (n % 8);
  return (arr[byte] & mask) === mask;
}

let animationId = null;
let animationsPerFrame = 1;
const renderLoop = () => {
  // debugger;

  // Tick the universe
  for (let i = 0; i < animationsPerFrame; i++) {
    universe.tick();
  }

  drawGrid(); // Draw the grid
  drawCells(); // Draw the cells

  animationId = requestAnimationFrame(renderLoop); // Request the next animation frame
}

const drawGrid = () => {
  ctx.beginPath(); // Start drawing a path
  ctx.strokeStyle = GRID_COLOR; // Set the color of the path to GRID_COLOR

  // Vertical lines.
  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }

  // Horizontal lines.
  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
};

const getIndex = (row, column) => {
  return row * width + column;
};

const drawCells = () => {
  // Get the cells from Universe
  const cellsPtr = universe.cells();

  // Point cellsPtr at the cells in memory
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height / 8);

  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);

      // This is updated!
      ctx.fillStyle = bitIsSet(idx, cells)
        ? ALIVE_COLOR
        : DEAD_COLOR;

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  ctx.stroke();
};

/// Play and Pause
const isPaused = () => {
  return animationId === null;
};

const playPauseButton = document.getElementById("play-pause");

const play = () => {
  playPauseButton.textContent = "⏸";
  renderLoop();
};

const pause = () => {
  playPauseButton.textContent = "▶";
  cancelAnimationFrame(animationId);
  animationId = null;
};

playPauseButton.addEventListener("click", event => {
  if (isPaused()) {
    play();
  } else {
    pause();
  }
});

/// Set to random
const randomizeButton = document.getElementById("randomize");
randomizeButton.addEventListener("click", event => {
  universe.randomize();
  drawGrid();
  drawCells();
});

/// Clear the universe
const clearButton = document.getElementById("clear");

clearButton.addEventListener("click", event => {
  universe.clear();
  drawGrid();
  drawCells();
});

/// Toggle a cell
canvas.addEventListener("click", event => {	
  const boundingRect = canvas.getBoundingClientRect();

  const scaleX = canvas.width / boundingRect.width;
  const scaleY = canvas.height / boundingRect.height;

  const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
  const canvasTop = (event.clientY - boundingRect.top) * scaleY;

  const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
  const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

  universe.toggle_cell(row, col);

  drawGrid();
  drawCells();
});

/// Animation speed
const animationsPerFrameSlider = document.getElementById("animation-per-frame-slider");

animationsPerFrameSlider.addEventListener("change", event => {
  animationsPerFrame = animationsPerFrameSlider.value;
  console.log(animationsPerFrame);
});

/// Start the game
play();