## Level Structure
The game is structured in levels. Each level contains two or more rooms.
Each level contains a *first room*, a *final room* and a list of rooms which (can be) entered in a random
order and rooms which can be entered only through another room.

A room can force a sequence by pointing to a next room as one of the results of a player choice.
For example a room can show these choices ("go left", "go right") and point to one specific room for each choice.

When a Level starts the *first room* is entered.
After any room, if no follow up is set, a random room will be selected as next room.
The player can gain *level points* as rewards for clearing rooms. Once a specific amount of *level points* is aquired the *final room* will be entered.
The *final room* can also start a sequence of rooms.
Once the final room (or sequence) is finished the next level will start.

### Example Setup
```
first room:
- [A (next:B)]
```
```
random rooms:
- [C]
- [D (next: E)]
- [F (next: G)]
```
```
final room:
- [X (next: Y)]
```
```
others:
- [B]
- [E]
- [G (next: H)]
- [H]
- [Y]
```
**possible sequence**
```
[A] -> [B] -> [D] -> [E] -> [F] -> [X] -> [Y]
```

## Encounters
A player choice inside a room can result into an encounter.
If the encounter is won the game continues in the next room (random or specific to the last choice).

### example 1
**room r1**  
text: "A crazy monster blocks your way"
choices: "run away" (nothing) | "fight" (encounter: e1)

**encounter e1**  
Turn based fight against Crazy Monster


### example 2
**room r2**  
text: "You see a normal door and a bloody door"
choices: "normal door" (room: r3) | "bloody door" (encounter: e2, room: r4)

**encounter e2**  
Turn based fight against Bloody Zoombie