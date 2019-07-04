#!/usr/bin/env python3

from os import environ

from matplotlib.pyplot import subplots
from pandas import read_csv

from viz import export, rink


def main():
    # $ cd $WD
    # $ cat sql/shots.sql | sql ruskie.db -csv > viz/data.csv
    data = read_csv("{}/viz/data.csv".format(environ["WD"]))
    data.sort_values("goal", inplace=True)
    _, ax = subplots(figsize=(5.5, 4.5))
    ax.scatter(
        data.x,
        data.y,
        c=data.goal.map({0: "c", 1: "tomato"}),
        s=25,
        alpha=0.75,
    )
    rink(ax)
    export(ax, "{}/viz/shots.png".format(environ["WD"]))


if __name__ == "__main__":
    main()