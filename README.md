# Verlet
Physics Engine built using [Speedy2D](https://github.com/QuantumBadger/Speedy2D) for the graphics and [Vecto-rs](https://github.com/Zycrasion/vecto-rs) (Built by me) for the maths & QuadTree.

### Debug
![](https://github.com/Zycrasion/verlet_multithreaded/blob/master/res/example1.png?raw=true)
![](https://github.com/Zycrasion/verlet_multithreaded/blob/master/res/example2.png?raw=true)

### Release
![](https://github.com/Zycrasion/verlet_multithreaded/blob/master/res/example3.png?raw=true)
![](https://github.com/Zycrasion/verlet_multithreaded/blob/master/res/example4.png?raw=true)

## Running
In a Terminal Write
```bash
git clone https://github.com/Zycrasion/verlet_multithreaded
cd verlet_multithreaded
cargo run -r
```

## Controls
-   Space - Toggle Autofill
-   Backspace - Delete currently grabbed Node
-   C - Clear all Nodes
-   Middle Mouse Button - Pan View
-   Scroll - Zoom
-   Escape - Exit
-   V - Toggle QuadTree View
-   N - Add Node
-   Left Click - Grab Node
-   Shift - Take Screenshot

## Performance on Ryzen 5600G (Debug Build)
|Circles| FPS|
|-------|----|
|1508   | 60 |
|4434   | 53 | 
|5014   | 45 |
|10,000~  | 15 |

## Peformance on Ryzen 5600G (Release)
|Circles            | FPS   |
|-------------------|-------|
|0 - 20,000         | 55-60 |
|20,000 - 26,000    | 40-55 |
|26,000 - 32,000    | 30-40 |

## Peformance on 2017 Macbook Air (Release)
I honestly forgot about release builds. I tested this on a 6 year old laptop and got 26494 circles before hitting 20 fps.
|Circles| FPS|
|-------|----|
|26,494  | 20 |

## QuadTree Implementation
I have used the QuadTree from vecto-rs v0.7.4,
The QuadTree is rebuilt every frame due to the dynamic nature of the Nodes.
In Order to Reduce the side effects a QuadTree may have on collisions they are only
allowed to subdivide 4 times. This keeps the QuadTree cells big enough to fit nodes in and to be able to compute the collisions properly.

## Multithreading?
No, It was originally meant to be multithreaded but that doesn't increase performance significantly and only increased code complexity so it was removed, Collisions could be calculated multithreaded but it would introduce race conditions that I don't think is worth it for an extra couple thousand Nodes.
