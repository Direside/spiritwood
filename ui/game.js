'use strict';

import { getCookie } from './cookie.js'
import { disablePlayerActions, enableGetTile } from './menu.js';

function newGame() {
    fetch(`${window.backend}/game`, {
        // mode: 'no-cors',
        method: "POST",
        headers: {
            ...window.headers
        }
    })
        .then((response) => {
            return response.json();
        })
        .then((data) => {
            window.gameID = data.id
            window.joinGame()
        });
}

function joinGame() {
    window.playerName = getCookie("playerName")
    
    if (!window.gameID) {
        window.gameID = prompt("Game ID")
    }
    
    document.getElementById("debugGameID").innerText = window.gameID
    fetch(`${window.backend}/game/${window.gameID}?player=${window.playerName}`, {
        method: "PUT"
    }).then((response) => {
        return response.json();
    }).then((data) => {
        localStorage.setItem('playerIndex', data.order);
        window.playerID = data.order;
        window.headers += new Headers({
            "Authorization": `Bearer ${data.key}`
        });
        window.readyScreen();
    })
}

function getGame() {
    fetch(`${window.backend}/game/${window.gameID}/`, {
        headers: {
            ...window.headers
        },
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

            let turnNumber = document.getElementById('turn-number');
            turnNumber.innerText = data.turn;

            let playersTurn = document.getElementById('players-turn');
            playersTurn.innerText = window.players[data.current_player];

            if (data.current_player != localStorage.getItem('playerIndex')){
                console.log("not your turn", data.current_player, "!=", window.playerID)
                disablePlayerActions();
            } else {
                console.log("it's your turn")
                enableGetTile();
            }
        })
}

function startGame() {
    console.log(window.gettingPlayers)
    clearInterval(window.gettingPlayers);
    fetch(`${window.backend}/game/${window.gameID}/start`, {
        method: "PUT",
        headers: {
            ...window.headers,
            'Content-Type': 'application/json',
        },
    }).then(() => {
        enterGame();
    })
}

const enterGame = () => {
    main.classList.remove("hide");
    ready.classList.add("hide");
    menu.classList.add("hide");

    window.update();
}

const loadExistingGame = () => {
    const gameID = decodeURIComponent(window.location.hash.substring(1));
    if (gameID && gameID.length > 0) {
        window.gameID = gameID;
        enterGame();
    }
}

function endTurn() {
    this.fetch(`${window.backend}/game/${window.gameID}/moves/endturn`, {
        headers: {
            ...window.headers,
            'Content-Type': 'application/json',
        },
        method: "PUT"
    })
}

export { newGame, getGame, joinGame, startGame, enterGame, loadExistingGame, endTurn }