import imageio as io
import sys

if __name__ == '__main__':
    src_fpath = sys.argv[1]
    im = io.v3.imread(src_fpath)
    dst_fpath = src_fpath.replace(".ppm", ".png")
    io.v3.imwrite(dst_fpath, im)