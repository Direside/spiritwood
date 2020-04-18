#!/bin/sh

set -e

SVR=${SVR-https://ecs.eddsteel.com}
CRL="curl -s"

id=$($CRL -X POST $SVR/game | jq -r .id)

$CRL $SVR/game/$id | jq .

$CRL -X PUT $SVR/game/$id?player=alice
$CRL -X PUT $SVR/game/$id?player=bob
$CRL -X PUT $SVR/game/$id?player=charles

$CRL $SVR/game/$id | jq .

$CRL -X PUT $SVR/game/$id/start

$CRL $SVR/game/$id | jq .
