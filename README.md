# Chroma Key Video with Color Picker and Anti-Aliasing

This project demonstrates a chroma key (green screen) effect using raw JavaScript and the HTML5 `<canvas>` element. It includes a color picker to select the base color and a slider to adjust the color radius, with anti-aliasing to create smooth edges around the masked areas.

## Features

- Select base color using a color picker
- Adjust the color radius with a slider
- Anti-aliasing for smooth edge transitions
- Blinking effect on the masked area

## Prerequisites

- Modern web browser with HTML5 support

## Getting Started

### Download the Project

Clone or download the project files to your local machine.

### Setup

1. Ensure you have a green screen video file named `greenscreen.mp4` in the same directory as the HTML file. Alternatively, update the video source path in the code to match your video file location.

### Running the Project

1. Open the `index.html` file in your web browser.

### Usage

1. Click anywhere on the canvas to start the video playback.
2. Use the color picker to select the base color for chroma keying.
3. Adjust the slider to set the color radius for masking.
4. Observe the masked area blinking blue with anti-aliased edges.

## Code Overview

### HTML Structure

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <title>Chroma Key Example with Color Picker and Slider</title>
</head>
<body>
  <input type="color" id="colorPicker" value="#00ff00" style="position: absolute; top: 10px; left: 10px;">
  <input type="range" id="radiusSlider" min="0" max="255" value="50" style="position: absolute; top: 40px; left: 10px;">
  <canvas id="canvas" width="640" height="480" style="position: absolute; top: 80px; left: 10px;"></canvas>

  <script src="script.js"></script>
</body>
</html>

## Video Credit 
## Anna Shvets
https://www.pexels.com/video/cute-puppy-sitting-on-a-stool-4838318/
