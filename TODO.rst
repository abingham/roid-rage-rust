* Make sure collision-world is cleaned when entities are deleted.
  
  * One idea is to have a component for "destroyed" which is processed uniformly at the end of a cycle. This let's us separate the
    concept of "collision" from "destroyed" since e.g. a ship may take more than one collision before being destroyed.

* Bullets should be destroyed on collision (and produce fragments).

* Add actual targeting.