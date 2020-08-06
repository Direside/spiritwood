'use strict';

import { shuffle, shuffle2, addToDeck, drawCard } from './card.js';
import { rollDice } from './dice.js';
import { getCookie, setCookie, clearCookie } from './cookie.js'


const menu = document.getElementById("menu");
const ready = document.getElementById("ready");
const main = document.getElementById("main");
const board = document.getElementById("board");
const credits = document.getElementById("credits");
const name = document.getElementById("playerName");
const nextTileImage = document.getElementById("next-tile-image");

let height = 10;
let width = 15;
let bits = "36px"

const backend = "http://192.168.0.104:8000"

window.allTiles = [
    "bear_cave_corner_symbol.png",
    "cross.png",
    "graveyard_straight_symbol.png",
    "tee.png",
    "straight.png"
]

if (!window.playerName) {
    changeName();
} else {
    name.innerText = getCookie("playerName")
}

function changeName() {
    window.playerName = prompt("Please enter your name", "Harry Potter");
    setCookie("playerName", decodeURIComponent(window.playerName))
    name.innerText = getCookie("playerName")
}

window.clearPlayName = function clearPlayName() {
    clearCookie("playerName")
    changeName();
}

window.newGame = function newGame() {
    fetch(`${backend}/game`, {
        // mode: 'no-cors',
        method: "POST",
        headers: new Headers({
            "Access-Control-Allow-Origin": "*",
            'Access-Control-Allow-Headers': "*"
        })
    })
        .then((response) => {
            return response.json();
        })
        .then((data) => {
            console.log(data);
            window.gameID = data.id
            console.log(window.gameID)
            window.joinGame()
        });
}

window.joinGame = function joinGame() {
    window.playerName = getCookie("playerName")

    if (!window.gameID) {
        window.gameID = prompt("Game ID")
    }

    document.getElementById("debugGameID").innerText = window.gameID
    fetch(`${backend}/game/${window.gameID}?player=${window.playerName}`, {
        method: "PUT"
    }).then((data) => {
        window.playerID = data.playerID
        window.header = new Headers({
            "Authorization": `Bearer ${data.playerToken}`
        })
        window.readyScreen()
    })
}

window.updatePlayerList = function getGame() {
    fetch(`${backend}/game/${window.gameID}/`, {
        method: "GET"
    })
        .then((response) => {
            console.log(response)
            return response.json();
        })
        .then((data) => {
            console.log(data);
            window.players = data.players
            let players = document.getElementById("players")
            let playersHTML = ""

            window.players.forEach((player) => {
                playersHTML += `<li>${player}</li>`
            })

            players.innerHTML = playersHTML
        })
}

window.readyScreen = function readyScreen() {
    ready.classList.remove("hide");
    menu.classList.add("hide");

    location.href = `http://localhost:5000/#${window.gameID}`

    window.updatePlayerList()
    window.gettingPlayers = setInterval(window.updatePlayerList, 1000);
}

window.endTurn = function endTurn() {
    this.fetch(`${backend}/game/${window.gameID}/moves/endturn`, {
        headers: window.headers,
        method: "PUT"
    })
}

window.startGame = function startGame() {
    console.log(window.gettingPlayers)
    clearInterval(window.gettingPlayers);
    fetch(`${backend}/game/${window.gameID}/start`, {
        method: "PUT"
    }).then((r) => {
        return r.json();
    }).then(data => {
        enterGame();
    })
}

window.getNextTile = function getNextTile() {
    fetch(`${backend}/game/${window.gameID}/tile`, {
        headers: window.headers,
    })
    .then(r => {
        return r.json();
    })
    .then(data => {
        nextTileImage.innerHTML = `<img src='${data.image}' ondragstart='dragStart(event, ${data.id})' height='80 width='80' />`;
    });
}

