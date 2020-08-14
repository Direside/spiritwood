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

export { readyScreen, creditScreen }