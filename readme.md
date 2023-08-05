# rt1we-rs

![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/sevas/rt1we-rs/rust.yml)
[![codecov](https://codecov.io/gh/sevas/rt1we-rs/branch/main/graph/badge.svg?token=OATNZZ420B)](https://codecov.io/gh/sevas/rt1we-rs)

Raytracer in one weekend, in rust.

https://github.com/RayTracing/raytracing.github.io/

This is a toy project for educational purpose. 
Don't expect anything mindblowing. Just simple code from first principles. 

Current status: 
![](media/latest.png)

[Video of a simple trajectory](media/output.mp4)

<div>
<video width="99%" height="360" autoplay loop muted>
    <source src="media/output.mp4" type="video/mp4" markdown="1">
</video>
</div>


## Word of Caution

Since I am just learning rust, some stuff might be badly implemented.
Especially, custom operators and best practices for data ownership may
be completely wrong, as I am currently focusing on "making things work", and
improve as I read more about the language and libraries.

Don't use any code in this repo as reference for anything.


## python tools

- `scripts/ppm2png.py`: convert a folder of ppm files to png
- `scripts/imview.py`: [pyqtgraph](https://pyqtgraph.readthedocs.io/en/latest/)-based viewer for image files. Image will be updated every time the file is changed. 
