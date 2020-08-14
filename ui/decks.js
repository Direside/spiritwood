'use strict';

// Cards
let skel = {
    name: "skeleton",
    image: "skell.png",
    description: "Kills you. Roll 3+ to escape."
}

// Graveyard
window.gy = [];

window.gy = addToDeck(window.gy, "skeleton", 18);
window.gy = addToDeck(window.gy, "zombie", 12);
window.gy = addToDeck(window.gy, "dracula", 4);
window.gy = addToDeck(window.gy, "sword", 2);
window.gy = addToDeck(window.gy, "shield", 3);
window.gy = addToDeck(window.gy, "potions", 10);
window.gy = addToDeck(window.gy, "nothing", 25);

window.gy = shuffle(window.gy);

// Pile of Bones
window.bone = [];

window.bone = addToDeck(window.bone, "key", 2);
window.bone = addToDeck(window.bone, "leg bone", 7);
window.bone = addToDeck(window.bone, "spell", 4);
window.bone = addToDeck(window.bone, "nothing", 5);
window.bone = addToDeck(window.bone, "portal", 1);

window.bone = shuffle(window.bone);
