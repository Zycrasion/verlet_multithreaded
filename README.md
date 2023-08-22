# Verlet
![](https://github.com/Zycrasion/verlet_multithreaded/blob/master/res/example1.png?raw=true)
![](https://github.com/Zycrasion/verlet_multithreaded/blob/master/res/example2.png?raw=true)

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

## Performance on Ryzen 5600G
|Circles| FPS|
|-------|----|
|1508   | 60 |
|4434   | 53 | 
|5014   | 45 |
|1000~  | 15 |

## QuadTree Implementation
I have used the QuadTree from vecto-rs v0.7.4,
The QuadTree is rebuilt every frame due to the dynamic nature of the Nodes.
In Order to Reduce the side effects a QuadTree may have on collisions they are only
allowed to subdivide 4 times. This keeps the QuadTree cells big enough to fit nodes in and to be able to compute the collisions properly.

## Multithreading?
No, It was originally meant to be multithreaded but that doesn't increase performance significantly and only increased code complexity so it was removed, Collisions could be calculated multithreaded but it would introduce race conditions that I don't think is worth it for an extra couple thousand Nodes.