#!/bin/bash

#path
read -p "Enter network.yaml path:" path
sed -i 's/127.0.0.1/0.0.0.0/g' $path