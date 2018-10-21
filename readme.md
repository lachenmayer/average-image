# average-image

Averages all of the passed in images in HSL space.

Usage: `average-image <images.jpg>+`

Saves the resulting file as `out.jpg`. Doesn't check whether `out.jpg` exists already. The output file will have the same dimensions as the first file passed in.

This is essentially just a quick shell script, so it doesn't do any error checking or anything else you might expect from a fully-formed tool. It also doesn't deal with floating-point error, but that doesn't really matter.