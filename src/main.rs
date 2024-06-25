use std::collections::{HashMap, HashSet};

pub trait Room {
    fn to_coords(&self) -> Vec<Coord>;
}

#[derive(Debug, Eq, PartialEq)]
pub struct Coord {
    x: i32,
    y: i32, // Recall that smaller y means more at the top.
}

impl Coord {
    /// Creates a new [`Coord`].
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Room for Coord {
    fn to_coords(&self) -> Vec<Coord> {
        let mut v = Vec::new();
        let copy = Coord::new(self.x, self.y);
        v.push(copy);
        return v;
    }
}

pub struct Rect {
    left_x: i32,
    top_y: i32,
    width: u32,
    height: u32,
}

impl Room for Rect {
    fn to_coords(&self) -> Vec<Coord> {
        if self.width == 0 || self.height == 0 {
            return Vec::new(); // TODO would be an immutable empty singleton be possible here?
        } else {
            let mut v = Vec::new();
            for x_shift in 0..self.width {
                for y_shift in 0..self.height {
                    let bigger_x = self.left_x.checked_add_unsigned(x_shift);
                    let bigger_y = self.top_y.checked_add_unsigned(y_shift);
                    match (bigger_x, bigger_y) {
                        (Some(bigger_x), Some(bigger_y)) => v.push(Coord::new(bigger_x, bigger_y)),
                        _ => (), // Don't extend on overflows
                    }
                }
            }
            return v;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Coord;
    use crate::Room;

    #[test]
    fn coord_to_coords() {
        let c = Coord::new(0, 0);
        let coords = c.to_coords();
        let coords_head = coords.get(0);
        match coords_head {
            None => panic!("Coord.to_coords should return a non-empty list"),
            Some(head) => assert_eq!(c, *head),
        }
    }
}

mod proptests {
    use crate::Coord;
    use crate::Rect;
    use crate::Room;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn coord_to_coords(x in 0..256, y in 0..256) {
            let c = Coord::new(x, y);
            let coords = c.to_coords();
            let coords_head = coords.get(0);
            match coords_head {
                None => panic!("Coord.to_coords should return a non-empty list"),
                Some(head) => assert_eq!(c, *head),
            }
        }

        #[test]
        fn rect_to_coords_size(width in 0..256u32, height in 0..256u32, left_x in 0..256, top_y in 0..256) { // Small sizes, because we don't want to run into overflows in Rect::to_coords()
            let r = Rect{left_x, top_y, width, height};
            let coords = r.to_coords();
            match usize::try_from(r.width * r.height) {
                Ok(area_size) => assert_eq!(coords.len(), area_size),
                Err(_) => panic!("Unexpected conversion failure"),
            }
        }
    }
}

fn main() {
    println!("Generating a dungeon for you my lord");

    let width: u32 = 80;
    let height: u32 = 50;

    let header = "#".repeat(width as usize);
    let inside = " ".repeat(width as usize - 2);
    let inside2 = format!("#{inside}#");

    // Print top wall
    println!("{}", header);

    let mut rooms: Vec<&dyn Room> = Vec::new();
    let room1: Rect = Rect {
        left_x: 2,
        top_y: 2,
        width: 8,
        height: 4,
    };
    rooms.push(&room1);

    // Because we draw maps by iterating over y, then x; we populate this assymetric map first.
    // We want the map to contain unsigned integers, because the part of the map we draw
    // only consists of positive coordinates.
    let mut y_to_xs: HashMap<u32, Vec<u32>> = HashMap::new();
    for room in rooms.iter() {
        let coords = room.to_coords();
        for coord in coords {
            let x_u32: Result<u32, _> = coord.x.try_into();
            let y_u32: Result<u32, _> = coord.y.try_into();
            match (x_u32, y_u32) {
                (Err(_), _) => (),
                (_, Err(_)) => (),
                (Ok(positive_x), Ok(positive_y)) => {
                    let opt_xs = y_to_xs.get_mut(&positive_x);
                    match opt_xs {
                        Some(xs) => xs.push(positive_x),
                        None => {
                            let mut xs = Vec::new();
                            xs.push(positive_x);
                            y_to_xs.insert(positive_y, xs);
                            ()
                        }
                    }
                }
            }
        }
    }

    // Print inside
    for y in 1..(height - 1) {
        match y_to_xs.get(&y) {
            Some(xs) => {
                let to_print: HashSet<&u32> = HashSet::from_iter(xs);
                // left wall
                print!("#");
                for x in 1..width - 1 {
                    if to_print.contains(&x) {
                        print!("#")
                    } else {
                        print!(" ")
                    }
                }
                // right wall and wrapping to new line
                println!("#");
            }
            None => println!("{}", inside2),
        }
    }

    // Print bot wall
    println!("{}", header);
}
