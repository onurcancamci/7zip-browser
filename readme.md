# 7zip-browser

This is command line utility for browsing and partially extracting archives. It is a wrapper around p7zip with nodejs.

## Requirements

- Nodejs (tested with v14.6)
- p7zip (>= v16.02)
    - 7z binary should be in your path

## How To Run

- Install dependencies `npm install`
- Compile with `npm run build`
- Run with `./run.sh -i <input file> -o <output directory>` 

## How It Works

Firstly you need to run it with input file and output directory. When this program starts extracting, it extracts inside given directory.  
After running, move the cursor with k/j or up/down arrow keys. Enter for opening directories and marking files. "m" is also used for marking files but it can mark folders as well. Press "e" to extract marked files into pre-determined output directory. After extracting program exits.
