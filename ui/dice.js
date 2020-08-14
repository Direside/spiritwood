'use strict';

function rollDice(id) {
    let el = document.getElementById(id);
    let result = Math.floor(Math.random() * 6) + 1;
    displayElement.src = `./images/dice/d${result}.png`;
}

export { rollDice }