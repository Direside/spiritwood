'use strict';

import {shuffle, shuffle2, addToDeck, drawCard} from './card.js';
import {rollDice} from './dice.js';
import { getCookie, setCookie } from './cookie'


let menu = document.getElementById("menu");
let main = document.getElementById("main");
let board = document.getElementById("board");
let credits = document.getElementById("credits");

let selectedPlayers = document.getElementById("players");

window.players = [];
window.currPlayer = 0;

let height = 10;
let width = 15;
let bits = "36px"

window.newGame = function newGame() {
    fetch("http://localhost:8000/game", {
        mode: 'no-cors',
        method: "POST",
        headers: new Headers({
            "Access-Control-Allow-Origin": "*",
            'Access-Control-Allow-Headers': "*"
        })
    })
    .then((response) => {
        console.log(response)
        return response.text();
      })
      .then((data) => {
        console.log(data);

        window.game_id = data.id
        window.playerName = getCookie("playerName")
        if (!window.playerName) {
            window.playerName = prompt("Please enter your name", "Harry Potter");
            setCookie("playerName", window.playerName)
        }

      });
}

window.startGame = function startGame() {
    window.newGame()
    // Get number of players
    let numPlayers = selectedPlayers.options[selectedPlayers.selectedIndex].value;

    for(let i=0;i<numPlayers;i++){
    let p = {
        name: "Player " + (i+1),
        x: null,
        y: null
    }
    window.players.push(p);
    }

    main.classList.remove("hide");
    menu.classList.add("hide");

    window.players.forEach((_,i) => {
        let pos = 3+i*2
        document.getElementById(`board-${pos}-8`).innerText = i;
        window.players[i].x = pos;
        window.players[i].y = 8;
    })

    console.log(window.players);
}

window.roll = function roll(id){
    let el = document.getElementById(id)
    rollDice(el);
}

window.draw = function draw(deck, discard, displayId){
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
    switch(e.which) {
        case 37: // left
            if ( x > 0 ) { 
                document.getElementById(`board-${x}-${y}`).innerText = "";
                let xpos = x - 1;
                window.players[window.currPlayer]["x"] = xpos;
                document.getElementById(`board-${xpos}-${y}`).innerText = window.currPlayer;
            }
        break;

        case 38: // up
            if ( y > 0 ) { 
                document.getElementById(`board-${x}-${y}`).innerText = "";
                let ypos = y - 1;
                window.players[window.currPlayer]["y"] = ypos;
                document.getElementById(`board-${x}-${ypos}`).innerText = window.currPlayer;
            }
            break;

        case 39: // right
            if ( x < 16 ) { 
                document.getElementById(`board-${x}-${y}`).innerText = "";
                let xpos = x + 1;
                window.players[currPlayer]["x"] = xpos;
                document.getElementById(`board-${xpos}-${y}`).innerText = currPlayer;
            }
            break;

        case 40: // down
            if ( y < 16 ) { 
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

const tiles = [
    {
        id: 1,
        type: "+",
        image: "images/tiles/cross.png"
    },
    {
        id: 2,
        type: "-",
        image: "images/tiles/straight.png"
    },
    {
        id: 3,
        type: "T",
        image: "images/tiles/tee.png"
    }
]

var current = ""

window.dragStart = function dragStart(event) {
    current = event.target.dataset.url
}

window.drop = function hmm(event) {
    event.preventDefault()
    const data = event.target.dataset
    const x = data.x
    const y = data.y
    tiles[x] = tiles[x] || []
    tiles[x][y] = tiles[x][y] || ""
    tiles[x][y] = current
    current = ""
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

const getTile = (x, y) => {
    const url = (tiles[x] || [])[y]
    if (url) {
        return "background-image: url(" + url + ")"
    } else {
        return ""
    }
}

function paint() {
    let grid = "";

    for (let y of ys) {
        for (let x of xs) {
            grid += `<div id="board-${x}-${y}" class="board-cell" data-x="${x}" data-y="${y}" ondragover="over(event)" ondrop="drop(event)" ondragenter="enter(event)" ondragleave="leave(event)" style="${getTile(x, y)}"></div>`
        }
    }

    board.innerHTML = grid;
}

paint()

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