#!/usr/bin/env python3

from sys import argv

from numpy import arange, array, concatenate, flip, meshgrid, repeat, \
    tile, vstack
from pandas import DataFrame


def main():
    params = {
        "min_x": 0,
        "max_x": 100,
        "min_y": -45,
        "max_y": 45,
        "i": 1.0,
    }
    x = arange(params["min_x"], params["max_x"] + params["i"], params["i"])
    y = arange(params["min_y"], params["max_y"] + params["i"], params["i"])
    xy = array(meshgrid(x, y)).T.reshape(-1, 2)
    ls = repeat([0, 1], xy.shape[0])
    rs = flip(ls.copy())
    zs = repeat([0], ls.shape[0])
    data = DataFrame(concatenate(
        (tile(xy, (2, 1)), vstack((ls, rs, zs)).T),
        axis=1,
    ))
    data[4] = data[4].astype(int)
    data[[4, 0, 1, 2, 3]].to_csv(argv[1], index=False, header=False)


if __name__ == "__main__":
    main()
