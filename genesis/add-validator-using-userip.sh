#!/bin/bash
read -p "Enter validator name:" name
read -p "Enter validator ip:" ip
read -p "Enter validator start port:" port

num1=2
port1=`expr $port`
port2=`expr $port1 + $num1`
port3=`expr $port2 + $num1`
port4=`expr $port3 + $num1`



bfc genesis_ceremony add-validator \
--name "validator-$name" \
--validator-key-file "validator-$name.key" \
--worker-key-file "validator-$name-worker.key" \
--account-key-file "validator-$name-account.key" \
--network-key-file "validator-$name-network.key" \
--network-address "/ip4/$ip/tcp/$port1/http" \
--p2p-address "/ip4/$ip/udp/$port2" \
--narwhal-primary-address "/ip4/$ip/udp/$port3" \
--narwhal-worker-address "/ip4/$ip/udp/$port4" \
--description "Default Validator" \
--image-url abc \
--project-url "https://www.benfen.org/"

echo "end of add validator"
