#!/bin/bash

cd testing && cargo r -r && cd ../ && python3 plot.py
