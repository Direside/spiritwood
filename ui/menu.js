'use strict';

function readyScreen() {
    ready.classList.remove("hide");
    menu.classList.add("hide");

    location.href = `http://localhost:5000/#${window.gameID}`
    window.update();
}

function creditScreen() {
    credits.classList.remove("hide");
    menu.classList.add("hide");
}

function disablePlayerActions() {
    document.getElementById('get-tile').disabled = true;
    document.getElementById('move').disabled = true;
    document.getElementById('search').disabled = true;
    document.getElementById('roll').disabled = true;
    document.getElementById('end').disabled = true;
}

function enableGetTile() {
    document.getElementById('get-tile').disabled = false;
}

export { readyScreen, creditScreen, disablePlayerActions, enableGetTile }