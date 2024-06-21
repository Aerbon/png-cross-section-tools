# PNG Cross Section Tools

I wrote this program because i couldn't get my CAD software to calculate the area moment of inertia of some components in my project.

## How To Use

the program will request a file path to a png image of your cross-section.
The cross-section should be in white and the background black (though technically only the red channel will be looked at).
The image should have only 3 channels (no alpha / transparency).

The program will then load the image and request the scale of the image, in pixels per mm.

Once both are submited the results will be printed.

## TODO

- allow input of the path and scale via command line arguments.
- allow output to a file via command line arguments.
