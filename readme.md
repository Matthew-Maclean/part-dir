# part-dir

Partition a directory by size.

## Example use

    # split photos into 100mb-sized parts, but keep subfolders
    part-dir ~/photos ~/photos-parts -s=100mb

    # split videos into 1gb-sized parts, and also split subfolders
    part-dir ~/videos ~/videos-parts -s=1gb -r

# Usage

    part-dir INPUT OUTPUT -s <size> [-r | --recurse] [-p | --pack <packing>]

`INPUT`: The directory to partition.  
`OUTPUT`: The directory to copy parts to.  
`<size>`: The size of each part, a number followed by a suffix:

- no suffix, or suffix `b`: in bytes  
- suffix `k` or `kb`: in kilobytes  
- suffix `m` or `mb`: in megabytes  
- suffix `g` or `gb`: in gigabytes

`<packing>`: The packing option, higher levels may reduce the number of parts,
and make part sizes closer the the maximum. Levels:

- `none`: no packing (default)  
- `normal`: pack parts with items from other parts
- `tight`: sort parts by descending size before packing

This program only copies files, it will not delete any of the source files.
Please look at the source code before you trust this with anything important.
