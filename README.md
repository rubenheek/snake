# Snake in 1 hour

Personal challenge to code snake in Rust using the macroquad crate in 1 hour.

Limitations:
- Random coordinates are generated for the apple until these consitute a vacant cell. If few cells are left, this may take extremely long, rendering the game unplayable.
- Cell vacancy and self-collision are checked by iterating over every snake part, which takes increasingly long.
