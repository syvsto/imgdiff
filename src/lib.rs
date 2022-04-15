mod utils;

use std::ops::Sub;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct Pixel {
    r: f32,
    g: f32,
    b: f32,
}

impl Sub<&Pixel> for &Pixel {
    type Output = Pixel;
    fn sub(self, rhs: &Pixel) -> Self::Output {
        Pixel {
            r: (self.r - rhs.r).abs(),
            g: (self.g - rhs.g).abs(),
            b: (self.b - rhs.b).abs(),
        }
    }
}

type Image = Vec<Pixel>;
type ImageRaw = Vec<f32>;

#[wasm_bindgen]
pub fn diff(img1: ImageRaw, img2: ImageRaw) -> ImageRaw {
    let i1 = from_raw(img1);
    let i2 = from_raw(img2);
    let diff = diff_image(i1, i2);
    as_raw(diff)
}

fn diff_image(img1: Image, img2: Image) -> Image {
    let mut out = Vec::with_capacity(img1.len());
    for i in 0..img1.len() {
        let px1 = &img1[i];
        let px2 = &img2[i];
        out.push(px1 - px2);
    }

    out
}

fn from_raw(v: ImageRaw) -> Image {
    let mut out = Vec::with_capacity(v.len() / 3);
    let mut i = 0;
    while i < v.len() {
        out.push(Pixel {
            r: v[i],
            g: v[i + 1],
            b: v[i + 2],
        });
        i += 3;
    }
    out
}
fn as_raw(img: Image) -> ImageRaw {
    let mut out = Vec::with_capacity(img.len() * 3);
    for p in &img {
        out.push(p.r);
        out.push(p.g);
        out.push(p.b);
    }
    out
}
