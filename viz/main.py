#!/usr/bin/env python3

from os import environ

from matplotlib import lines, patches
from matplotlib.pyplot import close, savefig, subplots, tight_layout
from pandas import read_csv


class Plot:
    def setup(data):
        _, ax = subplots(figsize=(5.5, 4.5))
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
        params = {
            "faceoff_x": 69,
            "faceoff_y": 22.475,
            "faceoff_radius": 15,
            "blueline_x": 29,
            "centerline_x": 0,
            "centerline_y": 0,
            "goal_x": 87,
            "goal_y": -3,
            "goal_width": 2,
            "goal_height": 6,
            "boardscurve_x": 77.45,
            "boardsmin_x": -5,
            "boardsmax_x": 100,
            "boards_y": 45,
            "goalline_y": 42.6,
        }
        kwargs = {"alpha": 0.25, "zorder": 0}
        ax.set_xlim([params["boardsmin_x"], 100])
        ax.set_ylim([params["boards_y"] * -1, params["boards_y"]])
        ax.axvline(params["centerline_x"], c="r", lw=7, **kwargs)
        ax.axhline(params["centerline_y"], c="k", lw=1.5, ls="--", **kwargs)
        ax.axvline(params["blueline_x"], c="b", lw=7, **kwargs)
        ax.add_patch(patches.Rectangle(
            (params["goal_x"], params["goal_y"]),
            params["goal_width"],
            params["goal_height"],
            color="k",
            **kwargs,
        ))
        for (z, angle) in [(-1, 0), (1, 90)]:
            ax.add_patch(patches.Circle(
                (params["faceoff_x"], params["faceoff_y"] * z),
                params["faceoff_radius"],
                lw=2,
                fill=None,
                **kwargs,
            ))
            ax.add_patch(patches.Arc(
                [params["boardscurve_x"], params["faceoff_y"] * z],
                params["boards_y"],
                params["boards_y"],
                angle=angle,
                theta1=270,
                theta2=0,
                lw=0.5,
            ))
        xs = [
            lines.Line2D(
                [params["boardsmin_x"], params["boardscurve_x"]],
                [params["boards_y"], params["boards_y"]],
                c="k"
            ),
            lines.Line2D(
                [params["boardsmin_x"], params["boardscurve_x"]],
                [params["boards_y"] * -1, params["boards_y"] * -1],
                c="k"
            ),
            lines.Line2D(
                [params["boardsmax_x"], params["boardsmax_x"]],
                [params["faceoff_y"] * -1, params["faceoff_y"]],
                c="k",
            ),
            lines.Line2D(
                [params["goal_x"], params["goal_x"]],
                [params["goalline_y"] * -1, params["goalline_y"]],
                c="r",
                lw=2,
                **kwargs,
            ),
        ]
        for x in xs:
            ax.add_line(x)

    def export(ax, filename):
        ax.set_xticks([])
        ax.set_yticks([])
        ax.axis("off")
        ax.set_aspect("equal")
        tight_layout()
        savefig(filename)
        close()


def main():
    # $ cd $WD
    # $ cat sql/shots.sql | sql ruskie.db -csv > viz/data.csv
    ax = Plot.setup(read_csv("{}/viz/data.csv".format(environ["WD"])))
    Plot.background(ax)
    Plot.export(ax, "{}/viz/shots.png".format(environ["WD"]))


if __name__ == "__main__":
    main()
