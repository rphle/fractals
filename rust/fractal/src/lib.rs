use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = WebAssembly)]
    static MEMORY: JsValue;
}

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq)]
pub enum FractalType {
    Mandelbrot = 0,
    Julia = 1,
    BurningShip = 2,
}

#[wasm_bindgen]
pub struct FractalRenderer {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
    palette: Vec<u32>,
    fractal_type: FractalType,
    julia_re: f64,
    julia_im: f64,
}

#[wasm_bindgen]
impl FractalRenderer {
    pub fn new(width: u32, height: u32) -> FractalRenderer {
        let size = (width * height * 4) as usize;
        let mut renderer = FractalRenderer {
            width,
            height,
            pixels: vec![0; size],
            palette: vec![],
            fractal_type: FractalType::Mandelbrot,
            julia_re: -0.7,
            julia_im: 0.27015,
        };
        renderer.regenerate_palette();
        renderer
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.pixels.resize((width * height * 4) as usize, 0);
    }

    pub fn set_fractal_type(&mut self, f_type: FractalType) {
        self.fractal_type = f_type;
    }

    pub fn set_julia_constant(&mut self, re: f64, im: f64) {
        self.julia_re = re;
        self.julia_im = im;
    }

    pub fn get_buffer_ptr(&self) -> *const u8 {
        self.pixels.as_ptr()
    }

    pub fn get_buffer_len(&self) -> usize {
        self.pixels.len()
    }

    pub fn render(&mut self, x_min: f64, x_max: f64, y_min: f64, y_max: f64, max_iter: u32) {
        let w = self.width as usize;
        let h = self.height as usize;
        let dx = (x_max - x_min) / self.width as f64;
        let dy = (y_max - y_min) / self.height as f64;

        let pixels_ptr = self.pixels.as_mut_ptr() as *mut u32;
        let palette_ptr = self.palette.as_ptr();
        let palette_len = self.palette.len();
        let palette_mask = (palette_len - 1) as usize;

        const BLACK: u32 = 0xFF000000;

        match self.fractal_type {
            FractalType::Mandelbrot => {
                for py in 0..h {
                    let c_im = y_max - py as f64 * dy;
                    let row_base = py * w;

                    for px in 0..w {
                        let c_re = x_min + px as f64 * dx;

                        // Cardioid and bulb check
                        let q = (c_re - 0.25) * (c_re - 0.25) + c_im * c_im;
                        let in_cardioid = q * (q + (c_re - 0.25)) < 0.25 * c_im * c_im;
                        let in_bulb = (c_re + 1.0) * (c_re + 1.0) + c_im * c_im < 0.0625;

                        if in_cardioid || in_bulb {
                            unsafe {
                                *pixels_ptr.add(row_base + px) = BLACK;
                            }
                            continue;
                        }

                        let iter = mandelbrot(c_re, c_im, max_iter);

                        unsafe {
                            let color = if iter >= max_iter as f64 {
                                BLACK
                            } else {
                                let idx = ((iter * 20.0 + 200.0) as usize) & palette_mask;
                                *palette_ptr.add(idx)
                            };
                            *pixels_ptr.add(row_base + px) = color;
                        }
                    }
                }
            }
            FractalType::Julia => {
                let julia_re = self.julia_re;
                let julia_im = self.julia_im;

                for py in 0..h {
                    let y = y_max - py as f64 * dy;
                    let row_base = py * w;

                    for px in 0..w {
                        let x = x_min + px as f64 * dx;
                        let iter = julia(x, y, julia_re, julia_im, max_iter);

                        unsafe {
                            let color = if iter >= max_iter as f64 {
                                BLACK
                            } else {
                                let idx = ((iter * 20.0 + 200.0) as usize) & palette_mask;
                                *palette_ptr.add(idx)
                            };
                            *pixels_ptr.add(row_base + px) = color;
                        }
                    }
                }
            }
            FractalType::BurningShip => {
                for py in 0..h {
                    let c_im = y_max - py as f64 * dy;
                    let row_base = py * w;

                    for px in 0..w {
                        let c_re = x_min + px as f64 * dx;
                        let iter = burning_ship(c_re, c_im, max_iter);

                        unsafe {
                            let color = if iter >= max_iter as f64 {
                                BLACK
                            } else {
                                let idx = ((iter * 20.0 + 200.0) as usize) & palette_mask;
                                *palette_ptr.add(idx)
                            };
                            *pixels_ptr.add(row_base + px) = color;
                        }
                    }
                }
            }
        }
    }

