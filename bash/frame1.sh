#!/bin/bash

# 1 = framebackground
# 2 = user avatar
# 3 = user name
# 4 = repo description
# 5 = output
convert $1 $2 -gravity center -geometry +0-150  -composite -gravity center -font ./font/Nunito.ttf -pointsize 100 -fill black -annotate +0+200 $3 -pointsize 30 -annotate +0+300 "$4" -gravity Center -append $5
