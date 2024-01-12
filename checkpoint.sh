#!/bin/bash

infix=$(date '+%F_%T')
output="/tmp/chess-checkpoint-$infix.tar"
tar \
    --exclude='vendor' \
    --exclude='chess-checkpoint-*' \
    --exclude='backup/*' \
    --exclude='.git' \
    --exclude='target' \
    -zcvf \
    $output .
mv $output backup/
