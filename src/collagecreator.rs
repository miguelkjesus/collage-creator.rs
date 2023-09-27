#[path = "utils.rs"]
#[macro_use]
mod utils;

use image::RgbaImage;
use rand::{thread_rng, Rng, seq::SliceRandom};

pub struct TrialOptions {
    pub init_attempts: u32,
    pub survivors: u32,
    pub offspring: u32,
    pub generations: u32,
    pub offspring_mutation_modifier: f32
}

#[derive(Clone)]
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

    pub fn apply_to<'b>(&self, scene: &'b RgbaImage) -> RgbaImage {
        todo!()
    }
}

pub struct CollageCreator<'a> {
    pub target: &'a RgbaImage,
    pub sources: &'a Vec<RgbaImage>,
    pub output: &'a mut RgbaImage,
    pub trial_options: TrialOptions
}

#[derive(Clone)]
struct ScoredImage<'a> {
    mutated: MutatedImage<'a>,
    attempt: RgbaImage,
    closeness: i32
}

impl<'a> CollageCreator<'a> {
    pub fn start(&mut self, trials: u32) {
        for _trial in 1..trials {
            let best_attempts = self.create_initial_attempts();
        }
    }

    fn create_initial_attempts(&self) -> Vec<ScoredImage> {
        let mut best_attempts: Vec<ScoredImage> = vec![];
        let mut att_number = 0;

        while att_number < self.trial_options.init_attempts {
            let img = self.sources.choose(&mut thread_rng()).unwrap();
            let mutated = MutatedImage::randomized(img, tuple_as!(self.output.dimensions(), i32));
            let attempt_img = self.output.clone();
            mutated.apply_to(&attempt_img);
            let closeness = get_closeness(&attempt_img, &self.output);
            let mut attempt = ScoredImage { mutated, attempt: attempt_img, closeness };

            if (best_attempts.len() as u32) < self.trial_options.init_attempts {
                best_attempts.push(attempt);
            } else {
                let mut i = 0;
                while i < best_attempts.len() {
                    let best_att = &best_attempts[i];
                    if attempt.closeness > best_att.closeness {
                        let temp = best_attempts[i].clone();
                        best_attempts[i] = attempt;
                        attempt = temp;
                    }
                    i += 1;
                }
            }

            att_number += 1;
        }

        best_attempts
    }


}

fn get_closeness(a: &RgbaImage, b: &RgbaImage) -> i32 {
    let mut pixels_a = a.pixels();
    let mut pixels_b = b.pixels();
    let mut total = 0;

    while let Some(pxa) = pixels_a.next() {
        let pxb = pixels_b.next().unwrap();
        total += pxa.0[0] as i32 - pxb.0[0] as i32
            + pxa.0[1] as i32 - pxb.0[1] as i32
            + pxa.0[2] as i32 - pxb.0[2] as i32;
    }

    total
}

// pub fn recreate<'a>(target: &'a RgbaImage, sources: Vec<&'a RgbaImage>, output: &'a mut RgbaImage, trial_options: Trial) {
//     CollageCreator {
//         target,
//         sources,
//         output,
//         trial: trial_options
//     };
// }