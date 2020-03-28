

function addToDeck(deck, card, times = 1) {
    let c = Array.from({length: times}, _ => card)
    return deck.concat(c);
}

function drawCard(deck, discard) {
    let topCard = deck.pop();

    if (discard) {
        discard.push(topCard);
    }
    return topCard;
}

function shuffle2(arr = [], result = []) {
    if (!arr.length) return result
    const index = ~~(Math.random() * arr.length)
    result.push(arr[index])
    arr.splice(index,1)
    return shuffle2(arr, result)
}

// http://stackoverflow.com/questions/2450954/how-to-randomize-shuffle-a-javascript-array
function shuffle(array) {
    var currentIndex = array.length
        , temporaryValue
        , randomIndex
        ;

    // While there remain elements to shuffle...
    while (0 !== currentIndex) {

        // Pick a remaining element...
        randomIndex = Math.floor(Math.random() * currentIndex);
        currentIndex -= 1;

        // And swap it with the current element.
        temporaryValue = array[currentIndex];
        array[currentIndex] = array[randomIndex];
        array[randomIndex] = temporaryValue;
    }

    return array;
}

export {shuffle, shuffle2, addToDeck, drawCard};