use std::collections::HashSet;
use std::env::args;


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

fn main() -> Result<(), i32> {
    let (width, mut start) = match &*args().skip(1).collect::<Vec<_>>() {
        [] => (0, SpaceInvader(0)),
        [w] => {
            let w = w.parse().map_err(|_| 2)?;
            (w, SpaceInvader((0b1 << w as u128) - 1))
        }
        [w, start] => {
            let w = w.parse().map_err(|_| 2)?;
            (w, SpaceInvader(start.parse().map_err(|_| 3)?))
        }
        [_, _, ..] => {
            eprintln!("Too many args");
            return Err(1);
        }
    };

    let mut set = HashSet::with_capacity(width as usize);

    loop {
        println!("{:01$b}", start.0, width as usize);
        if !set.insert(start) {
            break
        }

        start = start.next(width);
    }


    Ok(())
}
