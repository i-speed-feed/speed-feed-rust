#!/bin/bash

wget https://bintray.com/ookla/download/download_file?file_path=ookla-speedtest-1.0.0-x86_64-linux.tgz -O ookla.tgz
mkdir ookla
tar -xf ookla.tgz -C ookla

mv ookla/speedtest /usr/bin/speedtest

speedtest