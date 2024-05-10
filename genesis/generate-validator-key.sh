#!/bin/bash
echo "start generating....input your validator name:"
read -p "Enter your name:" name
bfc keytool generate-with-name bls12381 "validator-$name" 
bfc keytool generate-with-name ed25519 "validator-$name-worker"
bfc keytool generate-with-name ed25519 "validator-$name-account"
bfc keytool generate-with-name ed25519 "validator-$name-network"

echo "key done"

