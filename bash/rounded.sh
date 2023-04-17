#!/bin/bash
 magick $1 -alpha set \
    \( +clone -distort DePolar 0 \
       -virtual-pixel HorizontalTile -background None -distort Polar 0 \) \
    -compose Dst_In -composite -trim +repage $2

