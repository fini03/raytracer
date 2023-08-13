# Raytracer
This is a simple raytracer developed as a final project for the course "GFX - Foundations of Computer Graphics" at the University of Vienna. The objectives of this task was to understand the conepts of raytracing and create a raytracer which renders a scene based on a given XML file following a specific [file specification](http://vda.univie.ac.at/Teaching/Graphics/23s/Labs/Lab3/lab2_file_specification.html). It currently supports all the effects listed on the course site including supersampling, rendering julia sets and the kd-tree acceleration structure using N log N building algorithm. Please note that the project will only run with Rust 1.17 due to OnceLock. Per default multithreading is on (specify the number of threads num_threads you want to activate in the toml file) and kdtree off (set in the toml file kdtree = true to activate).

## Usage
cd into src directory and run:
`cargo run --release <path to XML> <path to toml file (optional)>`

Example:
```cargo run --release ../scenes/example1.xml```

To use the toml file the command will look like this:
```cargo run --release ../scenes/chess.xml ../scenes/chess.toml```

A list of commands to render some of the scenes:
```cargo run --release ../scenes/example3.xml ../scenes/cook.toml
cargo run --release ../scenes/example4-area.xml ../scenes/cook.toml
cargo run --release ../scenes/example6.xml ../scenes/anim.toml
cargo run --release ../scenes/example6-anti.xml ../scenes/anti.toml
cargo run --release ../scenes/example6-fresnel.xml ../scenes/fresnel.toml
cargo run --release ../scenes/example6-julia.xml ../scenes/julia-dof-cook.toml
cargo run --release ../scenes/example7-dof.xml ../scenes/dof.toml
cargo run --release ../scenes/example9-normal.xml ../scenes/normal.toml
cargo run --release ../scenes/spotlight.xml
```

To use supersampling append `../scenes/supersampling.toml` to the command. Only one toml file can be added at a time, to have multiple effects or trying them out, write your own toml files. There are some already specified. 

## Example Output
<table>
  <tr>
    <td> <img src="https://github.com/fini03/raytracer/blob/main/output/example3-cook.png"  alt="1" width = 512px height = 440px ></td>
    <td><img src="https://github.com/fini03/raytracer/blob/main/output/example4-area.png" alt="2" width = 512px height = 440px></td>
   </tr> 
   <tr>
      <td><img src="https://github.com/fini03/raytracer/blob/main/output/example7-dof.png" alt="3" width = 512px height = 440px></td>
      <td><img src="https://github.com/fini03/raytracer/blob/main/output/example9-normal.png" align="right" alt="4" width = 512px height = 440px>
  </td>
  </tr>
</table>
<table>
  <tr>
      <td><img src="https://github.com/fini03/raytracer/blob/main/output/example6-julia.png" alt="5" width = 1920px height = 512px></td>
   </tr>
   <tr>
      <td><img src="https://github.com/fini03/raytracer/blob/main/output/chess.png" align="right" alt="6" width = 1280px height = 512px>
   </td>
   </tr>
</table>

## Resources
* [Raytracing in one weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
* [Fast and efficient implementation of KD-Tree for raytracer in Rust](https://www.flomonster.fr/articles/kdtree.html)
* [Heuristics for ray tracing using space subdivision](https://graphicsinterface.org/wp-content/uploads/gi1989-22.pdf)
* [On building fast kd-Trees for Ray Tracing, and on doing that in O(N log N)](http://www.irisa.fr/prive/kadi/Sujets_CTR/kadi/Kadi_sujet2_article_Kdtree.pdf)
* [Exact AABB of transformed spheres](https://tavianator.com/2014/ellipsoid_bounding_boxes.html)
* [Depth of field](https://stackoverflow.com/questions/10012219/how-to-implement-depth-of-field-in-ray-tracer)
* [Cook Torrance](https://graphicscompendium.com/gamedev/15-pbr)
* [Bilinear interpolation](https://en.wikipedia.org/wiki/Bilinear_interpolation)
* [Quaternion Julia Sets](https://www.cs.cmu.edu/~kmcrane/Projects/QuaternionJulia/paper.pdf)
* [Reflection, Refraction, Fresnel](https://www.scratchapixel.com/lessons/3d-basic-rendering/introduction-to-shading/reflection-refraction-fresnel.html)
* [Reflection, Refract, Fresnel, Beer's Law](https://blog.demofox.org/2017/01/09/raytracing-reflection-refraction-fresnel-total-internal-reflection-and-beers-law/)
* [Fresnel/Beer's Law](https://blog.demofox.org/2017/01/09/raytracing-reflection-refraction-fresnel-total-internal-reflection-and-beers-law/)
* [Area Lights](http://raytracerchallenge.com/bonus/area-light.html)
* [Normal Mapping](http://www.opengl-tutorial.org/intermediate-tutorials/tutorial-13-normal-mapping/)
