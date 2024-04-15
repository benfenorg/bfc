#!/bin/bash
read -p "Enter validator name:" name
read -p "Enter validator dns:" ip
read -p "Enter validator start port:" port

num1=2
port1=`expr $port`
port2=`expr $port1 + $num1`
port3=`expr $port2 + $num1`
port4=`expr $port3 + $num1`



../bfc genesis_ceremony add-validator \
--name "validator-$name" \
--validator-key-file "validator-$name.key" \
--worker-key-file "validator-$name-worker.key" \
--account-key-file "validator-$name-account.key" \
--network-key-file "validator-$name-network.key" \
--network-address "/dns/$ip/tcp/$port1/http" \
--p2p-address "/dns/$ip/udp/$port2" \
--narwhal-primary-address "/dns/$ip/udp/$port3" \
--narwhal-worker-address "/dns/$ip/udp/$port4" \
--description abc \
--image-url abc \
--project-url abc

echo "end of add validator"

