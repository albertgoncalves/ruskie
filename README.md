# ruskie

![](cover.png)

Needed things
---
 * [Nix](https://nixos.org/nix/)

Quick start
---
```
$ ./shell
[nix-shell:path/to/ruskie]$ cd dev/
[nix-shell:path/to/ruskie/dev]$ ./main
[nix-shell:path/to/ruskie/dev]$ cd ../viz/
[nix-shell:path/to/ruskie/viz]$ cat ../sql/shots.sql | sql ../ruskie.db -csv > data.csv
[nix-shell:path/to/ruskie/viz]$ python main.py && open shots.png
```
