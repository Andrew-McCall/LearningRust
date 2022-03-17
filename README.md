# Learning Rust

I wanted to learn rust; I'm a massive nerd.
Here are some of my first projects.

*Everything is a work in progress - the enjoyment was programming not polish graphics although that is not out of the picture*

Key Libs:
- Piston_Window
- Rand
- Opengl_graphics
- Standard Time & Thread were used also

# Projects

## Fortress
### Description
A tower defence game, Enemies spawn walking left to right at various speeds. Shoot them to not let them reach and damage you. 
Based on piston using opengl_graphics. Included Text and texture rendering and mouse and keyboard inputs.

### Future Improvements
- Better Textures
- Resizeable screen (stuck to 480, 360)
- Animations
- Sound Effects

## Snake
### Description
Classic video game. The snake seeks apples, growing with each and will die/lose if the snake runs into itself or border. (Snake is controllable by player, This is the challange i'm learning - *still with piston_window*)

### Future Improvements
- Auto Restart on buttonpress (After death)
- ~~circle apples (Done)~~ 
- speed on size
- Score / Win Condition


## Conway's Game Of Life
### Description
My first program in rust.
A simple simulation created by John Conway. Pixels are either live or dead; Depending on their neighbour count they will spawn, survive, or die.

### Future Improvements
May add editor so no random start
Refactor Code to be pretty after some more rust experience. 

# Notes
Gif of Conway's Game Of Life: https://i.imgur.com/kcvTdes.mp4
Snake: ![Snake](https://i.imgur.com/6S2vtOf.png)
Fortress: ![Fortress](https://i.imgur.com/SIPmyRM.png)
