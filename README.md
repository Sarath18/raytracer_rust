# Raytracer
This is my implementation of the book [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
by [*Peter Shirley*](https://github.com/petershirley) in Rust.

## Final result
**Image size:** 1200 x 800 pixels
**Samples per pixel:** 500

![](results/output.png)

## Dependencies
- image "0.23.9"
- rand "0.7.3"

## Build
```bash
cargo build --release
```

## Run
```bash
cargo run --release
```

## The Process

### First Image
![First Image](results/image.png )

### Gradient Background
![](results/bg.png)

### A Sphere
![](results/sphere.png)

### Surface Normals
![](results/world.png)

### Anti-aliasing
![](results/antialiasing.png)

### Diffuse
![](results/diffuse.png)

### Lambertian Reflection
![](results/lambertian_reflection.png)

### Materials
![](results/mat.png)

### Metal
![](results/reflective.png)

### Fuzzy Metal
![](results/fuzzy.png)

### Refractive Material
![](results/refractive.png)

### Camera Position
![](results/position_camera.png)

### Defocus Blur
![](results/defocus_blur.png)

### The Final Render
![](results/output.png)