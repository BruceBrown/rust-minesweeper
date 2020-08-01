#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Rect {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}
impl Rect {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
    /// Returns the x-position of the left side of this rectangle.
    pub fn left(&self) -> i32 {
        self.x
    }

    /// Returns the x-position of the right side of this rectangle.
    pub fn right(&self) -> i32 {
        self.x + self.width as i32
    }

    /// Returns the y-position of the top side of this rectangle.
    pub fn top(&self) -> i32 {
        self.y
    }

    /// Returns the y-position of the bottom side of this rectangle.
    pub fn bottom(&self) -> i32 {
        self.y + self.height as i32
    }

    /// Returns the width of this rectangle.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the height of this rectangle.
    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn contains_point<P>(&self, point: P) -> bool
    where
        P: Into<(i32, i32)>,
    {
        let (x, y) = point.into();
        let inside_x = x >= self.left() && x < self.right();
        inside_x && (y >= self.top() && y < self.bottom())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Point {
        Point::new(x, y)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct SystemTime {
    pub time: u64,
}

impl SystemTime {
    pub fn now() -> Self {
        Self {
            time: js_sys::Date::now() as u64,
        }
    }

    pub fn elapsed(&self) -> Result<std::time::Duration, std::time::SystemTimeError> {
        Ok(std::time::Duration::from_millis(
            js_sys::Date::now() as u64 - self.time,
        ))
    }
}
