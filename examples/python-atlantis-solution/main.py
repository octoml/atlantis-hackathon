import sys
import json


def actions_for_state(state: dict) -> dict:
    return {}


for line_str in sys.stdin:
    actions = actions_for_state(json.loads(line_str))

    print(json.dumps(actions), flush=True)