window.getBoardTiles = async function getBoardTiles(x, y, radius) {
    return await fetch(`${backend}/game/${window.gameID}/tiles?x=${x}&y=${y}&radius=${radius}`, {
        headers: window.headers,
        method: "GET"
    }).then(r => {
        const data = r.json();
        console.log(data);
        return data;
    });
}

window.placeTile = async function placeTile(x, y, tileId) {
    x = parseInt(x);
    y = parseInt(y);
    tileId = parseInt(tileId);

    await fetch(`${backend}/game/${window.gameID}/placetile`, {
        headers: {
            ...window.headers,
            'Content-Type': 'application/json',
        },
        method: "PUT",
        body: JSON.stringify({
            x,
            y,
            tile: tileId,
        }),
    });
}

window.credit = function credit() {
    credits.classList.remove("hide");
    menu.classList.add("hide");
}

window.roll = function roll(id) {
    let el = document.getElementById(id)
    rollDice(el);
}

window.draw = function draw(deck, discard, displayId) {
    let discardPile = document.getElementById(displayId);
    let card = drawCard(deck, discard);
    discardPile.innerHTML = `<span>${card}</span>`
    setCardArt(card, discardPile);
}


// Arrow Keys
window.addEventListener('keyup', keyPress);
function keyPress(e) {
    let x = window.players[window.currPlayer]["x"]
    let y = window.players[window.currPlayer]["y"]
    switch (e.which) {
        case 37: // left
            if (x > 0) {
                document.getElementById(`board-${x}-${y}`).innerText = "";
                let xpos = x - 1;
                window.players[window.currPlayer]["x"] = xpos;
                document.getElementById(`board-${xpos}-${y}`).innerText = window.currPlayer;
            }
            break;

        case 38: // up
            if (y > 0) {
                document.getElementById(`board-${x}-${y}`).innerText = "";
                let ypos = y - 1;
                window.players[window.currPlayer]["y"] = ypos;
                document.getElementById(`board-${x}-${ypos}`).innerText = window.currPlayer;
            }
            break;

        case 39: // right
            if (x < 16) {
                document.getElementById(`board-${x}-${y}`).innerText = "";
                let xpos = x + 1;
                window.players[currPlayer]["x"] = xpos;
                document.getElementById(`board-${xpos}-${y}`).innerText = currPlayer;
            }
            break;

        case 40: // down
            if (y < 16) {
                document.getElementById(`board-${x}-${y}`).innerText = "";
                let ypos = y + 1;
                window.players[currPlayer]["y"] = ypos;
                document.getElementById(`board-${x}-${ypos}`).innerText = currPlayer;
            }
            break;

        default: return; // exit this handler for other keys
    }
    e.preventDefault(); // prevent the default action (scroll / move caret)
}

// Monster Decks
function setCardArt(cardName, flippedCard) {
    switch (cardName) {
        case "skeleton":
        case "zombie":
        case "dracula":
            flippedCard.classList.add("monster");
            break;
        case "sword":
        case "shield":
        case "potion":
            flippedCard.classList.add("item");
            break;
        default:
            flippedCard.classList.remove("monster");
            flippedCard.classList.remove("item");
    }
}

// Cards
let skel = {
    name: "skeleton",
    image: "skell.png",
    description: "Kills you. Roll 3+ to escape."
}

// Graveyard
window.gy = [];

window.gy = addToDeck(window.gy, "skeleton", 18);
window.gy = addToDeck(window.gy, "zombie", 12);
window.gy = addToDeck(window.gy, "dracula", 4);
window.gy = addToDeck(window.gy, "sword", 2);
window.gy = addToDeck(window.gy, "shield", 3);
window.gy = addToDeck(window.gy, "potions", 10);
window.gy = addToDeck(window.gy, "nothing", 25);

window.gy = shuffle(window.gy);

// Pile of Bones
window.bone = [];

