use image::{ImageBuffer, Luma, RgbImage};

/// Computes a binary mask based on the red-channel difference between two frames.
///
/// - `prev`: The previous RGB frame buffer.
/// - `curr`: The current RGB frame buffer.
/// - `threshold`: Minimum absolute difference in red channel value (0-255) to trigger white.
fn compute_red_diff_mask(
    prev: &RgbImage,
    curr: &RgbImage,
    threshold: u8,
) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let (width, height) = curr.dimensions();

    // Create an 8-bit grayscale output buffer with the same dimensions
    ImageBuffer::from_fn(width, height, |x, y| {
        let prev_pixel = prev.get_pixel(x, y);
        let curr_pixel = curr.get_pixel(x, y);

        // Extract the Red channel (index 0) from both pixels
        let prev_red = prev_pixel[0];
        let curr_red = curr_pixel[0];

        // Calculate absolute difference without underflow wrapping
        let red_delta = prev_red.abs_diff(curr_red);

        // Map to white (255) if delta >= threshold, otherwise black (0)
        if red_delta >= threshold {
            Luma([255])
        } else {
            Luma([0])
        }
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Load two consecutive frame images from disk
    let frame_a = image::open("frame_001.png")?.into_rgb8();
    let frame_b = image::open("frame_002.png")?.into_rgb8();

    // Ensure frames match dimensions before processing
    if frame_a.dimensions() != frame_b.dimensions() {
        return Err("Frame dimensions do not match!".into());
    }

    // 2. Set your threshold (e.g., a variation of 25 or more in the red channel)
    let red_threshold = 25;

    // 3. Perform the difference operation
    let diff_mask = compute_red_diff_mask(&frame_a, &frame_b, red_threshold);

    // 4. Save the resulting black-and-white frame
    diff_mask.save("diff_output.png")?;

    println!("Difference mask successfully saved to diff_output.png");
    Ok(())
}
