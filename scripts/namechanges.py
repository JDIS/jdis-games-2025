from pathlib import Path
import json
import argparse
import matplotlib.pyplot as plt
import matplotlib.dates as pltdates
import numpy as np
import datetime

def gen_changes_from_history(history_folder: Path):
    history = dict()
    for save in history_folder.iterdir():
        timestamp = int(save.stem.removeprefix("save_"))
        with save.open("rb") as f:
            history[timestamp] = json.load(f)["players"]

    timestamps = sorted(history.keys())
    data = [history[t] for t in timestamps]
    teams = list({token for save in data for token in save})
    initial = data[0]

    out = [[initial[t]["name"]] for t in teams]
    for save in data:
        for i, team in enumerate(teams):
            if save[team]["name"] != out[i][-1]:
                out[i].append(save[team]["name"])
    return "\n".join(" -> ".join(l) for l in out)

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("history_folder", type=Path)
    args = parser.parse_args()
    print(gen_changes_from_history(args.history_folder))
