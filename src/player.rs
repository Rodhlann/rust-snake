use crate::point::Point;

pub const DIRS: (Point, Point, Point, Point) = (
  Point { x: 0, y: -1}, // N
  Point { x: 1, y: 0}, // E
  Point { x: 0, y: 1}, // S
  Point { x: -1, y: 0}, // W
);

pub struct Player {
  pub position: Point,
  pub points: i32,
  pub new_point: bool,
  pub tail: Vec<Point>
}

impl Player {
  pub fn update(&mut self, dir: &Point) {
    let current_pos = &self.position;
    let next_pos = Point { x: current_pos.x + dir.x, y: current_pos.y + dir.y };

    if self.points > 0 {
      if !self.tail.is_empty() && !self.new_point {
        self.tail.pop();
      }
      if self.new_point {
        self.new_point = false
      }
      let mut new = vec!(Point { x: current_pos.x, y: current_pos.y });
      new.extend(self.tail.iter().cloned());
      self.tail = new;
    }

    self.position = next_pos;
  }
}