# hr16-extract

here is a quick and dirty utility for extracting sample data from raw sample ROM binaries, on alesis drum machines using the HR-16 sample format.

the tool is written in rust, assumes a linux environment, and requires `sox` to be available on the command line.

this page by the bending/music group **burnkit2600** helpfully described the ROM format:
http://www.burnkit2600.com/diy-sound-roms/

this tool works by parsing the ROM data, partitioning it into discrete samples, applying appropriate bit-shifts throughout each sample, and exporting 
the `sox` tool is then used to convert the new 16-bit `.raw` to `.wav` format. (yes, it would be nicer to just add the .wav header directly in this tool, but i am lazy.)

### usage:

`hr16-extract <ROM> <output folder> <output filename prefix>`

all arguments are required.
output folder is created if it doesn't exits.
each sample will appear as `output_<prefix>_<number>.wav` and also `output_<prefix>_<number>.raw`. 