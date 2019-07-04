#!/usr/bin/env python3

from os import environ

from matplotlib import patches
from matplotlib.pyplot import close, savefig, subplots, tight_layout
from pandas import read_csv


class Plot:
    def setup(data):
        _, ax = subplots(figsize=(6.5, 5.5))
        data.sort_values("goal", inplace=True)
        ax.scatter(
            data.x,
            data.y,
            c=data.goal.map({0: "c", 1: "tomato"}),
            s=25,
            alpha=0.75,
        )
        return ax

    def background(ax):
        kwargs = {"alpha": 0.25, "zorder": 0}
        ax.axvline(0, c="r", lw=7, **kwargs)
        ax.axvline(29, c="b", lw=7, **kwargs)
        ax.axhline(0, c="k", lw=1.5, **kwargs)
        for xy in [(69, 22), (69, -22)]:
            ax.add_patch(patches.Circle(
                xy,
                15,
                lw=2,
                fill=None,
                **kwargs,
            ))
        ax.add_patch(patches.Rectangle(
            (87, -3),
            2,
            6,
            color="k",
            **kwargs,
        ))

    def aspect(ax):
        ax.set_xlim([-15, 100])
        ax.set_ylim([-45, 45])
        ax.set_xticks([])
        ax.set_yticks([])
        ax.set_aspect("equal")
        tight_layout()

    def export(ax, filename):
        savefig(filename)
        close()


def main():
    # $ cd $WD
    # $ cat sql/shots.sql | sql ruskie.db -csv > viz/data.csv
    ax = Plot.setup(read_csv("{}/viz/data.csv".format(environ["WD"])))
    Plot.background(ax)
    Plot.aspect(ax)
    Plot.export(ax, "{}/viz/shots.png".format(environ["WD"]))


if __name__ == "__main__":
    main()
