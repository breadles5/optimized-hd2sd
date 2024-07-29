# optimized-hd2sd
Simple app designed to take osu skin HD textures and create SD textures from them.   
 - Finds all HD textures (denoted with `@2x` in file name) in the current selected directory and all subdirectories.   
 - Creates new SD tectures with bicubic filtering.   
 - Losslessly optimizes both input and output file sizes.
 - By default, IT WILL OVERWRITE EXISTING NON-`@2x` IMAGES UNLESS YOU FOLLOW THE INSTRUCTIONS BELOW
## preserving original scaled (non-`@2x`) images
Use the `-p` or `--preserve` cli flag if running from terminal.   
For a more gui based solution apply the following steps:
1. create shortcut to optimized-hd2sd.exe
2. open properties
3. add ` -p` or ` --preserve` to the target

It should look something like this:   
target: `C:\path\to\optimized-hd2sd.exe -p`   

replace with the actual path to optimized-hd2sd.exe   
The space matters since it will work as a cli flag