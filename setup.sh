#! /bin/bash

sudo mkdir /usr/share/rnms
sudo cp -r src/scripts/ /usr/share/rnms
sudo cp target/release/rnms /usr/bin/