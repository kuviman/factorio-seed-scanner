use super::*;

pub mod specific {
    use super::*;

    pub struct Map {
        prefix_sums: Vec<Vec<i32>>,
        values: Vec<Vec<bool>>,
    }

    fn hex_to_rgba(hex: &str) -> image::Rgba<u8> {
        let hex = hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
        image::Rgba([r, g, b, 255])
    }

    impl Map {
        pub fn new(image: &image::RgbaImage, color: &str) -> Self {
            let color = hex_to_rgba(color);
            let mut prefix_sums = vec![vec![0; image.height() as usize]; image.width() as usize];
            let mut values = vec![vec![false; image.height() as usize]; image.width() as usize];
            for (x, y, pixel) in image.enumerate_pixels() {
                let x = x as usize;
                let y = y as usize;
                let value = *pixel == color;
                values[x][y] = value;
                let value = if value { 1 } else { 0 };
                prefix_sums[x][y] += value;
                if x > 0 {
                    prefix_sums[x][y] += prefix_sums[x - 1][y];
                }
                if y > 0 {
                    prefix_sums[x][y] += prefix_sums[x][y - 1];
                }
                if x > 0 && y > 0 {
                    prefix_sums[x][y] -= prefix_sums[x - 1][y - 1];
                }
            }
            Self {
                values,
                prefix_sums,
            }
        }
        pub fn aabb(&self) -> Aabb2<i32> {
            Aabb2 {
                min: vec2(0, 0),
                max: vec2(self.values.len() as i32, self.values[0].len() as i32),
            }
        }
        pub fn at(&self, pos: vec2<i32>) -> bool {
            if self.aabb().contains(pos) {
                self.values[pos.x as usize][pos.y as usize]
            } else {
                false
            }
        }
        fn prefix_sum_at(&self, pos: vec2<i32>) -> i32 {
            let pos = vec2(
                (pos.x).min(self.values.len() as i32) - 1,
                (pos.y).min(self.values[0].len() as i32) - 1,
            );
            if pos.x < 0 || pos.y < 0 {
                return 0;
            };
            self.prefix_sums[pos.x as usize][pos.y as usize]
        }
        pub fn query(&self, bb: Aabb2<i32>) -> i32 {
            self.prefix_sum_at(bb.top_right())
                - self.prefix_sum_at(bb.top_left())
                - self.prefix_sum_at(bb.bottom_right())
                + self.prefix_sum_at(bb.bottom_left())
        }
    }
}

pub mod colors {
    pub const IRON: &str = "698593";
    pub const COPPER: &str = "cc6236";
    pub const COAL: &str = "000000";
    pub const STONE: &str = "af9b6c";
    pub const BITERS: &str = "ff1919";
    pub const OIL: &str = "c633c4";
    pub const URANIUM: &str = "00b200";
}

pub struct Map {
    pub iron: specific::Map,
    pub copper: specific::Map,
    pub coal: specific::Map,
    pub stone: specific::Map,
    pub biters: specific::Map,
    pub oil: specific::Map,
    pub uranium: specific::Map,
}

fn extend(bb: Aabb2<i32>, dir: vec2<i32>, amount: i32) -> Aabb2<i32> {
    if dir.x < 0 {
        bb.extend_left(-dir.x * amount)
    } else if dir.y < 0 {
        bb.extend_down(-dir.y * amount)
    } else {
        bb.extend_positive(dir * amount)
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Patches {
    pub safe: usize,
    pub total: usize,
}

impl Map {
    pub fn new(image: &image::RgbaImage) -> Self {
        Self {
            iron: specific::Map::new(image, colors::IRON),
            copper: specific::Map::new(image, colors::COPPER),
            coal: specific::Map::new(image, colors::COAL),
            stone: specific::Map::new(image, colors::STONE),
            biters: specific::Map::new(image, colors::BITERS),
            oil: specific::Map::new(image, colors::OIL),
            uranium: specific::Map::new(image, colors::URANIUM),
        }
    }
    pub fn aabb(&self) -> Aabb2<i32> {
        self.iron.aabb()
    }
    pub fn is_safe(&self, area: Aabb2<i32>, safe_dist: i32) -> bool {
        self.biters.query(area.extend_uniform(safe_dist)) == 0
    }
    pub fn find_patches(&self, ore: &specific::Map, safe_dist: i32) -> Patches {
        let mut patches = Vec::<Aabb2<i32>>::new();
        let mut safe = 0;
        for pos in self.aabb().points() {
            if ore.at(pos) && !patches.iter().any(|patch| patch.contains(pos)) {
                let mut patch = Aabb2::point(pos).extend_positive(vec2(1, 1));
                loop {
                    let extended = patch.extend_uniform(1);
                    if ore.query(extended) > ore.query(patch) {
                        patch = extended;
                    } else {
                        break;
                    }
                }
                for dir in [vec2(-1, 0), vec2(1, 0), vec2(0, -1), vec2(0, 1)] {
                    let shrunk = extend(patch, dir, -1);
                    if ore.query(shrunk) == ore.query(patch) {
                        patch = shrunk;
                    } else {
                        break;
                    }
                }
                if self.is_safe(patch, safe_dist) {
                    safe += 1;
                }
                patches.push(patch.extend_uniform(safe_dist));
            }
        }
        Patches {
            total: patches.len(),
            safe,
        }
    }
}
