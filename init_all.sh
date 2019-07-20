#!/usr/bin/env bash

for p in param3.yaml param4.yaml
do
    sudo ./target/debug/init_dev enp3s0 $p
done
sleep 1
sudo ./send_external_trig.py

