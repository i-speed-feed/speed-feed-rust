#!/bin/bash

whoami

wget https://bintray.com/ookla/download/download_file?file_path=ookla-speedtest-1.0.0-x86_64-linux.tgz
tar -xf speedtest.tgz

ls -la

mv ookla-speedtest-1.0.0-x86_64-linux/speedtest /usr/bin/speedtest