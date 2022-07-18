use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::time::Instant;
use utils::{Formation, InfoType};

pub mod utils;

const MULTIPLIER: isize = 0x5DEECE66D;
const MASK: isize = (1 << 48) - 1;
const PHI: usize = 0x9E3779B97F4A7C15;

fn get_texture_side(x: i32, y: i32, z: i32) -> i32 {
    let mut seed = get_coordinate_random(x, y, z);
    seed = (seed ^ MULTIPLIER) & MASK;
    ((seed * 0xBB20B4600A69 + 0x40942DE6BA) as u32 >> 16) as i32 & 0b1
}

fn get_coordinate_random(x: i32, y: i32, z: i32) -> isize {
    let mut l = (x * 3129871) as isize ^ (z as isize * 116129781) ^ y as isize;
    l = l * l * 42317861 + l * 11;
    l >> 16
}

fn get_texture_top(x: i32, y: i32, z: i32) -> i32 {
    let mut seed = get_coordinate_random(x, y, z);
    seed = (seed ^ MULTIPLIER) & MASK;
    seed = ((seed * 0xBB20B4600A69 + 0x40942DE6BA) as usize >> 16) as isize;
    (seed as i32).abs() & 0b11
}

fn stafford_mix_13(z: isize) -> isize {
    const A: usize = 0xBF58476D1CE4E5B9;
    const B: usize = 0x94D049BB133111EB;
    let mut z = (z ^ (z as usize >> 30) as isize) * A as isize;
    z = (z ^ (z as usize >> 27) as isize) * B as isize;
    z ^ (z as usize >> 31) as isize
}

fn sodium_random(mut seed: isize) -> isize {
    const A: usize = 0xff51afd7ed558ccd;
    const B: usize = 0xc4ceb9fe1a85ec53;
    seed ^= (seed as usize >> 33) as isize;
    seed *= A as isize;
    seed ^= (seed as usize >> 33) as isize;
    seed *= B as isize;
    seed ^= (seed as usize >> 33) as isize;
    seed += PHI as isize;
    let rand1 = stafford_mix_13(seed);
    seed += PHI as isize;
    let rand2 = stafford_mix_13(seed);
    rand1 + rand2
}

fn get_texture_top_sodium(x: i32, y: i32, z: i32) -> i32 {
    (sodium_random(get_coordinate_random(x, y, z)) as i32).abs() % 4
}

fn get_texture_side_sodium(x: i32, y: i32, z: i32) -> i32 {
    (sodium_random(get_coordinate_random(x, y, z)) as i32).abs() % 2
}

pub fn find(info: Formation) {
    let start = Instant::now();
    let tops_and_bottoms: Vec<_> = info
        .rotation_info
        .iter()
        .filter(|i| i.info_type == InfoType::TopsAndBottoms)
        .collect();
    let sides: Vec<_> = info
        .rotation_info
        .iter()
        .filter(|i| i.info_type == InfoType::Sides)
        .collect();
    let sodium = info.sodium;
    (info.x_min..info.x_max).into_par_iter().for_each(|x| {
        for z in info.z_min..=info.z_max {
            'next_attempt: for y in info.y_min..=info.y_max {
                for info in &tops_and_bottoms {
                    if sodium {
                        if info.rotation
                            != get_texture_top_sodium(x + info.x, y + info.y, z + info.z)
                        {
                            continue 'next_attempt;
                        } else {
                            if info.rotation != get_texture_top(x + info.x, y + info.y, z + info.z)
                            {
                                continue 'next_attempt;
                            }
                        }
                    }
                }
                for info in &sides {
                    if sodium {
                        if info.rotation
                            != get_texture_side_sodium(x + info.x, y + info.y, z + info.z)
                        {
                            continue 'next_attempt;
                        }
                    } else {
                        if info.rotation != get_texture_side(x + info.x, y + info.y, z + info.z) {
                            continue 'next_attempt;
                        }
                    }
                }
                println!("X: {x} Y: {y} Z: {z}");
            }
        }
    });
    let elapsed = start.elapsed();
    println!("{} seconds", elapsed.as_secs());
}

#[cfg(test)]
mod tests {
    use crate::{
        get_texture_side, get_texture_side_sodium, get_texture_top, get_texture_top_sodium,
    };

