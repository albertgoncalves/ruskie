#!/usr/bin/env python3

from matplotlib import lines, patches
from matplotlib.pyplot import close, savefig, tight_layout
from numpy import arange, array, concatenate, cos, radians, sin


def curve(r, degree):
    theta = radians(degree)
    return (r * sin(theta), r * cos(theta))


def unit_boards():
    params = {
        "min_x": -1,
        "min_y": 0,
        "max": 4.5,
    }
    params["delta_y"] = params["max"] - params["min_y"]
    lower_x, lower_y = curve(1, arange(90, 180, 1))
    upper_x, upper_y = curve(1, arange(0, 90, 1))
    xs = concatenate(
        [
            array([params["min_x"]]),
            upper_x + params["max"] - 1,
            array([params["max"]]),
            lower_x + params["max"] - 1,
            array([params["min_x"]]),
        ],
        axis=None,
    )
    ys = concatenate(
        [
            array(params["max"]),
            upper_y + params["max"] - 1,
            array([(params["delta_y"] / 2) + params["min_y"]]),
            lower_y + params["min_y"] + 1,
            array([params["min_y"]]),
        ],
        axis=None,
    )
    return (xs / params["delta_y"], ys / params["delta_y"])


def rink(ax):
    params = {
        "blueline_x": 29,
        "boards_y": 45,
        "boardscurve_x": 77.45,
        "boardsmin_x": -5,
        "boardsmax_x": 100,
        "centerline": 0,
        "faceoff_radius": 15,
        "faceoff_x": 69,
        "faceoff_y": 22,
        "goal_x": 87,
        "goal_y": -3,
        "goal_width": 2,
        "goal_height": 6,
        "goalline_y": 43,
        "pad": 1,
    }
    params["boardspad_y"] = params["boards_y"] - (params["pad"] * 0.75)
    kwargs = {"alpha": 0.25, "zorder": 0}
    ax.set_xlim([
        params["boardsmin_x"] - params["pad"],
        params["boardsmax_x"] + params["pad"],
    ])
    ax.set_ylim([
        (params["boards_y"] + params["pad"]) * -1,
        params["boards_y"] + params["pad"],
    ])
    boards_xs, boards_ys = unit_boards()
    ax.plot(
        boards_xs * params["boardsmax_x"],
        (boards_ys * params["boards_y"] * 2) - params["boards_y"],
        lw=3.5,
        c="k",
        **kwargs,
    )
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
    xs = [
        lines.Line2D(
            [params["goal_x"], params["goal_x"]],
            [params["goalline_y"] * -1, params["goalline_y"]],
            c="r",
            lw=2,
            **kwargs,
        ),
        lines.Line2D(
            [params["centerline"], params["centerline"]],
            [params["boardspad_y"] * -1, params["boardspad_y"]],
            c="r",
            lw=7,
            **kwargs,
        ),
        lines.Line2D(
            [params["blueline_x"], params["blueline_x"]],
            [params["boardspad_y"] * -1, params["boardspad_y"]],
            c="b",
            lw=7,
            **kwargs,
        ),
        lines.Line2D(
            [params["boardsmin_x"], params["boardsmax_x"]],
            [params["centerline"], params["centerline"]],
            c="k",
            lw=1.5,
            ls="--",
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
