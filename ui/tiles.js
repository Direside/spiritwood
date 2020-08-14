'use strict';

const nextTileImage = document.getElementById('next-tile-image');

function getNextTile() {
    fetch(`${window.backend}/game/${window.gameID}/tile`, {
        headers: window.headers,
    })
    .then(r => {
        return r.json();
    })
    .then(data => {
        window.tile = data
        nextTileImage.innerHTML = `<img src='${data.image}' ondragstart='dragStart(event, ${data.id}, ${data.image})' height='80 width='80' />`;
    });
}

async function getBoardTiles(x, y, radius) {
    return await fetch(`${window.backend}/game/${window.gameID}/tiles?x=${x}&y=${y}&radius=${radius}`, {
        headers: window.headers,
        method: "GET"
    }).then(r => {
        const data = r.json();
        console.log(data);
        return data;
    });
}

async function placeTile(x, y, tileId, rotation) {
    x = parseInt(x);
    y = parseInt(y);
    tileId = parseInt(tileId);

    await fetch(`${window.backend}/game/${window.gameID}/placetile`, {
        headers: {
            ...window.headers,
            'Content-Type': 'application/json',
        },
        method: "PUT",
        body: JSON.stringify({
            x,
            y,
            tile: tileId,
            rotation
        }),
    });
}

const updateLocalTile = () => {
    let el = document.getElementById(`board-${window.x}-${window.y}`);
    let r = window.rotation * 90;
    console.log("Updating rotation: ", window.rotation)
    switch (r) {
        case 0:
            el.classList.remove('rotate-90');
            el.classList.remove('rotate-180');
            el.classList.remove('rotate-270');
            break;
        case 90:
            el.classList.remove('rotate-0');
            el.classList.remove('rotate-180');
            el.classList.remove('rotate-270');
            break;
        case 180:
            el.classList.remove('rotate-0');
            el.classList.remove('rotate-90');
            el.classList.remove('rotate-270');
            break;
        case 270:
            el.classList.remove('rotate-0');
            el.classList.remove('rotate-90');
            el.classList.remove('rotate-180');
            break;
    }
    el.classList.add(`rotate-${r}`);
    el.style = `background-image: url(${window.tile.image})`;
    console.log("UDAPTE", window.tile.image)
}

function rotateLeft() {
    window.rotation = (window.rotation + 3) % 4;
    updateLocalTile();
}

function rotateRight() {
    window.rotation = (window.rotation + 1) % 4;
    updateLocalTile();
}

function drag(event, tileId, image) {
    current = event.dataTransfer.setData("tile_id", tileId);
}

async function drop(event) {
    event.preventDefault()

    console.log(`Drop event: %o`, event);

    const data = event.target.dataset
    const x = data.x
    const y = data.y
    window.x = x
    window.y = y
    window.rotation = 0

    let handleDrop = document.getElementById("submit-tile-button")
    handleDrop.onclick = async function() { await window.placeTile(x, y, window.tile.id, window.rotation); };
    
    event.target.classList.remove("over")
    updateLocalTile();
}

function over(event) {
    event.preventDefault()
}

function enter(event) {
    event.target.classList.add("over")
}

function leave(event) {
    event.target.classList.remove("over")
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
    let tiles = window.tiles;
    const { left, right, top, bottom } = calcBoundingBox(window.xs, window.ys);
    let diameter = right - left;
    if (bottom - top > diameter) {
        diameter = bottom - top;
    }
    const radius = diameter / 2;
    const x = left + radius + (diameter % 2);
    const y = top + radius + (diameter % 2);

    const placedTiles = await window.getBoardTiles(x, y, radius);
    console.log("Placed Tiles: ", placedTiles)
    for (const placedTile of placedTiles) {
        if (!tiles[placedTile.x]) {
            tiles[placedTile.x] = [];
        }

        // TODO: Change structure on the window.backend to make this no longer required.
        placedTile.tile.rotation = placedTile.rotation;

        tiles[placedTile.x][placedTile.y] = placedTile.tile;
    }
};

const getTile = (x, y) => {
    let tiles = window.tiles;
    const tile = (tiles[x] || [])[y]
    if (tile) {
        return "background-image: url(" + tile.image + ")"
    } else {
        return ""
    }
}

const getRotation = (x, y) => {
    const tile = (tiles[x] || [])[y]
    console.log("TILE: ", tile)
    if (tile && tile.hasOwnProperty('rotation')) {
        return tile.rotation * 90
    } else {
        return 0
    }
}

export { getNextTile, getBoardTiles, updateLocalTile, placeTile, rotateLeft, rotateRight, drag, drop, over, enter, leave, fetchTiles, getTile, getRotation }