#!/usr/bin/env python3

from os import environ

from matplotlib.pyplot import close, savefig, subplots, tight_layout
from pandas import read_csv


class Plot:
    def setup(data):
        fig, ax = subplots(figsize=(12, 6))
        ax.scatter(data.x, data.y, s=15, alpha=0.2)
        return fig, ax

    def background(ax):
        for f in [ax.axvline, ax.axhline]:
            f(0, c="k", lw=0.5)

    def aspect(ax):
        ax.set_xlim([-100, 100])
        ax.set_ylim([-45, 45])
        ax.set_aspect("equal")
        tight_layout()

    def export(ax, filename):
        savefig(filename)
        close()


def main():
    _, ax = Plot.setup(read_csv("{}/viz/data.csv".format(environ["WD"])))
    Plot.background(ax)
    Plot.aspect(ax)
    Plot.export(ax, "{}/viz/shots.png".format(environ["WD"]))


if __name__ == "__main__":
    main()
