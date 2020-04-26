#!/bin/sh

set -e

#SVR=${SVR-https://ecs.eddsteel.com}
SVR="${SVR-http://localhost:8000}"
CRL="curl -s"

echo "Create a game"
id=$($CRL -X POST $SVR/game | jq -r .id)
echo "$id"

$CRL $SVR/game/$id | jq .

echo "Join the game"
$CRL -X PUT $SVR/game/$id?player=alice | jq .
$CRL -X PUT $SVR/game/$id?player=bob | jq .
$CRL -X PUT $SVR/game/$id?player=charles | jq .

echo "Check who's in the game"
$CRL $SVR/game/$id | jq .

echo "Start the game"
$CRL -X PUT $SVR/game/$id/start | jq .

echo "Check the game state"
$CRL $SVR/game/$id | jq .

echo "Get a tile"
TILE=$($CRL $SVR/game/$id/tile)
echo "$TILE" | jq .

echo "Check for missing map tile"
$CRL "$SVR/game/$id/tiles?x=3&y=5" | jq .

echo "Place a tile on the map"
$CRL -X PUT $SVR/game/$id/tiles/3/5 --data "$($CRL $SVR/game/$id/tile | jq .)" | jq .

echo "Check the tile was placed"
$CRL "$SVR/game/$id/tiles?x=3&y=5" | jq .

echo "Place a tile on a spot that's not empty"
$CRL -X PUT $SVR/game/$id/tiles/3/5 --data "$($CRL $SVR/game/$id/tile | jq .)" | jq .

echo "End turn"
$CRL -X PUT $SVR/game/$id/endturn | jq .
