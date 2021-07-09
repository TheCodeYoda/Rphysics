# Rphysics
A simple Physics simulation written completely in Rust which supports
* Elastic and Inelastic collisons
* gravity

## Usage

> **_NOTE:_**  0.0 <= Coeffecient of restitiution <= 1.0

1. Clone the repo using the link [https://github.com/TheCodeYoda/Rphysics.git]
2. cd Rphysics/
3. cargo run [gravity state] [no.of circles] [coeffecient of restitiution]

Example 

* cargo run off 10 1.0

## Simulations

* Elastic Collisons
![Alt Text](/gifs/collision.gif)
* Gravity 
![Alt Text](/gifs/gravity.gif)
