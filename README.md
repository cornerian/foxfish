# foxfish

Foxfish is a Super Smash Bros. Melee position evaluation engine. 

## How does it work?

It takes in data from a single frame and outputs a score that corresponds to how much advantage the target player currently has in terms of a stock. If the value is 0, both players are exactly even. If the value is 0.5, the target player could be considered to have a lead of 0.5 stocks. The value becomes 1 if the player has a true combo to death available.

This is achieved by factoring percentages, position on (or off) stage, distance between players, if either player is currently in a committed state (not free to react), etc.

The exact details can be found in `evaluate.rs`.