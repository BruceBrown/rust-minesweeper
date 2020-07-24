use super::Options;
use sdl2::rect::Rect;

// constants used in GUI layout
const GRID_LEFT: i32 = 15;
const GRID_TOP: i32 = 81;
const TILE_SIDE: u32 = 20;
const DIGIT_PANEL_WIDTH: u32 = 65;
const DIGIT_PANEL_HEIGHT: u32 = 37;
const TIMER_TOP: u32 = 21;
const FLAG_TOP: u32 = 21;
const FACE_TOP: u32 = 19;
const FACE_WIDTH: u32 = 42;
const FACE_HEIGHT: u32 = 42;

const BEGINNER_HEIGHT: u32 = 276;
const BEGINNER_WIDTH: u32 = 210;
const INTERMEDIATE_HEIGHT: u32 = 416;
const INTERMEDIATE_WIDTH: u32 = 350;
const EXPERT_HEIGHT: u32 = 416;
const EXPERT_WIDTH: u32 = 630;

const DIGIT_WIDTH: u32 = 19;
const DIGIT_HEIGHT: u32 = 33;
const DIGIT_PANEL_HORZ_MARGIN: u32 = (DIGIT_PANEL_WIDTH - (3 * DIGIT_WIDTH)) / 4;
const DIGIT_PANEL_VERT_MARGIN: u32 = (DIGIT_PANEL_HEIGHT - DIGIT_HEIGHT) / 2;

const BEGINNER_DIGIT_PANEL_OFFSET: u32 = 16;
const DEFAULT_DIGIT_PANEL_OFFSET: u32 = 20;

#[derive(Debug)]
pub struct Layout {
    pub options: Options,
}

impl Layout {
    pub fn new() -> Self {
        Self {
            options: Options::new(),
        }
    }

    pub fn tile_side() -> u32 {
        TILE_SIDE
    }

    pub fn height(&self) -> u32 {
        match self.options.level {
            "beginner" => BEGINNER_HEIGHT,
            "intermediate" => INTERMEDIATE_HEIGHT,
            "expert" => EXPERT_HEIGHT,
            &_ => BEGINNER_HEIGHT,
        }
    }

    pub fn width(&self) -> u32 {
        match self.options.level() {
            "beginner" => BEGINNER_WIDTH,
            "intermediate" => INTERMEDIATE_WIDTH,
            "expert" => EXPERT_WIDTH,
            &_ => BEGINNER_WIDTH,
        }
    }

    pub fn grid(&self) -> Rect {
        let width = self.options.columns * TILE_SIDE as i16;
        let height = self.options.rows * TILE_SIDE as i16;
        Rect::new(GRID_LEFT, GRID_TOP, width as u32, height as u32)
    }

    pub fn tile(&self, bounding_box: Rect, index: i16) -> Rect {
        let (row, column) = self.options.row_column(index as u16);
        let left = bounding_box.x + (column * TILE_SIDE as i16) as i32;
        let top = bounding_box.y + (row * TILE_SIDE as i16) as i32;
        Rect::new(left, top, TILE_SIDE, TILE_SIDE)
    }

    pub fn timer_digit_panel(&self) -> Rect {
        let left = self.width() - self.digit_panel_offset() - DIGIT_PANEL_WIDTH;
        self.digit_panel(left, TIMER_TOP)
    }

    pub fn flag_digit_panel(&self) -> sdl2::rect::Rect {
        let left = self.digit_panel_offset();
        self.digit_panel(left, FLAG_TOP)
    }

    pub fn digit_panel(&self, left: u32, top: u32) -> sdl2::rect::Rect {
        sdl2::rect::Rect::new(
            left as i32,
            top as i32,
            DIGIT_PANEL_WIDTH,
            DIGIT_PANEL_HEIGHT,
        )
    }

    pub fn digit_panel_offset(&self) -> u32 {
        match self.options.level() {
            "beginner" => BEGINNER_DIGIT_PANEL_OFFSET,
            "intermediate" => DEFAULT_DIGIT_PANEL_OFFSET,
            "expert" => DEFAULT_DIGIT_PANEL_OFFSET,
            &_ => BEGINNER_DIGIT_PANEL_OFFSET,
        }
    }

    pub fn timer_digit(&self, position: u32) -> Rect {
        let mut left = self.width() - self.digit_panel_offset() - DIGIT_PANEL_WIDTH;
        left += DIGIT_PANEL_HORZ_MARGIN * (position + 1) + DIGIT_WIDTH * position;
        let top = TIMER_TOP + DIGIT_PANEL_VERT_MARGIN;
        Rect::new(left as i32, top as i32, DIGIT_WIDTH, DIGIT_HEIGHT)
    }

    pub fn flag_digit(&self, position: u32) -> Rect {
        let mut left = self.digit_panel_offset();
        left += DIGIT_PANEL_HORZ_MARGIN * (position + 1) + DIGIT_WIDTH * position;
        let top = FLAG_TOP + DIGIT_PANEL_VERT_MARGIN;
        Rect::new(left as i32, top as i32, DIGIT_WIDTH, DIGIT_HEIGHT)
    }

    pub fn face(&self) -> Rect {
        let left = self.width() / 2 - FACE_WIDTH / 2;
        Rect::new(left as i32, FACE_TOP as i32, FACE_WIDTH, FACE_HEIGHT)
    }
}
