#!/usr/bin/env python3

from os import environ

from matplotlib.pyplot import close, savefig, subplots, tight_layout
from pandas import read_csv


def data():
    X = read_csv(
        "{}/model/data/gen_data.csv".format(environ["WD"]),
        names=["_", "x", "y", "l", "r"],
    )
    Y = read_csv("{}/model/out/preds.txt".format(environ["WD"]), names=["z"])
    X["z"] = Y.z
    rows = X["l"] == 1.0
    return (X.loc[rows], X.loc[~rows])


def plot(a, b):
    _, axs = subplots(2, 1, figsize=(5, 10))
    axs[0].tricontourf(a.x, a.y, a.z)
    axs[1].tricontourf(b.x, b.y, b.z)
    for i in range(2):
        axs[i].set_aspect("equal")
    tight_layout()
    savefig("{}/model/out/plot.png".format(environ["WD"]))
    close


def main():
    plot(*data())


if __name__ == "__main__":
    main()
