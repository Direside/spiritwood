'use strict';

import { draw } from './card.js';
import { rollDice } from './dice.js';

import { getNextTile, getBoardTiles, updateLocalTile, placeTile, getRotation, rotateLeft, rotateRight, drag, drop, over, enter, leave, fetchTiles, getTile } from './tiles.js';
import { getPlayerName, changePlayerName, clearPlayerName, keyPress } from './player.js';
import { newGame, getGame, joinGame, startGame, enterGame, loadExistingGame, endTurn } from './game.js';
import { readyScreen, creditScreen } from './menu.js';

// Set up global functions
window.backend = "http://localhost:8000";

window.headers = new Headers({
    "Access-Control-Allow-Origin": "*",
    'Access-Control-Allow-Headers': "*",
    "Content-Type": "application/json"
});

// Player
const name = document.getElementById("playerName");
window.players = [];
window.currPlayer = null;

if (!window.playerName) {
    changePlayerName();
} else {
    window.playerName = getPlayerName();
    name.innerText = window.playerName;
}

window.addEventListener('keyup', keyPress);
window.clearPlayName = clearPlayerName;

// Menus
window.readyScreen = readyScreen;
window.credit = creditScreen;

// Game Logic
window.newGame = newGame;
window.joinGame = joinGame;
window.startGame = startGame;
window.endTurn = endTurn;
window.update = getGame;

// Tiles
window.getNextTile = getNextTile;
window.getBoardTiles = getBoardTiles;
window.placeTile = placeTile;

window.rotateLeft = rotateLeft;
window.rotateRight = rotateRight;

window.dragStart = drag;
window.drop = drop;
window.over = over;
window.enter = enter;
window.leave = leave;

// Dice & Cards
window.roll = rollDice;
window.draw = draw;

// Board Setup
const board = document.getElementById("board");
const height = 10;
const width = 15;
const bits = "36px";

board.style.setProperty("--width", width);
board.style.setProperty("--height", height);
board.style.setProperty("--bits", bits)

window.xs = Array.from({ length: width }, (_, i) => i)
window.ys = Array.from({ length: height }, (_, i) => i)
window.tiles = [];
window.current = "";

async function paint() {
    if (window.gameID) {
        await fetchTiles();
    }

    let grid = "";

    for (let y of ys) {
        for (let x of xs) {
            grid += `<div id="board-${x}-${y}" class="board-cell rotate-${getRotation(x, y)}" data-x="${x}" data-y="${y}" ondragover="over(event)" ondrop="drop(event)" ondragenter="enter(event)" ondragleave="leave(event)" style="${getTile(x, y)}"></div>`
        }
    }

    board.innerHTML = grid;
}

loadExistingGame();
paint();

getGame();
setInterval(() => {
    if (window.gameID) {
        getGame()
    }
}, 10000);