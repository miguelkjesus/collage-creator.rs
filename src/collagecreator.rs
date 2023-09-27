#[path = "utils.rs"]
#[macro_use]
mod utils;

use image::RgbaImage;
use rand::{thread_rng, Rng};

pub struct TrialOptions {
    pub init_attempts: u32,
    pub survivors: u32,
    pub offspring: u32,
    pub generations: u32,
    pub offspring_mutation_modifier: f32
}

struct MutatedImage<'a> {
    pub image: &'a RgbaImage,
    pub pos: (i32, i32),
    pub angle: u32,
    pub size: f32,
}

impl<'a> MutatedImage<'a> {
    pub fn randomized(image: &'a RgbaImage, scene_dims: (i32, i32)) -> Self {
        let mut rng = thread_rng();
        let img_dims = tuple_as!(image.dimensions(), i32);
        let pos = (
            rng.gen_range(-img_dims.0..scene_dims.0),
            rng.gen_range(-img_dims.0..scene_dims.1),
        );
        let angle = rng.gen_range(0..360);
        let size = rng.gen_range(0.001..1000.);
        MutatedImage { image, pos, angle, size }
    }

    pub fn mutate(&mut self, rel_pos_variation: f32, rel_size_variation: f32, rel_angle_variation: f32) {
        let mut rng = thread_rng();
        let pos_variation = (self.size as f32 * rel_pos_variation) as i32;
        self.pos = (
            rng.gen_range(self.pos.0 - pos_variation..self.pos.0 + pos_variation),
            rng.gen_range(self.pos.1 - pos_variation..self.pos.1 + pos_variation),
        );
        self.angle = rng.gen_range(
            (self.angle as f32 / rel_angle_variation) as u32
            ..(self.angle as f32 * rel_angle_variation) as u32
        );
        self.size = rng.gen_range(self.size / rel_size_variation..self.size * rel_size_variation);
    }

    pub fn apply(scene: &RgbaImage) -> RgbaImage {
        todo!()
    }
}

pub struct CollageCreator<'a> {
    pub target: &'a RgbaImage,
    pub sources: &'a Vec<RgbaImage>,
    pub output: &'a mut RgbaImage,
    pub trial_options: TrialOptions
}

impl<'a> CollageCreator<'a> {
    pub fn start(&mut self, trials: u32) {
        for trial in 1..trials {
            let best_attempts: Vec<MutatedImage> = Vec::new();

            let mut att = 0;
            while att < self.trial_options.init_attempts {

                att += 1;
            }
        }
    }


}

// pub fn recreate<'a>(target: &'a RgbaImage, sources: Vec<&'a RgbaImage>, output: &'a mut RgbaImage, trial_options: Trial) {
//     CollageCreator {
//         target,
//         sources,
//         output,
//         trial: trial_options
//     };
// }