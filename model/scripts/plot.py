#!/usr/bin/env python3

from os import environ

from matplotlib.pyplot import subplots
from pandas import read_csv

from viz import export, rink


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
        axs[i].tricontourf(lr[k].x, lr[k].y, lr[k].z, cmap="Oranges", alpha=0.5)
        axs[i].set_title(k)
        rink(axs[i], zorder=3)
    export("{}/model/out/plot.png".format(environ["WD"]))


def main():
    plot(data())


if __name__ == "__main__":
    main()
