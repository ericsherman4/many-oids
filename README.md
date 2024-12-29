# Many *oids
Simulating Hypocycloid, Superior Hypotrchoid, Inferior Hypotrchoid, Epicycloid, Super Epitrochoid, and Inferior Epitrochoid, Retrograde Motion, and Epicycles

## Research

https://wwwtyro.net/2019/11/18/instanced-lines.html
https://www.reddit.com/r/bevy/comments/1ciwzb1/is_it_bad_to_use_gizmos_in_the_game/
https://github.com/ForesightMiningSoftwareCorporation/bevy_polyline
https://www.reddit.com/r/bevy/comments/1e04xk8/how_to_create_2d_object_from_arbitrary_list_of/
seems like you either use polyline or gizmos. giszmos seems performant enough even tho they are redrawn every frame

method one seems to be try gizmos? 
if that doesnt work, try primitives https://docs.rs/bevy/0.14.2/bevy/math/primitives/index.html
if that doesnt work try polyline
if that doesnt work, try custom meshes with maybe a custom shader

check this out which uses cylinders 
https://www.reddit.com/r/bevy/comments/1hdmsrs/lorenz_system_rendered_in_bevy/

Some cool drawings made by a spirograph https://en.wikipedia.org/wiki/Spirograph#/media/File:Various_Spirograph_Designs.jpg

