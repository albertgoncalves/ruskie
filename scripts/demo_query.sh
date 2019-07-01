#!/usr/bin/env bash

cat $WD/sql/demo.sql | sqlite3 --column --header $WD/ruskie.db | less
