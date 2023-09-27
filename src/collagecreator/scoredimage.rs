use image::RgbaImage;
use crate::collagecreator::mutatedimage::MutatedImage;

#[derive(Clone)]
pub struct ScoredImage<'a> {
    pub mutated: MutatedImage<'a>,
    pub attempt: RgbaImage,
    pub closeness: i32
}