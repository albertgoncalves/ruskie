#!/usr/bin/env python3

from os import environ

from matplotlib.pyplot import close, savefig, subplots, tight_layout
from pandas import read_csv


def data():
    X = read_csv(
        "{}/model/data/gen.csv".format(environ["WD"]),
        names=["_", "x", "y", "l", "r"],
    )
    Y = read_csv("{}/model/out/pred.csv".format(environ["WD"]), names=["z"])
    X["z"] = Y.z
    rows = X["l"] == 1.0
    return {
        "left": X.loc[rows],
        "right": X.loc[~rows],
    }


def plot(lr):
    _, axs = subplots(2, 1, figsize=(5, 10))
    for (i, k) in enumerate(lr.keys()):
        axs[i].tricontourf(lr[k].x, lr[k].y, lr[k].z)
        axs[i].set_aspect("equal")
        axs[i].set_title(k)
    tight_layout()
    savefig("{}/model/out/plot.png".format(environ["WD"]))
    close


def main():
    plot(data())


if __name__ == "__main__":
    main()
