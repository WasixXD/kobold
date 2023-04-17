#!/bin/bash

# 1 = Repo name
# 2 = Repo desc
# 3 = Coded with
# 4 = Created at
# 5 = organization
# 6 = output


convert kobold.jpg -font font/Nunito.ttf -gravity center -pointsize 100 -annotate +0-450 $1 -pointsize 30 -annotate +0-375 "$2" -pointsize 50 -annotate -300-100 'CODED WITH' -fill white -pointsize 30 -annotate -300-50 "$3" -fill black -pointsize 50 -annotate +300-100 'CREATED AT' -pointsize 30 -fill white -annotate +300-50 "$4" -fill black -pointsize 50 -annotate +0+200 'ORGANIZATION' -fill white -pointsize 30 -annotate +0+250 "$5" $6
