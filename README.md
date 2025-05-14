# tinyrenderer-rust

Implementing the Dmitry V. Sokolov (ssloy) tinyrenderer in rust.

The goal at first is port the main logic to rust with minimal changes besides maybe rust special stuff. Also I try to always generate the same images as the original cpp code,

### Progress

### Results

Just some images that were generated while coding this.

#### Gray shaded head

The normal generated image

![Gray shades](./assets/gray-shaded-head.png)

#### Color theme shading

I added a function that allows shading by a range of colors

![Color theme shading](assets/theme-shaded-head.png)

#### Z Buffer integrated into the scanline method

![Z Buffer](assets/zbuffer.png)

#### Texture mapping

![Texture mapping](assets/texture-mapped-head.png)

Frog test:

![frog test](assets/frog-test.png)

#### Perspective correction

![Perspective correction](assets/perspective-correction.png)

#### Moving the camera

![Moving the camera](assets/moved-camera.png)
