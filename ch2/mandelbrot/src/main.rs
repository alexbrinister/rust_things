// Import the Complex number type from the num crate and the complex submodule
use num::complex::Complex;

/**
 * \brief Converts between the output space (a grid of rows and columns) and a range that surrounds
 * the Mandelbrot set (a continuous region near (0,0)).
 * \param max_iters if a value has not escaped before reaching the maximum number of iterations,
 * it's considered to be within the Mandelbrot set
 * \param x_min Minimum x coordinate of the space under search for set members
 * \param x_max Maximum x coordinate of the space under search for set members
 * \param y_min Minimum y coordinate of the space under search for the set members
 * \param y_max Maximum y coordinate of the space under search for set members
 * \param width Output width in pixels
 * \param height Output height in pixels
 * \returns A table representing a Mandelbrot set
 */
fn calculate_mandelbrot( max_iters: usize,
                         x_min: f64,
                         x_max: f64,
                         y_min: f64,
                         y_max: f64,
                         width: usize,
                         height: usize,
                         ) -> Vec<Vec<usize>> {
    // Create a container to house the data from each row
    let mut rows: Vec<_> = Vec::with_capacity(width);

    // Iterates row by row, allowing us to print the output line by line
    for img_y in 0..height {
        let mut row: Vec<usize> = Vec::with_capacity(height);

        // This loop calculates the proportion of the space covered in our output and converts that
        // to points within the search space.
        for img_x in 0..width {
            let x_percent = img_x as f64 / width as f64;
            let y_percent = img_y as f64 / height as f64;
            let cx = x_min + (x_max - x_min) * x_percent;
            let cy = y_min + (y_max - y_min) * y_percent;
            let escaped_at = mandelbrot_at_point(cx, cy, max_iters);
            row.push(escaped_at);
        }

        rows.push(row);
    }

    rows
}

/**
 * \brief Checks if the input point is in the Mandelbrot set
 * \details Called at every pixel (e.g., every row and column that's printed to stdout)
 * \param cx the X coordinate of the point
 * \param cy the Y coordinate of the point
 * \param max_iters the maximum number of iterations to perform the check
 * \returns The escape value for the input point
 */
fn mandelbrot_at_point( cx: f64,
                        cy: f64,
                        max_iters: usize,
                        ) -> usize {
    let mut z = Complex { re: 0.0, im: 0.0 }; // initializes a complex number at the origin with real (re) and imaginary (im) parts at 0.0
    let c = Complex::new(cx, cy); // initializes a complex number from the coordinates provided as function arguments

    for i in 0..=max_iters {
        if z.norm() > 2.0 { // checks the escape condition and calculates the distance from the origin (0,0), an absolute value of a complex number
            return i;
        }

        z = z * z + c; // repeatedly mutates z to check whether c lies within the Mandelbrot set
    }

    max_iters // as i is no longer in scope, we fall back to max_iters
}

/**
 * \brief Render the Mandelbrot set in a terminal with ASCII characters
 */
fn render_mandelbrot(escape_vals: Vec<Vec<usize>>) {
    for row in escape_vals {
        let mut line = String::with_capacity(row.len());

        for column in row {
            let val = match column {
                0..=2 => ' ',
                3..=5 => '.',
                6..=10 => '•',
                11..=30 => '*',
                31..=100 => '+',
                101..=200 => 'x',
                201..=400 => '$',
                401..=700 => '#',
                _ => '%',
            };

            line.push(val);
        }

        println!("{}", line);
    }
}

fn main() {
    let mandelbrot = calculate_mandelbrot(1000, -2.0, 1.0, -1.0, 1.0, 100, 24);
    render_mandelbrot(mandelbrot);
}
