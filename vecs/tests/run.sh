#!/usr/bin/env bash

echo "Assuming I'm on /vecs/tests/"

gcc -g3 -fsanitize=address -I../include/ -L../../target/debug/ -lvecs test.c
LD_LIBRARY_PATH="../../target/debug" ./a.out
