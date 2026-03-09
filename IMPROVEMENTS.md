# Improvement Ideas

## Performance

- **Avoid `sqrt()`** — Compare `distance²` against `radius²` instead. You're doing a `sqrt` per pixel (~307k/frame at 60fps). Squaring the thresholds is free and eliminates the most expensive operation in the hot loop.
- **Use `OffscreenCanvas` + Web Worker** — Move the draw loop off the main thread so UI stays responsive, especially on lower-end devices.
- **Process in SIMD** — Rust's `std::simd` or the `packed_simd` crate can process multiple pixels in parallel on supported browsers.

## Visual Quality

- **HSL/HSV color space** — Euclidean distance in RGB doesn't match human perception well. Converting to HSL or HSV and keying primarily on hue would handle shadows and highlights much better (a dark green and bright green are far apart in RGB but close in hue).
- **Spill suppression** — Green screen light often "spills" onto the subject's edges. A desaturation pass on near-threshold pixels would clean this up.
- **Background replacement** — Currently masked pixels are just transparent (or blue debug). Adding an option to composite a replacement image/video behind the subject is the natural next step.

## UX

- **Eyedropper tool** — Let users click on the video to pick the key color directly instead of guessing with the color picker.
- **Labels for controls** — The color picker and slider have no labels; a new user wouldn't know what they do.
- **Upload custom video** — Allow drag-and-drop or file input instead of only the bundled `greenscreen.mp4`.
- **Live webcam input** — Use `getUserMedia()` as a video source for real-time green screen with a webcam.
- **Before/after toggle** — A button to quickly compare original vs. keyed output.

## Code Quality

- **The alpha feathering is inverted** (`lib.rs:30-31`) — `alpha_factor` is highest when `distance` is close to `max_distance` (far from the key color), but it's being used to reduce alpha. This means pixels closer to the key color in the feather zone keep more opacity. The formula should be `(distance - radius) / (max_distance - radius)` instead.
- **Remove the blink-blue debug mode** or put it behind a toggle — it's distracting for end users.
- **`maxDistance` should be a `f64`** on the JS side — it's computed as `radius * 1.5` but passed as a truncated integer.