window.bone = addToDeck(window.bone, "key", 2);
window.bone = addToDeck(window.bone, "leg bone", 7);
window.bone = addToDeck(window.bone, "spell", 4);
window.bone = addToDeck(window.bone, "nothing", 5);
window.bone = addToDeck(window.bone, "portal", 1);

window.bone = shuffle(window.bone);

// Board Setup
board.style.setProperty("--width", width);
board.style.setProperty("--height", height);
board.style.setProperty("--bits", bits)


const xs = Array.from({ length: width }, (_, i) => i)
const ys = Array.from({ length: height }, (_, i) => i)

let tiles = [];

var current = ""

window.dragStart = function dragStart(event, tileId) {
    current = event.dataTransfer.setData("tile_id", tileId);
}

window.drop = async function drop(event) {
    event.preventDefault()

    console.log(`Drop event: %o`, event);

    const data = event.target.dataset
    const x = data.x
    const y = data.y

    // Handle dropped tile
    if (event.dataTransfer.getData("tile_id")) {
        const tileId = event.dataTransfer.getData("tile_id");
        await window.placeTile(x, y, tileId);
    }

    event.target.classList.remove("over")
    paint()
}

window.over = function over(event) {
    event.preventDefault()
}

window.enter = function enter(event) {
    event.target.classList.add("over")
}

window.leave = function leave(event) {
    event.target.classList.remove("over")
}

const enterGame = () => {
    main.classList.remove("hide");
    ready.classList.add("hide");
    menu.classList.add("hide");
}

const loadExistingGame = () => {
    const gameID = decodeURIComponent(window.location.hash.substring(1));
    if (gameID && gameID.length > 0) {
        window.gameID = gameID;
        enterGame();
    }
}

const calcBoundingBox = (xs, ys) => {
    let left = xs[0];
    let right = xs[0];
    let top = ys[0];
    let bottom = ys[0];

    for (const x of xs) {
        if (x < left) {
            left = x;
        }
        if (x > right) {
            right = x;
        }
    }

    for (const y of ys) {
        if (y < top) {
            top = y;
        }
        if (y > bottom) {
            bottom = y;
        }
    }

    return {
        left,
        right,
        top,
        bottom,
    };
}

const fetchTiles = async () => {
    const {left, right, top, bottom} = calcBoundingBox(xs, ys);
    let diameter = right - left;
    if (bottom - top > diameter) {
        diameter = bottom - top;
    }
    const radius = diameter / 2;
    const x = left + radius + (diameter % 2);
    const y = top + radius + (diameter % 2);

    const placedTiles = await window.getBoardTiles(x, y, radius);
    for (const placedTile of placedTiles) {
        if (!tiles[placedTile.x]) {
            tiles[placedTile.x] = [];
        }

        tiles[placedTile.x][placedTile.y] = placedTile.tile;
    }
};

const getTile = (x, y) => {
    const tile = (tiles[x] || [])[y]
    if (tile) {
        return "background-image: url(" + tile.image + ")"
    } else {
        return ""
    }
}

async function paint() {
    if (window.gameID) {
        await fetchTiles();
    }

    let grid = "";

    for (let y of ys) {
        for (let x of xs) {
            grid += `<div id="board-${x}-${y}" class="board-cell" data-x="${x}" data-y="${y}" ondragover="over(event)" ondrop="drop(event)" ondragenter="enter(event)" ondragleave="leave(event)" style="${getTile(x, y)}"></div>`
        }
    }

    board.innerHTML = grid;
}

loadExistingGame();
paint();

const palette = document.createElement("div")
palette.classList.add("palette")
const images = []
for (let img of images) {
    const paletteEl = document.createElement("img")
    paletteEl.src = img
    paletteEl.style.width = "36px";
    paletteEl.style.height = "36px";
    paletteEl.classList.add("palette-art")
    paletteEl.draggable = true;
    paletteEl.dataset.url = img
    paletteEl.ondragstart = window.dragStart
    palette.appendChild(paletteEl)
}
document.getElementById("hand").appendChild(palette)
