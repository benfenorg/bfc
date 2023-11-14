#!/bin/bash
echo "start generating....input your validator name:"
read -p "Enter your name:" name
sui keytool generate-with-name bls12381 "validator-$name"
sui keytool generate-with-name ed25519 "validator-$name-worker"
sui keytool generate-with-name ed25519 "validator-$name-account"
sui keytool generate-with-name ed25519 "validator-$name-network"

echo "key done"