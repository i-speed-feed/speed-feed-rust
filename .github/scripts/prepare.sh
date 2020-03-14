#!/bin/bash

# install balena
wget https://github.com/balena-io/balena-cli/releases/download/v11.29.3/balena-cli-v11.29.3-linux-x64-standalone.zip -O balena.zip
unzip balena.zip -d balena
balena/balena-cli/balena login --credentials --email ${BALENA_USER} --password ${BALENA_PW}

# install ookla
wget https://bintray.com/ookla/download/download_file?file_path=ookla-speedtest-1.0.0-x86_64-linux.tgz -O ookla.tgz
mkdir ookla
tar -xf ookla.tgz -C ookla