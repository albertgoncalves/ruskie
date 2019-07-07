#!/usr/bin/env python3

from sys import argv

from numpy import arccos, degrees, sqrt
from pandas import read_csv


def distance(ax, ay, bx, by):
    return sqrt(((ax - bx) ** 2) + ((ay - by) ** 2))


def angle(bx, by, ax, ay, cx, cy):
    a = distance(bx, by, cx, cy)
    b = distance(ax, ay, cx, cy)
    c = distance(ax, ay, bx, by)
    return degrees(arccos(((a ** 2) + (c ** 2) - (b ** 2)) / (2 * a * c)))


def aperture(x, y):
    params = {
        "x": 87,
        "min_y": -3,
        "max_y": 3,
    }
    return angle(
        x,
        y,
        params["x"],
        params["min_y"],
        params["x"],
        params["max_y"],
    )


def main():
    data = read_csv(argv[1], names=["z", "x", "y", "l", "r"])
    data["a"] = aperture(data.x, data.y).fillna(0.0)
    data.loc[data.x > 87, "a"] = 0.0
    data.to_csv(argv[1], index=False, header=None)


if __name__ == "__main__":
    main()
