#!/bin/bash


ffmpeg -framerate 1/4 -i ./image/frame%d.png -vcodec libx264 -r 30 -pix_fmt yuv420p $1
