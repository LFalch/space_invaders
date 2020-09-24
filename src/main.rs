use std::collections::HashSet;
use std::env::args;

use image::GrayImage;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SpaceInvader(pub u128);

impl SpaceInvader {
    fn next(self, width: u8) -> SpaceInvader {
        let SpaceInvader(old) = self;
        let mut next = 0;
        let width_mask = (0b1 << width as u128) - 1;

        for i in 0..width {
            let mask = ((0b111 << i as u128) >> 1) & width_mask;
            if (mask & old).count_ones() & 1 == 1 {
                next |= 0b1 << i;
            }
        }
        SpaceInvader(next & width_mask)
    }
}

// TODO use clap for argument parsing instead

fn main() -> Result<(), i32> {
    // main_program(args().skip(1).collect::<Vec<_>>())
    for w in 1..128 {
        main_program(vec![format!("out{}.png", w), format!("{}", w), "10000".to_owned()])?;
    }
    Ok(())
}

fn main_program(args: Vec<String>) -> Result<(), i32> {
    let mut limit = None;
    let (out_path, width, mut start) = match &*args {
        [out_path, w] => {
            let w = w.parse().map_err(|_| 2)?;
            (out_path, w, SpaceInvader((0b1 << w as u128) - 1))
        }
        [out_path, w, length_limit] => {
            limit = Some(length_limit.parse().map_err(|_| 4)?);
            let w = w.parse().map_err(|_| 2)?;
            (out_path, w, SpaceInvader((0b1 << w as u128) - 1))
        }
        [out_path, w, length_limit, start] => {
            limit = Some(length_limit.parse().map_err(|_| 4)?);
            let w = w.parse().map_err(|_| 2)?;
            (out_path, w, SpaceInvader(start.parse().map_err(|_| 3)?))
        }
        _ => {
            eprintln!("Wrong amount of args");
            return Err(1);
        }
    };

    if width > 127 {
        eprintln!("Can't do that");
        return Err(10);
    }

    let mut set = HashSet::with_capacity(width as usize);
    let mut rows = Vec::with_capacity(width as usize);

    println!("{:01$b}", start.0, width as usize);
    loop {
        rows.push(start.0);
        if !set.insert(start) {
            break;
        }
        if let Some(limit) = limit {
            if rows.len() >= limit {
                break;
            }
        }

        start = start.next(width);
    }
    drop(set);
    let mut img = GrayImage::new(width as u32, rows.len() as u32);

    for (row, y) in rows.into_iter().zip(0..) {
        for i in 0..(width as u32) {
            let bit = (row >> (width - i as u8 - 1)) & 1;
            // println!("{}, {}: {}", i, y, bit);
            img.put_pixel(i, y, [(bit == 0) as u8 * 0xff].into())
        }
    }

    img.save(out_path).unwrap();

    Ok(())
}
