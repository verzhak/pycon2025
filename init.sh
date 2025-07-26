#!/usr/bin/sh

mkdir envs

python -m venv envs/pycon
source envs/pycon/bin/activate

pip install pylint

git clone --branch 0.11.8 https://github.com/astral-sh/ruff.git


