use super::Options;
use crate::sprites::Rect;

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

/**
 * LayoutConstants is an internal structure holding constants which vary based upon the skill level.
 */
#[derive(Debug)]
struct LayoutConstants {
    height: u32,
    width: u32,
    panel_offset: u32,
}

const BEGINNER_LAYOUT_CONSTANTS: LayoutConstants = LayoutConstants {
    height: BEGINNER_HEIGHT,
    width: BEGINNER_WIDTH,
    panel_offset: BEGINNER_DIGIT_PANEL_OFFSET,
};
const INTERMEDIATE_LAYOUT_CONSTANTS: LayoutConstants = LayoutConstants {
    height: INTERMEDIATE_HEIGHT,
    width: INTERMEDIATE_WIDTH,
    panel_offset: DEFAULT_DIGIT_PANEL_OFFSET,
};
const EXPERT_LAYOUT_CONSTANTS: LayoutConstants = LayoutConstants {
    height: EXPERT_HEIGHT,
    width: EXPERT_WIDTH,
    panel_offset: DEFAULT_DIGIT_PANEL_OFFSET,
};

/**
 * Layout holds all of the information and provides all of the layout information for minesweeper. Therer are const layouts for each
 * skill level. In theory, the methods should compile down to constants or a Rect which can be constant.
 */
#[derive(Debug, Copy, Clone)]
pub struct Layout {
    pub options: &'static Options,
    constants: &'static LayoutConstants,
}

pub const BEGINNER_LAYOUT: Layout = Layout {
    options: &super::options::BEGINNER_OPTIONS,
    constants: &BEGINNER_LAYOUT_CONSTANTS,
};
pub const INTERMEDIATE_LAYOUT: Layout = Layout {
    options: &super::options::INTERMEDIATE_OPTIONS,
    constants: &INTERMEDIATE_LAYOUT_CONSTANTS,
};
pub const EXPERT_LAYOUT: Layout = Layout {
    options: &super::options::EXPERT_OPTIONS,
    constants: &EXPERT_LAYOUT_CONSTANTS,
};

impl Layout {
    pub fn new() -> Self {
        match Options::new().level() {
            super::options::BEGINNER => BEGINNER_LAYOUT,
            super::options::INTERMEDIATE => INTERMEDIATE_LAYOUT,
            super::options::EXPERT => EXPERT_LAYOUT,
            &_ => BEGINNER_LAYOUT,
        }
    }

    pub fn tile_side() -> u32 {
        TILE_SIDE
    }

    pub fn height(&self) -> u32 {
        self.constants.height
    }

    pub fn width(&self) -> u32 {
        self.constants.width
    }

    pub fn grid(&self) -> Rect {
        let width = self.options.columns * TILE_SIDE as i16;
        let height = self.options.rows * TILE_SIDE as i16;
        Rect::new(GRID_LEFT, GRID_TOP, width as u32, height as u32)
    }

    pub fn tile(&self, bounding_box: Rect, index: i16) -> Rect {
        let (row, column) = self.options.row_column(index as u16);
        let left = bounding_box.left() + (column * TILE_SIDE as i16) as i32;
        let top = bounding_box.top() + (row * TILE_SIDE as i16) as i32;
        Rect::new(left, top, TILE_SIDE, TILE_SIDE)
    }

    pub fn grid_tile(&self, index: i16) -> Rect {
        let bounding_box = self.grid();
        let (row, column) = self.options.row_column(index as u16);
        let left = bounding_box.left() + (column * TILE_SIDE as i16) as i32;
        let top = bounding_box.top() + (row * TILE_SIDE as i16) as i32;
        Rect::new(left, top, TILE_SIDE, TILE_SIDE)
    }

    pub fn timer_digit_panel(&self) -> Rect {
        let left = self.width() - self.digit_panel_offset() - DIGIT_PANEL_WIDTH;
        self.digit_panel(left, TIMER_TOP)
    }

    pub fn flag_digit_panel(&self) -> Rect {
        let left = self.digit_panel_offset();
        self.digit_panel(left, FLAG_TOP)
    }

    pub fn digit_panel_offset(&self) -> u32 {
        self.constants.panel_offset
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

    fn digit_panel(&self, left: u32, top: u32) -> Rect {
        Rect::new(
            left as i32,
            top as i32,
            DIGIT_PANEL_WIDTH,
            DIGIT_PANEL_HEIGHT,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Rect;

    #[test]
    fn test_attributes() {
        let layout = super::BEGINNER_LAYOUT;
        let bounding_box = layout.grid();
        assert_eq!(layout.height(), super::BEGINNER_HEIGHT);
        assert_eq!(layout.width(), super::BEGINNER_WIDTH);
        assert_eq!(layout.grid(), Rect::new(15, 81, 180, 180));
        assert_eq!(
            layout.tile(bounding_box, 0),
            Rect::new(15, 81, super::TILE_SIDE, super::TILE_SIDE)
        );
        assert_eq!(
            layout.tile(bounding_box, 80),
            Rect::new(175, 241, super::TILE_SIDE, super::TILE_SIDE)
        );
        assert_eq!(layout.timer_digit_panel(), Rect::new(129, 21, 65, 37));
        assert_eq!(layout.flag_digit_panel(), Rect::new(16, 21, 65, 37));
        assert_eq!(
            layout.digit_panel_offset(),
            super::BEGINNER_DIGIT_PANEL_OFFSET
        );
        assert_eq!(layout.timer_digit(0), Rect::new(131, 23, 19, 33));
        assert_eq!(layout.timer_digit(1), Rect::new(152, 23, 19, 33));
        assert_eq!(layout.timer_digit(2), Rect::new(173, 23, 19, 33));
        assert_eq!(layout.flag_digit(0), Rect::new(18, 23, 19, 33));
        assert_eq!(layout.flag_digit(1), Rect::new(39, 23, 19, 33));
        assert_eq!(layout.flag_digit(2), Rect::new(60, 23, 19, 33));
        assert_eq!(
            layout.face(),
            Rect::new(84, 19, super::FACE_WIDTH, super::FACE_HEIGHT)
        );
    }
}
