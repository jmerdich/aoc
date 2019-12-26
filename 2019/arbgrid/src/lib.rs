use std::cmp::{max, min};
use std::collections::BTreeMap;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub struct Coord2D(pub i32, pub i32);

impl Coord2D {
    fn bound_max(self, other: Self) -> Self {
        Coord2D(max(self.0, other.0), max(self.1, other.1))
    }
    fn bound_min(self, other: Self) -> Self {
        Coord2D(min(self.0, other.0), min(self.1, other.1))
    }

    pub fn new(x: i32, y: i32) -> Self {
        Coord2D(x, y)
    }

    pub fn up(self, count: i32) -> Self {
        Coord2D(self.0, self.1 + count)
    }
    pub fn down(self, count: i32) -> Self {
        Coord2D(self.0, self.1 - count)
    }
    pub fn left(self, count: i32) -> Self {
        Coord2D(self.0 - count, self.1)
    }
    pub fn right(self, count: i32) -> Self {
        Coord2D(self.0 + count, self.1)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArbGrid<T> {
    map: BTreeMap<Coord2D, T>,
    default: T,
    min: Option<Coord2D>,
    max: Option<Coord2D>,
}

impl<T> ArbGrid<T>
where
    T: Clone + Debug + PartialEq,
{
    pub fn new(default: T) -> ArbGrid<T> {
        ArbGrid {
            map: BTreeMap::new(),
            default,
            min: None,
            max: None,
        }
    }

    pub fn get(&self, loc: Coord2D) -> &T {
        self.map.get(&loc).unwrap_or(&self.default)
    }

    pub fn get_mut(&mut self, loc: Coord2D) -> &mut T {
        let default = self.default.clone(); // separate to appease clippy
        self.map.entry(loc).or_insert(default)
    }

    pub fn insert(&mut self, loc: Coord2D, value: T) -> Option<T> {
        if let Some(min) = self.min {
            self.min = Some(min.bound_min(loc));
        } else {
            self.min = Some(loc);
        }
        if let Some(max) = self.max {
            self.max = Some(max.bound_max(loc));
        } else {
            self.max = Some(loc);
        }
        self.map.insert(loc, value)
    }

    pub fn to_string(&self, convert_fn: &Fn(&T, Coord2D) -> char) -> String {
        if self.min.is_none() || self.max.is_none() {
            return "".to_string();
        }
        let mut out = String::new();
        for y in (self.min.unwrap().1..=self.max.unwrap().1).rev() {
            for x in self.min.unwrap().0..=self.max.unwrap().0 {
                let loc = Coord2D(x, y);
                out.push(convert_fn(self.get(loc), loc));
            }
            out.push('\n');
        }
        out
    }

    pub fn trim(&mut self) {
        let old_map = std::mem::replace(&mut self.map, BTreeMap::new());
        self.min = None;
        self.max = None;
        let default = self.default.clone();
        for (k, v) in old_map.into_iter().filter(|(_k, v)| *v != default) {
            self.insert(k, v);
        }
    }

    pub fn from_str(
        origin: Coord2D,
        s: &str,
        convert_fn: &Fn(char, Coord2D) -> Option<T>,
    ) -> ArbGrid<T>
    where
        T: Default,
    {
        let mut out = ArbGrid::new(T::default());
        let lines: Vec<&str> = s.lines().collect();
        let y_0 = lines.len(); // We start here and subtract, since we orient it in the bottom-left.
        for (i, line) in lines.iter().enumerate() {
            let y = y_0 - i;
            for (x, c) in line.chars().enumerate() {
                let loc = Coord2D(x as i32 + origin.0, y as i32 + origin.1);
                let res = convert_fn(c, loc);
                if let Some(cell) = res {
                    out.insert(loc, cell);
                }
            }
        }
        out
    }

    pub fn bounds(&self) -> Option<(Coord2D, Coord2D)> {
        if self.min.is_none() || self.max.is_none() {
            None
        } else {
            Some((self.min.unwrap(), self.max.unwrap()))
        }
    }

    pub fn iter(&self) -> impl Iterator + '_ {
        self.map.iter()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn round_trip() {}
}
