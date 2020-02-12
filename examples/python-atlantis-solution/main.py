import sys
import json
import typing
import attr
import random

from enum import Enum


class Flavor(Enum):
    General = 'General'
    Vector = 'Vector'
    Matrix = 'Matrix'


class Color(Enum):
    Red = 'Red'
    Green = 'Green'
    Blue = 'Blue'


@attr.s(auto_attribs=True)
class Layer:
    color: Color
    thickness: int

    @classmethod
    def from_dict(cls, data: dict):
        return Layer(
            color=Color(data["color"]),
            thickness=data["thickness"]
        )


@attr.s(auto_attribs=True)
class Pearl:
    pearl_id: int
    layers: typing.List[Layer]

    @classmethod
    def from_dict(cls, data: dict):
        return Pearl(
            pearl_id=data["id"],
            layers=list(map(Layer.from_dict, data["layers"]))
        )


@attr.s(auto_attribs=True)
class Worker:
    worker_id: int
    flavor: Flavor
    desk: typing.List[Pearl]

    @classmethod
    def from_dict(cls, data: dict):
        return Worker(
            worker_id=data["id"],
            flavor=Flavor(data["flavor"]),
            desk=list(map(Pearl.from_dict, data["desk"]))
        )


@attr.s(auto_attribs=True)
class SimState:
    workers: typing.List[Worker]
    neighbor_map: typing.List[typing.Tuple[int, int]]
    score: float

    @classmethod
    def from_dict(cls, data: dict):
        args = {
            **data,
            **dict(workers=list(map(Worker.from_dict, data["workers"])))
        }

        return SimState(**args)


@attr.s(auto_attribs=True)
class PassInstruction:
    pearl_id: int
    to_worker: int

    def to_dict(self):
        return {
            "Pass": {
                "pearl_id": self.pearl_id,
                "to_worker": self.to_worker
            }
        }


@attr.s(auto_attribs=True)
class NomInstruction:
    pearl_id: int

    def to_dict(self):
        return {
            "Nom": self.pearl_id
        }


def random_action_for_worker(worker: Worker):
    if not worker.desk:
        return None

    pearl_to_nom = random.choice(worker.desk)

    return NomInstruction(pearl_id=pearl_to_nom.pearl_id)


def actions_for_state(state: SimState) -> dict:
    """
    Implement your policy here!

    For starts this just picks a random pearl to nom, which does not score
    very well.
    """
    actions = {
        worker.worker_id: random_action_for_worker(worker)
        for worker in state.workers
    }

    return {
        worker_id: action
        for (worker_id, action) in actions.items()
        if action is not None
    }


for line_str in sys.stdin:
    actions = actions_for_state(SimState.from_dict(json.loads(line_str)))

    actions_dict = {
        worker_id: action.to_dict()
        for (worker_id, action) in actions.items()
    }

    print(json.dumps(actions_dict), flush=True)
