#[derive(Clone)]
pub struct Image {
    pub aspect_ratio: f64,
    pub width: u64,
    pub height: u64,
    pub samples: u64,
    pub max_depth: u64,
}

impl Image {
    pub fn new(aspect_ratio: f64, width: u64, samples: u64, max_depth: u64) -> Self {
        let height = (width as f64 / aspect_ratio) as u64;
        Self {
            aspect_ratio,
            width,
            height,
            samples,
            max_depth,
        }
    }
}