    #[test]
    fn vanilla_top() {
        assert_eq!(get_texture_top(-1, 0, -1), 0);
        assert_eq!(get_texture_top(0, 0, -1), 1);
        assert_eq!(get_texture_top(1, 0, -1), 0);
        assert_eq!(get_texture_top(-2, 0, 0), 2);
        assert_eq!(get_texture_top(-1, 0, 0), 0);
        assert_eq!(get_texture_top(0, 0, 0), 0);
        assert_eq!(get_texture_top(1, 0, 0), 1);
        assert_eq!(get_texture_top(-1, 0, 1), 0);
        assert_eq!(get_texture_top(0, 0, 1), 3);
        assert_eq!(get_texture_top(1, 0, 1), 0);
    }

    #[test]
    fn vanilla_side() {
        assert_eq!(get_texture_side(-6, 0, 0), 1);
        assert_eq!(get_texture_side(-5, 0, 0), 0);
        assert_eq!(get_texture_side(-4, 0, 0), 1);
        assert_eq!(get_texture_side(-3, 0, 0), 1);
        assert_eq!(get_texture_side(-2, 0, 0), 0);
        assert_eq!(get_texture_side(-1, 0, 0), 0);
        assert_eq!(get_texture_side(0, 0, 0), 0);
        assert_eq!(get_texture_side(1, 0, 0), 1);
        assert_eq!(get_texture_side(2, 0, 0), 0);
        assert_eq!(get_texture_side(3, 0, 0), 0);
        assert_eq!(get_texture_side(4, 0, 0), 1);
        assert_eq!(get_texture_side(5, 0, 0), 1);
        assert_eq!(get_texture_side(6, 0, 0), 0);
        assert_eq!(get_texture_side(7, 0, 0), 1);
        assert_eq!(get_texture_side(8, 0, 0), 0);
    }

    #[test]
    fn sodium_top() {
        assert_eq!(get_texture_top_sodium(0, -56, 0), 3);
        assert_eq!(get_texture_top_sodium(1, -56, 0), 3);
        assert_eq!(get_texture_top_sodium(2, -56, 0), 3);
        assert_eq!(get_texture_top_sodium(3, -56, 0), 3);
        assert_eq!(get_texture_top_sodium(4, -56, 0), 1);
        assert_eq!(get_texture_top_sodium(5, -56, 0), 3);
        assert_eq!(get_texture_top_sodium(6, -56, 0), 0);
        assert_eq!(get_texture_top_sodium(7, -56, 0), 2);
        assert_eq!(get_texture_top_sodium(8, -56, 0), 0);
        assert_eq!(get_texture_top_sodium(9, -56, 0), 3);
        assert_eq!(get_texture_top_sodium(10, -56, 0), 1);
        assert_eq!(get_texture_top_sodium(11, -56, 0), 1);
        assert_eq!(get_texture_top_sodium(12, -56, 0), 3);
        assert_eq!(get_texture_top_sodium(13, -56, 0), 3);
    }

    #[test]
    fn sodium_side() {
        assert_eq!(get_texture_side_sodium(0, -56, 1), 1);
        assert_eq!(get_texture_side_sodium(1, -56, 1), 1);
        assert_eq!(get_texture_side_sodium(2, -56, 1), 1);
        assert_eq!(get_texture_side_sodium(3, -56, 1), 0);
        assert_eq!(get_texture_side_sodium(4, -56, 1), 0);
        assert_eq!(get_texture_side_sodium(5, -56, 1), 0);
        assert_eq!(get_texture_side_sodium(6, -56, 1), 1);
        assert_eq!(get_texture_side_sodium(7, -56, 1), 1);
        assert_eq!(get_texture_side_sodium(8, -56, 1), 1);
        assert_eq!(get_texture_side_sodium(9, -56, 1), 0);
        assert_eq!(get_texture_side_sodium(10, -56, 1), 1);
        assert_eq!(get_texture_side_sodium(11, -56, 1), 1);
        assert_eq!(get_texture_side_sodium(12, -56, 1), 1);
        assert_eq!(get_texture_side_sodium(13, -56, 1), 1);
    }
}
