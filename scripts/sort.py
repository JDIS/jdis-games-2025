from pathlib import Path
import json
import argparse
import matplotlib.pyplot as plt
import matplotlib.dates as pltdates
import numpy as np
import datetime

def gen_stats_from_file(save: Path):
    with save.open("rb") as f:
        final = json.load(f)["players"]

    teams = list(final.keys())

    for prop in ["score", "kills", "wins", "chests", "segfaults"]:
        print(f"\n{prop}:")
        values = np.array([final[t][prop] for t in teams])
        idxs = np.argsort(values)
        for i in idxs[::-1]:
            if final[teams[i]][prop] != 0:
                print(f"\t{final[teams[i]]['name']} - {final[teams[i]][prop]} {prop}")

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("save", type=Path)
    args = parser.parse_args()
    gen_stats_from_file(args.save)
