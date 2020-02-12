# Welcome to the Atlantis hackathon!

## Rules

1. Submit you own work. Use of libraries or external packages are allowed.
2. No teams, but feel free to discuss the problem with your peers as long as you submit your own work.
3. Please don't post solutions online, or if you do please don't post this a solution to the atlantis problem :)

# Atlantis
In the course of your underwater travels you are fortunate enough to stumble upon the lost city of Atlantis. But the local nautiloids who inhabit the city need help. They are trying to optimize their economy of pearl processing and are looking for consultants to maximize their daily production. In return for your aid, they promise to assist you with safe travel through the treacherous reefs that surround the city. You can't quite remember how you found yourself there in the first place, so you are grateful for their offer and decide to take them up on it.

The city's pearl processing routine seems simple enough: Exploler nautiloids go off and find raw pearls on the ocean floor and deposit them on the desk of the gate keeper (first) nautiloid of the city, they then need to be processed in the city and then ferried back at the main gate for transport out to their trade partners.

The processing procedure consists of stripping the pearls of their crusty outer coatings which come in three colors: red, blue, and green of varying thickness.

There are three type of nautiloids who take part in processing who at any discrete time step are either processing the unfinished pearls (dissolving the outer layers using their saliva which has strong digestive properties), or passing pearls from one neighboring nautiloid to another. `General` nautiloids can dissolve any coating, but do so at a slow pace of one unit thickness per step. `Vector` nautiloids are best at dissolving green coatings, and do so at a rate of 5 unit thicknesses per timesteps, and `Matrix` nautiloids can only dissolve blue coatings but are the quickest to do so. The full rate calculations are pasted below.
```
        match flavor {
            General => match layer {
                Red => 1,
                Green => 1,
                Blue => 1,
            },
            Vector => match layer {
                Red => 1,
                Green => 5,
                Blue => 2,
            },
            Matrix => match layer {
                Red => 0,
                Green => 0,
                Blue => 10,
            },
        }
```
Each nautiloid can only pass one pearl per timestep from and to their immediate neighbors. Each nautiloid has a desk that can hold an infinite number of pearls and a pearl has to be on their desk for them to work on it. Though you have heard that the gatekeeper nautiloid (id = 0) gets a bit frustrated when his desk gets too cluttered and has been known to turn away fresh pearls from time to time when that happens.

Pearls with no layers that are on the nautiloid's desk whose ID is `0` will be added to your total score (1 per pearl) at the end of each turn.

Your job, as honorary coordinator of the city, is to look over the set of nautiloids available to you, and their seating arrangement (who can reach who for pearl passing) and maximize the pearl output of the city each day.

At the beginning of each step of the simulation you (your program) will see (be passed through STDIN) the city's state printed as a single JSON object string. Eg,
```
{"workers":[{"id":0,"flavor":"General","desk":[]},{"id":1,"flavor":"Vector","desk":[]}],"neighbor_map":[[0,1]],"score":0}
```
Here we see a small town of two nautiloids, where the `neighbor_map` gives us the connectivity of the workers (here showing us that worker 0 and 1 are adjacent and can thus pass pearls between each other).

Another example with pearls would be:
```{"workers":[{"id":0,"flavor":"General","desk":[{"id":3446798295,"layers":[{"color":"Blue","thickness":17}]},{"id":2747624543,"layers":[{"color":"Red","thickness":12},{"color":"Green","thickness":11},{"color":"Blue","thickness":11}]},{"id":1487224610,"layers":[{"color":"Red","thickness":10}]},{"id":2795188117,"layers":[{"color":"Blue","thickness":24},{"color":"Red","thickness":13},{"color":"Green","thickness":27}]}]},{"id":1,"flavor":"Vector","desk":[]}],"neighbor_map":[[0,1]],"score":0}
```

Your job is to pass instructions to the pearls through STDOUT in another JSON object of the form:
```
{"1":{"Pass":{"pearl_id":52,"to_worker":0}},"0":{"Nom":1234}}
```
This line would instruct the gatekeeper nautiloid (id `0`) to use digestive juices to process pearl with id `1234` on their desk, while nautiloid `1` will pass pearl `52` from its desk to nautiloid `0`'s desk.

Any nautiloid that doesn't get instructions will sit idle, and any instructions that don't make sense for a given nautiloid (processing a type of pearl they can't process, or passing a pearl from their inbox to a neighbor when there isn't a pearl by that ID in their inbox) will also result in them being idle.

The simulation will last 50 timesteps by default, but in general you should try to write your code to be generic over runtime duration (and the atlantis runner binary is configurable).

## What we are looking for
Submissions will be ranked on their score from an `average-run` so make sure you test how well your solution performs across multiple runs!

For testing, you can provide the path to your program binary or script (with no arguments) when running the atlantis simulator:
```
$ ./atlantis single-run /code/my_binary_or_script
# or
$ ./atlantis single-run -- python myscript.py --annealing=max
```
`single-run` will run a single simulation printing the intermediate outputs for debugging and the final score while `average-run` mode will run many simulations and give you the average score across all of them. You can also control the PRNG seed (`--seed`) in each mode if you're debugging crashes or want to test against certain scenarios you see while testing.

A minimal example of a valid driver script in Python 3 is,
```python
import sys

for line in sys.stdin:
    print('{}', flush=True)
```

See the examples directory for some starter projects.

# Solution Submission

Submit your solution(s) https://forms.gle/utQt8xdeDg9KY8Rz6 please make sure your submission runs as docker container .tgz files.
