#!/bin/sh


if [ -d "run" ]; then
    rm -r run/
    mkdir run
else
    mkdir run
fi