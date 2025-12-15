pub enum Part {
    One,
    Two,
}

/// A 2D grid stored in a flat Vec in row-major order:
/// idx = y * width + x
pub struct Matrix<T> {
    width: usize,
    height: usize,
    cells: Vec<T>,
}

impl<T> Matrix<T> {
    /// Construct from Vec<Vec<T>> (rows). Must be rectangular and non-empty.
    pub fn from_rows(rows: Vec<Vec<T>>) -> Self {
        assert!(!rows.is_empty(), "rows must not be empty");
        let height = rows.len();
        let width = rows[0].len();
        assert!(width > 0, "rows must not contain empty rows");

        for (i, r) in rows.iter().enumerate() {
            assert_eq!(r.len(), width, "row {i} length != width");
        }

        let cells = rows.into_iter().flatten().collect::<Vec<T>>();
        Self { width, height, cells }
    }

    /// Construct by providing exact dimensions and a flat vec of size width*height.
    pub fn from_flat(width: usize, height: usize, cells: Vec<T>) -> Self {
        assert!(width > 0 && height > 0, "width/height must be > 0");
        assert_eq!(cells.len(), width * height, "cells length != width*height");
        Self { width, height, cells }
    }

    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }

    fn wrap_coord(n: i32, size: usize) -> usize {
        (n as i64).rem_euclid(size as i64) as usize
    }

    fn wrap_xy(&self, x: i32, y: i32) -> (usize, usize) {
        (Self::wrap_coord(x, self.width), Self::wrap_coord(y, self.height))
    }

    fn idx_wrapped(&self, x: i32, y: i32) -> usize {
        let (xw, yw) = self.wrap_xy(x, y);
        yw * self.width + xw
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height
    }

    pub fn at_checked(&self, x: i32, y: i32) -> Option<Point<'_, T>> {
        self.in_bounds(x, y).then(|| Point { m: self, x, y })
    }

    pub fn at(&self, x: i32, y: i32) -> Point<'_, T> {
        let (xw, yw) = self.wrap_xy(x, y);
        Point { m: self, x: xw as i32, y: yw as i32 }
    }

    pub fn at_mut(&mut self, x: i32, y: i32) -> PointMut<'_, T> {
        let (xw, yw) = self.wrap_xy(x, y);
        PointMut { m: self, x: xw as i32, y: yw as i32 }
    }

    pub fn get(&self, x: i32, y: i32) -> &T {
        let i = self.idx_wrapped(x, y);
        &self.cells[i]
    }

    pub fn get_mut(&mut self, x: i32, y: i32) -> &mut T {
        let i = self.idx_wrapped(x, y);
        &mut self.cells[i]
    }
}

impl<T: Clone> Clone for Matrix<T> {
    fn clone(&self) -> Self {
        Self {
            width: self.width,
            height: self.height,
            cells: self.cells.clone(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Point<'a, T> {
    m: &'a Matrix<T>,
    x: i32,
    y: i32,
}

impl<'a, T> Point<'a, T> {
    pub fn x(&self) -> i32 { self.x }
    pub fn y(&self) -> i32 { self.y }

    pub fn right(self, dx: i32) -> Self { self.m.at(self.x + dx, self.y) }
    pub fn left(self, dx: i32) -> Self { self.m.at(self.x - dx, self.y) }
    pub fn down(self, dy: i32) -> Self { self.m.at(self.x, self.y + dy) }
    pub fn up(self, dy: i32) -> Self { self.m.at(self.x, self.y - dy) }

    pub fn peek_right(&self, dx: i32) -> Point<'a, T> { self.m.at(self.x + dx, self.y) }
    pub fn peek_left(&self, dx: i32) -> Point<'a, T> { self.m.at(self.x - dx, self.y) }
    pub fn peek_down(&self, dy: i32) -> Point<'a, T> { self.m.at(self.x, self.y + dy) }
    pub fn peek_up(&self, dy: i32) -> Point<'a, T> { self.m.at(self.x, self.y - dy) }

    pub fn neighbors8_including_self(self) -> [Point<'a, T>; 9] {
        let m = self.m;
        let x = self.x;
        let y = self.y;

        [
            m.at(x - 1, y - 1), m.at(x, y - 1), m.at(x + 1, y - 1),
            m.at(x - 1, y),     m.at(x, y),     m.at(x + 1, y),
            m.at(x - 1, y + 1), m.at(x, y + 1), m.at(x + 1, y + 1),
        ]
    }

    pub fn neighbors8(self) -> [Point<'a, T>; 8] {
        let m = self.m;
        let x = self.x;
        let y = self.y;

        [
            m.at(x - 1, y - 1), m.at(x, y - 1), m.at(x + 1, y - 1),
            m.at(x - 1, y), /* skip center */ m.at(x + 1, y),
            m.at(x - 1, y + 1), m.at(x, y + 1), m.at(x + 1, y + 1),
        ]
    }

    pub fn neighbors8_bounded(&self) -> Vec<Point<'a, T>> {
        let mut out = Vec::with_capacity(8);

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue; // exclude self
                }
                let nx = self.x + dx;
                let ny = self.y + dy;

                if let Some(p) = self.m.at_checked(nx, ny) {
                    out.push(p);
                }
            }
        }
        out
    }

    pub fn get(&self) -> &'a T {
        self.m.get(self.x, self.y)
    }
}

pub struct PointMut<'a, T> {
    m: &'a mut Matrix<T>,
    x: i32,
    y: i32,
}

impl<'a, T> PointMut<'a, T> {
    pub fn x(&self) -> i32 { self.x }
    pub fn y(&self) -> i32 { self.y }

    pub fn right(&mut self, dx: i32) -> &mut Self {
        let w = self.m.width;
        self.x = Matrix::<T>::wrap_coord(self.x + dx, w) as i32;
        self
    }
    pub fn left(&mut self, dx: i32) -> &mut Self {
        let w = self.m.width;
        self.x = Matrix::<T>::wrap_coord(self.x - dx, w) as i32;
        self
    }
    pub fn down(&mut self, dy: i32) -> &mut Self {
        let h = self.m.height;
        self.y = Matrix::<T>::wrap_coord(self.y + dy, h) as i32;
        self
    }
    pub fn up(&mut self, dy: i32) -> &mut Self {
        let h = self.m.height;
        self.y = Matrix::<T>::wrap_coord(self.y - dy, h) as i32;
        self
    }

    pub fn peek_right(&self, dx: i32) -> Point<'_, T> {
        let m = &*self.m;
        m.at(self.x + dx, self.y)
    }
    pub fn peek_left(&self, dx: i32) -> Point<'_, T> {
        let m = &*self.m;
        m.at(self.x - dx, self.y)
    }
    pub fn peek_down(&self, dy: i32) -> Point<'_, T> {
        let m = &*self.m;
        m.at(self.x, self.y + dy)
    }
    pub fn peek_up(&self, dy: i32) -> Point<'_, T> {
        let m = &*self.m;
        m.at(self.x, self.y - dy)
    }

    pub fn get(&self) -> &T {
        let m = &*self.m;
        m.get(self.x, self.y)
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.m.get_mut(self.x, self.y)
    }

    pub fn set(&mut self, value: T) {
        *self.get_mut() = value;
    }
}