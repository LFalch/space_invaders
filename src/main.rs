use std::collections::HashSet;
use std::env::args;
use ramp::int::Int;

use image::GrayImage;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpaceInvader(pub Int);

impl SpaceInvader {
    #[inline]
    fn from_width(width: u8) -> SpaceInvader {
        let mut int = Int::from(0);
        int.set_bit(width as u32, true);
        int -= 1;
        SpaceInvader(int)
    }
    fn next(self, width: u8, extra: Option<Int>) -> SpaceInvader {
        let SpaceInvader(old) = self;
        let mut next = Int::from(0);
        let width_mask: Int = SpaceInvader::from_width(width).0;

        for i in 0..(width as u32) {
            let parents_odd = (i.saturating_sub(1)..=i.saturating_add(1))
                .map(|b| old.bit(b))
                .chain(extra.as_ref().map(|e| e.bit(i)))
                .fold(false, std::ops::BitXor::bitxor);
            next.set_bit(i, parents_odd);
        }
        SpaceInvader(next & width_mask)
    }
    // fn next_mut(&mut self, width: u8) {
    //     *self = self.clone().next(width);
    // }
}

// TODO use clap for argument parsing instead

fn main() -> Result<(), i32> {
    let mut args = args().skip(1).collect::<Vec<_>>();
    let time_travel = args[0] == "time_travel";
    if time_travel {
        args.remove(0);
    }
    let args = args;

    if args[0] == "all" {
        let arg = if time_travel {
            "-tt"
        } else {
            ""
        };
        for w in 1..=255 {
            main_program(vec![format!("out{}{}.png", arg, w), format!("{}", w), "65535".to_owned()], time_travel)?;
        }
        Ok(())
    }  else {
        
        main_program(args, time_travel)
    }
}

fn parse_width_and_start(s: &str) -> Result<(u8, SpaceInvader), i32> {
    if s.starts_with("0b") {
        let s = &s[2..];
        let w = s.len() as u8;

        Ok((w, SpaceInvader(Int::from_str_radix(s, 2).map_err(|_| 3)?)))
    } else {
        let w = s.parse().map_err(|_| 2)?;
        Ok((w, SpaceInvader::from_width(w)))
    }
}

fn main_program(args: Vec<String>, time_travel: bool) -> Result<(), i32> {
    let mut limit: Option<usize> = None;
    let (out_path, width, mut start) = match &*args {
        [out_path, s] => {
            let (w, si) = parse_width_and_start(s)?;
            (out_path, w, si)
        }
        [out_path, s, length_limit] => {
            let (w, si) = parse_width_and_start(s)?;
            limit = Some(length_limit.parse().map_err(|_| 4)?);

            (out_path, w, si)
        }
        _ => {
            eprintln!("Wrong amount of args");
            return Err(1);
        }
    };

    let mut rows = Vec::with_capacity(width as usize);
    
    println!("{:01$b}", start.0, width as usize);
    if time_travel {
        let mut set = HashSet::with_capacity(width as usize);

        loop {
            let old = rows.last().cloned();
            rows.push(start.0.clone());
            if let Some(old) = old.clone() {
                if !set.insert((old, start.clone())) {
                    break;
                }
            }
            if let Some(limit) = limit {
                if rows.len() >= limit {
                    break;
                }
            }

            start = start.next(width, old);
        }
    } else {
        let mut set = HashSet::with_capacity(width as usize);

        loop {
            rows.push(start.0.clone());
            if !set.insert(start.clone()) {
                break;
            }
            if let Some(limit) = limit {
                if rows.len() >= limit {
                    break;
                }
            }

            start = start.next(width, None);
        }
    }

    println!("length: {}", rows.len());
    let mut img = GrayImage::new(width as u32, rows.len() as u32);

    for (row, y) in rows.into_iter().zip(0..) {
        for i in 0..(width as u32) {
            let bit = row.bit(i);
            // println!("{}, {}: {}", i, y, bit);
            img.put_pixel(i, y, [(!bit) as u8 * 0xff].into())
        }
    }

    img.save(out_path).unwrap();

    Ok(())
}
