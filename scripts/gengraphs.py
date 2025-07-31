from pathlib import Path
import json
import argparse
import matplotlib.pyplot as plt
import matplotlib.dates as pltdates
import numpy as np
import datetime

def gen_graph(x, ys, labels, title):
    fig, ax = plt.subplots(dpi=200, figsize=[19.20, 10.80])
    ax.xaxis.set_major_formatter(pltdates.DateFormatter('%H:%M', tz=datetime.timezone(-datetime.timedelta(hours=4))))
    for y, label in zip(ys, labels):
        ax.plot(x, y, linewidth=2.0, label=label)
    ax.legend(loc='upper center', bbox_to_anchor=(0.5, -0.05), frameon=False, ncols=8)
    return fig

def extract_key(history, teams, key):
    out = [list() for _ in teams]
    for save in history:
        for i, team in enumerate(teams):
            out[i].append(save[team][key])
    return [np.array(x) for x in out]

def gen_graphs_from_history(history_folder: Path):
    history = dict()
    for save in history_folder.iterdir():
        timestamp = int(save.stem.removeprefix("save_"))
        with save.open("rb") as f:
            history[timestamp] = json.load(f)["players"]

    timestamps = sorted(history.keys())
    data = [history[t] for t in timestamps]
    teams = list({token for save in data for token in save})
    final = data[-1]
    labels = [final[t]["name"] for t in teams]

    timestamps = np.array(timestamps).astype('datetime64[s]')

    for prop in ["score", "kills", "wins", "chests", "segfaults"]:
        print(f"Generating {prop} graph...")
        fig = gen_graph(timestamps, extract_key(data, teams, prop), labels, f"{prop} par équipe")
        fig.savefig(f"graphs/{prop}.png", bbox_inches='tight')
        plt.close(fig)

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("history_folder", type=Path)
    args = parser.parse_args()
    gen_graphs_from_history(args.history_folder)
