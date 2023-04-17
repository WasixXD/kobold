
#!/bin/bash
# 1 = user name
# 2 = repo description
# 3 = stars count
# 4 = issues count
# 5 = output
# 6 = confirm or cancel
convert kobold.jpg -font font/Nunito.ttf -gravity center -pointsize 100 -annotate +0-450 "$1" -pointsize 30 -annotate +0-375 "$2" ./image/star.png -geometry -300-100 -composite -matte -pointsize 30 -annotate -300-95 $3 -pointsize 50 -annotate -300+25 'STARS'  ./image/ellipse.png -geometry +300-100 -composite -matte -pointsize 30 -annotate +300-95 $4 -pointsize 50 -annotate +300+25 'ISSUES' $6 -geometry -0+300 -composite -matte -pointsize 50 -annotate -0+425 'HAS A PAGE' -append $5 
