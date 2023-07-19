#!/usr/bin/env python
import imageio as io
import sys
import os

def ppm2png(src_fpath):
    im = io.v3.imread(src_fpath)
    dst_fpath = src_fpath.replace(".ppm", ".png")
    io.v3.imwrite(dst_fpath, im)

if __name__ == '__main__':
    src_fpath = sys.argv[1]
    if os.path.isfile(src_fpath):
        ppm2png(src_fpath)
    elif os.path.isdir(src_fpath):
        for each in os.listdir(src_fpath):
            if each.endswith(".ppm"):
                ppm2png(os.path.join(src_fpath, each))