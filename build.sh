#!/bin/sh

set -ex

PI_HOST=pi@192.168.0.23
PROJECT_DIR=/home/pi/gruber
PI_TARGET=armv7-unknown-linux-musleabihf
FILES="gruber.service config.json target/$PI_TARGET/release/gruber"

cargo build --release --target $PI_TARGET
rsync -r -vv $FILES $PI_HOST:$PROJECT_DIR

if [ "$1" = "--release" ]; then
    echo "Starting systemd service..."
    ssh $PI_HOST << EOF
        sudo systemctl link $PROJECT_DIR/gruber.service
        sudo systemctl enable gruber
        sudo systemctl restart gruber
EOF
else
    echo "Running in dev mode..."
    # Run the program directly for testing
    ssh -t $PI_HOST "
        sudo systemctl stop gruber;
        cd ./gruber;
        RUST_BACKTRACE=1 startx ./gruber"
fi
