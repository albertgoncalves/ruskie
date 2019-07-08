#!/usr/bin/env python3

from os import environ

from matplotlib.pyplot import subplots
from pandas import read_csv

from viz import export, rink


def data():
    X = read_csv(
        "{}/model/data/gen.csv".format(environ["WD"]),
        names=["_", "x", "y", "l", "r", "a"],
    )
    Y = read_csv("{}/model/out/pred.csv".format(environ["WD"]), names=["z"])
    X["z"] = Y.z
    rows = X["l"] == 1.0
    return {
        "left": X.loc[rows],
        "right": X.loc[~rows],
    }


def plot(lr):
    fig, axs = subplots(3, 1, figsize=(4, 13))
    axs[0].tricontourf(
        lr["left"].y,
        lr["left"].x,
        lr["left"].a,
        cmap="PuBuGn",
    )
    axs[0].set_title("aperture")
    rink(axs[0])
    for (i, k) in enumerate(lr.keys()):
        axs[i + 1].tricontourf(
            lr[k].y,
            lr[k].x,
            lr[k].z,
            cmap="Reds",
            alpha=0.8,
        )
        axs[i + 1].set_title(k)
        rink(axs[i + 1])
    export("{}/model/out/model.png".format(environ["WD"]))


def main():
    plot(data())


if __name__ == "__main__":
    main()
