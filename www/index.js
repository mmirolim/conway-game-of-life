'use strict'
import { Universe, Cell } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

let universeInitState = [];
let universe = Universe.new();
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

let STATE = 'stop';
const playEl = document.getElementById("play");

const pause = (e) => {
    STATE = 'pause';
    let target = e.target;
    target.textContent = 'play';
    target.removeEventListener('click', pause);
    target.addEventListener('click', play);
}

const play = (e) => {
    STATE = 'play';
    let target = e.target;
    target.textContent = 'pause';
    target.removeEventListener('click', play);
    target.addEventListener('click', pause);
    if (universeInitState.length > 0) {
	universe = Universe.new_with_state(new Uint32Array(universeInitState));
    }
    renderLoop();
}

playEl.addEventListener('click', play);

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

const drawCells = (ctx, cells) => {
    ctx.fillStyle = ALIVE_COLOR;
    cells.forEach((cell) => {
	ctx.fillRect(cell.x * (CELL_SIZE+1) + 1, cell.y * (CELL_SIZE + 1) + 1,
		     CELL_SIZE, CELL_SIZE);
	universeInitState.push(getIndex(cell.x, cell.y));
    });
    ctx.stroke();
}

const bitIsSet = (n, arr) => {
    const byte = Math.floor(n / 8);
    const mask = 1 << (n % 8);
    return (arr[byte] & mask) === mask;
}

const drawUniverse = (ctx, universe) => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height / 8);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
	for (let col = 0; col < width; col++) {
	    const idx = getIndex(row, col);

	    ctx.fillStyle = bitIsSet(idx, cells) ?  ALIVE_COLOR : DEAD_COLOR;
	    ctx.fillRect(col * (CELL_SIZE+1) + 1, row * (CELL_SIZE + 1) + 1,
			 CELL_SIZE, CELL_SIZE);
	}
    }

    ctx.stroke();
}
// return function defined by start, end coords
const yfn = (x1, y1, x2, y2) => {
    if (Math.abs(y2 - y1) < CELL_SIZE) {
	return function(x) {
	    return y1;
	}
    }
    if (Math.abs(x2 - x1) < CELL_SIZE) {
	return function(x) {
	    return (Math.abs(x - x1) < CELL_SIZE) ? y1 : 0;
	}
    }
    let k = (x2 - x1)/(y2 - y1);
    const y0 = y2 - k*x2;
    return function(x) {
	return k*x + y0;
    }
}

const cellFromPx = (x, y) => {
    const cellSize = CELL_SIZE + 1;
    let cx = Math.floor(x / cellSize);
    let cy = Math.floor(y/ cellSize);
    return {x: cx, y: cy}
}

// lines in pixels to cells coords
const canvasLineToCells = (x1, y1, x2, y2) => {
    const y = yfn(x1, y1, x2, y2);
    let temp;
    if (x1 > x2) {
	temp = x1;
	x1 = x2;
	x2 = temp;
    }
    let cells = [];
    for (let x = x1; x <= x2; x++) {
	let cell= cellFromPx(x, y(x));
	cells.push(cell);
    }
    return cells;
}

const getXYFromEvent = (e) => {
    const target = e.target || e.srcElement;
    const rect = target.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;
    return {x: x, y: y};
}

let DRAW_ON_CANVAS = false;
let DRAW_IN_PROCESS = false;
let drawLines = [];

const canvasDraw = () => {
    DRAW_ON_CANVAS = !DRAW_ON_CANVAS
}
const drawBtn = document.getElementById("draw");
drawBtn.addEventListener('click', canvasDraw);

const canvasHandleMouseDown = (e) => {
    if (!DRAW_ON_CANVAS) {
	return;
    }
    DRAW_IN_PROCESS = true;
    drawLines = [];    
    drawLines.push(getXYFromEvent(e));
}

const canvasHandleMouseMove = (e) => {
    if (!DRAW_IN_PROCESS) {
	return;
    }
    drawLines.push(getXYFromEvent(e));
    const end = drawLines[drawLines.length-1];
    const start = drawLines[drawLines.length-2];
    drawCells(ctx, canvasLineToCells(start.x, start.y,
				     end.x, end.y));
}

const canvasHandleMouseUp = (e) => {
    if (!DRAW_IN_PROCESS) {
	return;
    }
    drawLines.push(getXYFromEvent(e));
    const end = drawLines[drawLines.length-1];
    const start = drawLines[drawLines.length-2];
    drawCells(ctx, canvasLineToCells(start.x, start.y,
				     end.x, end.y));
    DRAW_IN_PROCESS = false;
}

canvas.addEventListener('mousedown', canvasHandleMouseDown);
canvas.addEventListener('mousemove', canvasHandleMouseMove);
canvas.addEventListener('mouseup', canvasHandleMouseUp);


const resetEl = document.getElementById("reset");
const reset = () => {
    STATE = 'stop';
    universeInitState = [];
    universe = Universe.new();
    drawLines = [];
    DRAW_IN_PROCESS = false;
    DRAW_ON_CANVAS = false;
    ctx.fillStyle = DEAD_COLOR;
    ctx.fillRect(0, 0, canvas.width, canvas.height);
    ctx.stroke();
    playEl.textContent = 'play';
    playEl.removeEventListener('click', pause);
    playEl.addEventListener('click', play);
}

resetEl.addEventListener('click', reset);

const renderLoop = () => {
    if (STATE !== 'play') {
	return;
    }
    universe.tick();

    drawGrid(ctx);
    drawUniverse(ctx, universe);
    requestAnimationFrame(renderLoop);
}
