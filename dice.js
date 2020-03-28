'use strict';

function rollDice(displayElement) {
    let result = Math.floor(Math.random() * 6) + 1;
    displayElement.src = `./images/d${result}.png`;
    console.log(displayElement)
}

export { rollDice }