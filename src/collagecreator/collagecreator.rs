#[path = "../utils.rs"]
#[macro_use]
mod utils;

use crate::collagecreator::{mutatedimage::MutatedImage, scoredimage::ScoredImage, trialoptions::TrialOptions};
use image::RgbaImage;
use rand::{thread_rng, seq::SliceRandom};

pub struct CollageCreator<'a> {
    pub target: &'a RgbaImage,
    pub sources: &'a Vec<RgbaImage>,
    pub output: &'a mut RgbaImage,
    pub trial_options: TrialOptions
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