#!/bin/bash
sudo install_name_tool -add_rpath /usr/local/lib ./target/debug/bfc
sudo install_name_tool -add_rpath /usr/local/lib ./target/debug/bfc-node


sudo install_name_tool -add_rpath /usr/local/lib ./target/release/bfc-node