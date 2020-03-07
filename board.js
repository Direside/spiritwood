// Get number of players
var url = new URL(window.location.href);
let players = url.searchParams.get("p");
console.log(players)

let main = document.getElementById("board");

let height = 10;
let width = 15;
let bits = "36px"

main.style.setProperty("--width", width);
main.style.setProperty("--height", height);
main.style.setProperty("--bits", bits)


const xs = Array.from({length: width}, (_, i) => i)
const ys = Array.from({length: height}, (_, i) => i)

const images = [
    "images/north-south-east.png",
    "images/south-east-west.png",
    "images/east-west.png",
    "images/south-east.png"
]

var current = ""
const tiles = []
tiles[15] =  []
tiles[15][15] = "/images/heart-1.jpg"

window.dragStart = function dragStart(event) {
    current = event.target.dataset.url
}

window.drop = function hmm (event) {
    event.preventDefault()
    const data = event.target.dataset
    const x = data.x
    const y = data.y
    tiles[x] = tiles[x] || []
    tiles[x][y] = tiles[x][y] || ""
    tiles[x][y] =  current
    current = ""
    event.target.classList.remove("over")
    paint()
}

window.over = function over (event) {
    event.preventDefault()
}

window.enter = function enter(event) {
    event.target.classList.add("over")
}

window.leave = function leave(event) {
    event.target.classList.remove("over")
}

const getTile = (x,y) => {
    const url = (tiles[x] || [])[y]
    if (url) {
        return "background-image: url(" + url + ")"
    } else {
        return ""
    }
}

function paint() {
    let board = "";

    for (let y of ys) {
        for (let x of xs) {
            board += `<div class="board-cell" data-x="${x}" data-y="${y}" ondragover="over(event)" ondrop="drop(event)" ondragenter="enter(event)" ondragleave="leave(event)" style="${getTile(x,y)}"></div>`
        }
    }

    main.innerHTML = board;
}

paint()

const palette = document.createElement("div")
palette.classList.add("palette")
for (let img of images) {
    const paletteEl = document.createElement("img")
    paletteEl.src= img
    paletteEl.style.width = "36px";
    paletteEl.style.height = "36px";
    paletteEl.classList.add("palette-art")
    paletteEl.draggable = true;
    paletteEl.dataset.url = img
    paletteEl.ondragstart = window.dragStart
    palette.appendChild(paletteEl)
}
document.getElementById("hand").appendChild(palette)