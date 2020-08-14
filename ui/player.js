'use strict';

import { getCookie, setCookie, clearCookie } from './cookie.js'

function getPlayerName() {
    return getCookie("playerName");
}

function changePlayerName() {
    window.playerName = prompt("Please enter your name", "Harry Potter");
    setCookie("playerName", decodeURIComponent(window.playerName))
    name.innerText = getCookie("playerName")
}

function clearPlayerName() {
    clearCookie("playerName")
    changeName();
}

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


export { getPlayerName, changePlayerName, clearPlayerName, keyPress }