    fn regenerate_palette(&mut self) {
        self.palette.clear();
        let size = 2048;

        for i in 0..size {
            let t = i as f64 / size as f64;

            let r = (0.5 + 0.5 * (6.28318 * (1.0 * t + 0.00)).cos()) * 255.0;
            let g = (0.5 + 0.5 * (6.28318 * (0.7 * t + 0.15)).cos()) * 255.0;
            let b = (0.5 + 0.5 * (6.28318 * (0.4 * t + 0.20)).cos()) * 255.0;

            let color = 0xFF000000 | ((b as u32) << 16) | ((g as u32) << 8) | (r as u32);
            self.palette.push(color);
        }
    }
}

#[inline(always)]
fn mandelbrot(c_re: f64, c_im: f64, max_iter: u32) -> f64 {
    let mut z_re: f64 = 0.0;
    let mut z_im: f64 = 0.0;

    let mut old_re: f64 = 0.0;
    let mut old_im: f64 = 0.0;
    let mut period = 0;

    let mut i = 0usize;
    let max = max_iter as usize;

    while i < max {
        if period == 0 {
            old_re = z_re;
            old_im = z_im;
        }
        period += 1;
        if period > 16 {
            period = 0;
            if z_re == old_re && z_im == old_im {
                return max_iter as f64;
            }
        }

        let z_re2: f64 = z_re * z_re;
        let z_im2: f64 = z_im * z_im;

        if z_re2 + z_im2 > 256.0 {
            let log_zn: f64 = (z_re2 + z_im2).ln() * 0.5;
            let nu: f64 = log_zn.ln() * std::f64::consts::LOG2_E;
            return i as f64 + 1.0 - nu;
        }

        z_im = 2.0 * z_re * z_im + c_im;
        z_re = z_re2 - z_im2 + c_re;
        i += 1;
    }

    max_iter as f64
}

#[inline(always)]
fn julia(x: f64, y: f64, c_re: f64, c_im: f64, max_iter: u32) -> f64 {
    let mut z_re: f64 = x;
    let mut z_im: f64 = y;

    let mut i = 0usize;
    let max = max_iter as usize;

    while i < max {
        let z_re2: f64 = z_re * z_re;
        let z_im2: f64 = z_im * z_im;

        if z_re2 + z_im2 > 256.0 {
            let log_zn: f64 = (z_re2 + z_im2).ln() * 0.5;
            let nu: f64 = log_zn.ln() * std::f64::consts::LOG2_E;
            return i as f64 + 1.0 - nu;
        }

        z_im = 2.0 * z_re * z_im + c_im;
        z_re = z_re2 - z_im2 + c_re;
        i += 1;
    }

    max_iter as f64
}

#[inline(always)]
fn burning_ship(c_re: f64, c_im: f64, max_iter: u32) -> f64 {
    let mut z_re: f64 = 0.0;
    let mut z_im: f64 = 0.0;

    let mut i = 0usize;
    let max = max_iter as usize;

    while i < max {
        let z_re2: f64 = z_re * z_re;
        let z_im2: f64 = z_im * z_im;

        if z_re2 + z_im2 > 256.0 {
            let log_zn: f64 = (z_re2 + z_im2).ln() * 0.5;
            let nu: f64 = log_zn.ln() * std::f64::consts::LOG2_E;
            return i as f64 + 1.0 - nu;
        }

        let z_re_abs: f64 = z_re.abs();
        let z_im_abs: f64 = z_im.abs();
        z_im = 2.0 * z_re_abs * z_im_abs + c_im;
        z_re = z_re2 - z_im2 + c_re;
        i += 1;
    }

    max_iter as f64
}
