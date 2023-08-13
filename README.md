claim section --

All tasks T1, T2, T3, T4, T5, T6 and T7 have been fully implemented. Additionally all effects (as well as own scenes, julia sets, kdtree and supersampling) have been implemented and made togglable. The project was written in Rust. The predefined scenes are in the scenes folder and the rendered images in the output folder.

-- tested environments --

The assignment was created on endeavourOS (version: 6.1.8-arch1-1, graphic card: Intel Corporation HD Graphics 620 (rev 02), tested on arch linux (version: 6.2.2-arch1-1, graphic card: Intel Corporation UHD Graphics 620 (rev 07).

-- additional and general remarks --
Please note that the project will only run with Rust 1.17 due to OnceLock.

Usage:
cd into src directory
cargo run --release <path to XML> <path to toml file (optional)>

Example:
cargo run --release ../scenes/example1.xml

To use the toml file the command will look like this:
cargo run --release ../scenes/chess.xml ../scenes/chess.toml

A list of commands to render all the scenes:
cargo run --release ../scenes/example1.xml
cargo run --release ../scenes/example2.xml
cargo run --release ../scenes/example3.xml
cargo run --release ../scenes/example3.xml ../scenes/cook.toml
cargo run --release ../scenes/example4.xml
cargo run --release ../scenes/example4-area.xml ../scenes/cook.toml
cargo run --release ../scenes/example4-julia.xml
cargo run --release ../scenes/example5.xml
cargo run --release ../scenes/example6.xml
cargo run --release ../scenes/example6.xml ../scenes/anim.toml
cargo run --release ../scenes/example6-anti.xml ../scenes/anti.toml
cargo run --release ../scenes/example6-fresnel.xml ../scenes/fresnel.toml
cargo run --release ../scenes/example6-julia.xml ../scenes/julia-dof-cook.toml
cargo run --release ../scenes/example7.xml
cargo run --release ../scenes/example7-dof.xml ../scenes/dof.toml
cargo run --release ../scenes/example8.xml
cargo run --release ../scenes/example9-normal.xml ../scenes/normal.tomlcargo run --release ../scenes/spotlight.xml
cargo run --release ../scenes/env.xml

To use supersampling just append ../scenes/supersampling.toml. You can only render one toml file at a time, to have multiple effects or try them out you would need to write the toml files yourself. There are some already specified. 

-- Resources --

To get started I used the recommended literature: https://raytracing.github.io/books/RayTracingInOneWeekend.html. Additionally I used other ressources:

* [Fast and efficient implementation of KD-Tree for raytracer in Rust](https://www.flomonster.fr/articles/kdtree.html)
* [Heuristics for ray tracing using space subdivision](https://graphicsinterface.org/wp-content/uploads/gi1989-22.pdf)
* [On building fast kd-Trees for Ray Tracing, and on doing that in O(N log N)](http://www.irisa.fr/prive/kadi/Sujets_CTR/kadi/Kadi_sujet2_article_Kdtree.pdf)
* [Exact AABB of transformed spheres](https://tavianator.com/2014/ellipsoid_bounding_boxes.html)
* [stackoverflow: Depth of field](https://stackoverflow.com/questions/10012219/how-to-implement-depth-of-field-in-ray-tracer)
* [graphicscompendium: Cook Torrance](https://graphicscompendium.com/gamedev/15-pbr)
* [Wikipedia](https://en.wikipedia.org/wiki/Bilinear_interpolation)
* [Quaternion Julia Sets](https://www.cs.cmu.edu/~kmcrane/Projects/QuaternionJulia/paper.pdf)
* [scratchapixel: Reflection, Refraction, Fresnel](https://www.scratchapixel.com/lessons/3d-basic-rendering/introduction-to-shading/reflection-refraction-fresnel.html)
* [demofox: Reflection, Refract, Fresnel, Beer's Law](https://blog.demofox.org/2017/01/09/raytracing-reflection-refraction-fresnel-total-internal-reflection-and-beers-law/)
* [graphicscompendium: Fresnel/Beer's Law](https://blog.demofox.org/2017/01/09/raytracing-reflection-refraction-fresnel-total-internal-reflection-and-beers-law/)
* [Area Lights](http://raytracerchallenge.com/bonus/area-light.html)
* [Normal Mapping](http://www.opengl-tutorial.org/intermediate-tutorials/tutorial-13-normal-mapping/)
