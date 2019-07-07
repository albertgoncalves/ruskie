#!/usr/bin/env python3

from sys import argv


def transform(csv, libsvm):
    for x in csv:
        xs = x.split(",")
        libsvm.write(str(int(xs[0])))
        for i in range(len(xs) - 1):
            y = float(xs[i + 1])
            if y != 0.0:
                libsvm.write(" {}:{}".format(i, y))
        libsvm.write("\n")


def main():
    with open(argv[2], "w") as libsvm:
        with open(argv[1], "r") as csv:
            transform(csv, libsvm)


if __name__ == "__main__":
    main()
