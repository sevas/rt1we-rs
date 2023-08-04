#!/bin/zsh

cd ../out || exit
ffmpeg  -framerate 24 -i anim_image_%05d.ppm -vcodec libx264 -pix_fmt yuv420p output.mov
