# Rphysics
A simple two-dimensional Physics Engine written completely in Rust which supports
* Elastic and Inelastic collisons.
* Mouse impulse support.
* Linear and rotational mechanics.
* gravity and friction simulation.


## Usage

> 0.0 <= Coeffecient of restitiution <= 1.0  
> rust version 1.47.0 required  
> Switch to dev branch for latest code  

1. Clone the repo
2. cd Rphysics/
3. cargo run **[gravity state]** **[no.of circles]** **[coeffecient of restitiution]**

Example 

* cargo run off 10 1.0

## Simulations

### Elastic collisons
![Elastic Collisons](/gifs/collision.gif)
### Mouse Impulse
![Mouse Impulse](/gifs/mouse_impulse.gif)
### Gravity
![Gravity](/gifs/gravity.gif)
