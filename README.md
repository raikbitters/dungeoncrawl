# Rusty Rogue-like

## Description

A dungeon crawler with procedural generated levels, monsters of increasing difficulty,
and turn-based movement.

## Story

The hero's hometown is suffering from a plague of monsters.
The hero must venture into the dungeon to find the source of the plague and destroy it.

## Basic Game Loops

1. Enter dungeon level.
2. Explore, revealing the map.
3. Encounter enemies whom the player fights of flees from.
4. Find power-ups and use them to strengthen the paler.
5. Locate the exit to the level - go to 1.

## Minimum Viable Product

1. ...

## Stretch Goals

1. ...

## Component composition

| Name      | Goblin    | Goblin Archer | Goblin Mage | Dragon  |
|-          |-          |-              |-            |-        |
| Render    | V         | V             | V           | V       |
| Position  | V         | V             | V           | V       |
| Name      | V         | V             | V           | V       |
| Melee AI  | V         |               |             | V       |
| Ranged AI |           | V             |             |         |
| Spell AI  |           |               | V           | V       |
