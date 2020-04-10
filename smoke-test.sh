#!/bin/sh

set -e

SVR=${SVR-https://ecs.eddsteel.com}
CRL="curl -s"

id=$($CRL -X POST $SVR/game | jq -r .id)

$CRL $SVR/game/$id | jq .

$CRL -X PUT $SVR/game/$id/alice
$CRL -X PUT $SVR/game/$id/bob
$CRL -X PUT $SVR/game/$id/charles

$CRL $SVR/game/$id | jq .

$CRL -X PUT $SVR/game/$id/alice -d '{"name": "alice", "state": "READY"}' -H 'Content-Type: application/json'
$CRL -X PUT $SVR/game/$id/bob -d '{"name": "bob", "state": "READY"}' -H 'Content-Type: application/json'
$CRL -X PUT $SVR/game/$id/charles -d '{"name": "charles", "state": "READY"}' -H 'Content-Type: application/json'

$CRL $SVR/game/$id | jq .
