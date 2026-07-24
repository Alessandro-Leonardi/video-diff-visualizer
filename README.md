# IN DEVELOPMENT

# Temporal Frame Differencing 
### (or motion detection via frame delta thresholding)

## Technical Summary

> **A Rust CLI tool for video temporal frame differencing. It decodes a video frame-by-frame, calculates the pixel-wise delta between consecutive frames, applies a configurable color/threshold mask (e.g., highlighting significant red-channel changes in white while setting static pixels to black), and re-encodes the resulting image sequence into a new video.**

## Rust Ecosystem & Tech Stack

| **Pipeline Stage**          | **Recommended Crate**       | **Purpose**                                                                                                                  |
| --------------------------- | --------------------------- | ---------------------------------------------------------------------------------------------------------------------------- |
| **Video Decoding/Encoding** | `ffmpeg-next` or `video-rs` | Handles opening video containers (`mp4`, `mkv`), extracting raw RGB/YUV frames, and encoding processed frames back to video. |
| **Pixel Manipulation**      | `image`                     | Provides high-performance buffer manipulation, thresholding, and channel inspection (`R`, `G`, `B`).                         |
| **CLI Arguments**           | `clap`                      | Easily parses flags like `--threshold 30`, `--channel red`, `--input input.mp4`, and `--output diff.mp4`.                    |
| **Parallel Processing**     | `rayon`                     | Speeds up frame-by-frame matrix math across CPU cores if doing heavy batch processing.                                       |

## Key Technical Considerations

1. **Memory Usage (File Size vs. RAM):** Loading an entire uncompressed high-definition video into RAM at once can consume dozens of gigabytes. Streams are usually processed via a **frame buffer pipe** (decode frame $N$ and frame $N-1$ $\to$ diff $\to$ encode frame $N$ $\to$ drop from RAM).
    
2. **Color Space:** Are you operating directly on RGB values, or would HSV / grayscale delta math be useful for subtle motion detection?
    
3. **Threshold Flexibility:** Do you want fixed absolute thresholds (e.g., $\vert{}R_t - R_{t-1}\vert{} > n$), or dynamic visual effects (like heatmaps or optical flow coloring)?
