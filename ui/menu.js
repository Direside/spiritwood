'use strict';

function readyScreen() {
    ready.classList.remove("hide");
    menu.classList.add("hide");

    location.href = `http://localhost:5000/#${window.gameID}`

    window.updatePlayerList()
    window.gettingPlayers = setInterval(window.updatePlayerList, 1000);
}

function creditScreen() {
    credits.classList.remove("hide");
    menu.classList.add("hide");
}

export { readyScreen, creditScreen }