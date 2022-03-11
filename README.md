# Learning Rust

I wanted to learn rust; I'm a massive nerd.
Here are some of my first projects.

# Projects

## Conway's Game Of Life
### Description
My first program in rust.
A simple simulation created by John Conway. Pixels are either live or dead; Depending on their neighbour count they will spawn, survive, or die.

Key Libs:
- Piston_Window
- Rand
- *Time & Thread was used also*

### Future Improvements
May add editor so no random start
Refactor Code to be pretty after some more rust experience. 

### Notes
Gif of my game: https://imgur.com/a/Xxhw7ss

## Snake
### Description
Classic video game. The snake seeks apples, growing with each and will die/lose if the snake runs into itself or border. (Snake is controllable by player, This is the challange i'm learning - *still with piston_window*)

### Future Improvements
- Auto Restart on buttonpress (After death)
- ~~circle apples (Done)~~ 
- speed on size
- Score / Win Condition

## Fortress (Beta)
### Description
Still a work in progress. A tower defence game, Enemies spawn walking left to right at various speeds. Shoot them to not let them reach and damage you. 
Based on pistion using opengl for graphics. Included Text and image rendering and mouse and keyboard inputs.
