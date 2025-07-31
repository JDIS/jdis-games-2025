import random
import json

alphabet = "234567abcdefghijklmnopqrstuvwxyz"
with open("save.json", "w") as f:
    json.dump({"players": {
        "".join(random.choices(alphabet, k=8)): {
            "name": f"DEFAULT{i}",
            "score": 0,
            "kills": 0,
            "wins": 0,
            "chests": 0,
            "segfaults": 0
        } for i in range(1,41)
    }}, f)
