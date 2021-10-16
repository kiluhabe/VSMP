#!/usr/bin/env bash

set -e

export DBUS_SYSTEM_BUS_ADDRESS=unix:path=/host/run/dbus/system_bus_socket

iwgetid -r

if [ $? -eq 0 ]; then
    printf 'Skipping WiFi Connect\n'
else
    printf 'Starting WiFi Connect\n'
    ./wifi-connect
fi

VIDEO_DIR=$HOME/videos

if [ ! -f ./youtube-dl ]; then
    echo "youtube-dl not found."
    exit 1
fi

if [ ! -d $VIDEO_DIR ]; then
    mkdir $VIDEO_DIR
fi

VIDEO_NAME=$(./youtube-dl --get-filename -o '%(title)s.%(ext)s' $VIDEO_URL --restrict-filenames)

echo "Downloading $VIDEO_NAME ..."

./youtube-dl -o $VIDEO_DIR/$VIDEO_NAME $VIDEO_URL

echo "Download finished."

echo "==== VSMP running options===="
echo "video: $VIDEO_DIR/$VIDEO_NAME"
echo "height: $HEIGHT"
echo "width: $WIDTH"
echo "fps: $FPS"
echo "interval: $INTERVAL"
echo "============================="

exec "$@ -h $HEIGHT -w $WIDTH -f $FPS -i $INTERVAL $VIDEO_DIR/$VIDEO_NAME"
