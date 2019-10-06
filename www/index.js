'use strict'
import { Universe, Cell } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

const universe = Universe.new();
const width = universe.width();
const height = universe.height();

const CELL_SIZE = 5; // px
const GRID_COLOR = 'grey';
const DEAD_COLOR = 'white';
const ALIVE_COLOR = 'green';

const canvas = document.getElementById("game-of-life-canvas");
// 1px border size
canvas.width = (CELL_SIZE +1) * width + 1;
canvas.height = (CELL_SIZE + 1) * height +1;
const ctx = canvas.getContext('2d');

const drawGrid = (ctx) => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    const cellSize = CELL_SIZE + 1;
    // vertical lines
    for (let i = 0; i <= width; i++) {
	let x = i * cellSize + 1;
	ctx.moveTo(x, 0);
	ctx.lineTo(x, ctx.height);
    }

    // horizontal lines
    for (let i = 0; i <= height; i++) {
	let y = i* cellSize + 1;
	ctx.moveTo(0, y);
	ctx.lineTo(ctx.width, y);
    }

    ctx.stroke();
}

const getIndex = (row, col) => {
    return row * width + col;
}

const drawCells = (ctx, universe) => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
	for (let col = 0; col < width; col++) {
	    const idx = getIndex(row, col);

	    ctx.fillStyle = cells[idx] === Cell.Dead ? DEAD_COLOR : ALIVE_COLOR;
	    ctx.fillRect(col * (CELL_SIZE+1) + 1, row * (CELL_SIZE + 1) + 1,
			 CELL_SIZE, CELL_SIZE);
	}
    }

    ctx.stroke();
}

const renderLoop = () => {
    universe.tick();

    drawGrid(ctx);
    drawCells(ctx, universe);
    
    requestAnimationFrame(renderLoop);
}

requestAnimationFrame(renderLoop);
