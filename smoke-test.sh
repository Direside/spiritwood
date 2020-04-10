#!/bin/sh

set -e

SVR=https://ecs.eddsteel.com
CRL="curl -s"

id=$($CRL -X POST $SVR/game | jq -r .id)

$CRL $SVR/game/$id | jq .

$CRL -X PUT $SVR/game/$id/edd
$CRL -X PUT $SVR/game/$id/shane
$CRL -X PUT $SVR/game/$id/julie

$CRL $SVR/game/$id | jq .

$CRL -X PUT $SVR/game/$id/edd/ready
$CRL -X PUT $SVR/game/$id/shane/ready
$CRL -X PUT $SVR/game/$id/julie/ready

$CRL $SVR/game/$id | jq .
