# Rphysics
A simple Physics simulation written completely in Rust which supports
* Elastic and Inelastic collisons
* gravity

## Usage

> **_NOTE:_**  0.0 <= Coeffecient of restitiution <= 1.0 , rust version 1.47.0 required 

1. Clone the repo
2. cd Rphysics/
3. cargo run **[gravity state]** **[no.of circles]** **[coeffecient of restitiution]**

Example 

* cargo run off 10 1.0

## Simulations

### Elastic collisons
![Elastic Collisons](/gifs/collision.gif)
### Gravity
![Gravity](/gifs/gravity.gif)
