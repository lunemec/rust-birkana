extern crate svg;

use svg::node::element::Path;
use svg::node::element::path::Data;

static X_OFFSET: i32 = 40;
static Y_OFFSET: i32 = 10;

pub struct Rune {
    rune_data: RuneData,
    path: Path,
}

pub struct RuneData {
    data: Data,
    x_start: i32, 
    x_offset: i32,
    y_offset: i32,
    position: i32,
}

impl Rune {
    pub fn from_data(rune_data: RuneData) -> Self {
        Rune {
            rune_data: rune_data,
            path: Path::new(),
        }
    }

    pub fn to_path(mut self) -> svg::node::element::Path {
        self.path = self.path
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 1)
            .set("d", self.rune_data.data);
        return self.path;
    }
}

impl RuneData {
    pub fn new(position: i32, x_start: i32) -> Self {
        RuneData {
            data: Data::new(),
            x_start: x_start,   // This is used to ADD to x_offset on the first letter.
            x_offset: X_OFFSET, // Offset X from previous letter (or image border).
            y_offset: Y_OFFSET, // Offset Y from previous letter (or image border).
            position: position, // Letter position in the image starting from 0.
        }
    }

    pub fn x0(mut self) -> Self {
        self.data = self.data
            .move_to((self.x_offset * self.position + self.x_start, self.y_offset))
            .line_by((0, 100));
        return self;
    }

    pub fn x1(mut self) -> Self {
        self.data = self.data
            .move_to((self.x_offset * self.position + self.x_start, self.y_offset))
            .line_by((25, 25));
        return self;
    }

    pub fn x2(mut self) -> Self {
        self.data = self.data
            .move_to((self.x_offset * self.position + self.x_start + 25, self.y_offset + 25))
            .line_by((-25, 25));
        return self;
    }

    pub fn x4(mut self) -> Self {
        self.data = self.data
            .move_to((self.x_offset * self.position + self.x_start, self.y_offset + 50))
            .line_by((25, 25));
        return self;
    }

    pub fn x8(mut self) -> Self {
        self.data = self.data
            .move_to((self.x_offset * self.position + self.x_start + 25, self.y_offset + 75))
            .line_by((-25, 25));
        return self;
    }

    pub fn close(mut self) -> Self {
        self.data = self.data.close();
        return self;
    }
}