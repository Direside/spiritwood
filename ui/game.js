'use strict';

import { getCookie } from './cookie.js'

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
            console.log(data);
            window.gameID = data.id
            console.log(window.gameID)
            window.joinGame()
        });
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

            let playersTurn = document.getElementById('players-turn')
            playersTurn.innerText = window.players[0];
        })
}

function joinGame() {
    window.playerName = getCookie("playerName")

    if (!window.gameID) {
        window.gameID = prompt("Game ID")
    }

    document.getElementById("debugGameID").innerText = window.gameID
    fetch(`${window.backend}/game/${window.gameID}?player=${window.playerName}`, {
        method: "PUT"
    }).then((data) => {
        window.playerID = data.order;
        window.headers = new Headers({
            "Authorization": `Bearer ${data.key}`
        });
        window.readyScreen();
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
    }).then((r) => {
        return r.json();
    }).then(data => {
        enterGame();
    })
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