use stdweb::unstable::TryInto;
use renderer::Renderer;

#[derive(Debug)]
pub struct Snake {
    head: Point,
    tail: Vec<Point>,
    food: Point,
    height: u32,
    width: u32,
    dir: Option<Direction>,
    next_dir: Option<Direction>,
    last_dir: Direction,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn is_opposite(self, other: Direction) -> bool {
        self == Direction::Up && other == Direction::Down
            || self == Direction::Down && other == Direction::Up
            || self == Direction::Right && other == Direction::Left
            || self == Direction::Left && other == Direction::Right
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Point(u32, u32);

impl Snake {
    pub fn new(width: u32, height: u32) -> Snake {
        let headx: u32 = js! { return Math.floor(Math.random() * @{width}) }
            .try_into()
            .unwrap();
        let heady: u32 = js! { return Math.floor(Math.random() * @{height}) }
            .try_into()
            .unwrap();
        let head = Point(headx, heady);
        let foodx: u32 = js! { return Math.floor(Math.random() * @{width}) }
            .try_into()
            .unwrap();
        let foody: u32 = js! { return Math.floor(Math.random() * @{height}) }
            .try_into()
            .unwrap();
        let food = Point(foodx, foody);
        let tail = Vec::new();
        Snake {
            head,
            tail,
            food,
            height,
            width,
            dir: None,
            next_dir: None,
            last_dir: Direction::Up,
        }
    }

    pub fn change_dir(&mut self, dir: Direction) {
        //if self.last_dir.is_opposite(dir) {
        //return;
        //}
        if !self.last_dir.is_opposite(dir) && self.dir.is_none() {
            self.dir = Some(dir)
        } else if self.dir.iter().any(|p| !p.is_opposite(dir)) {
            self.next_dir = Some(dir)
        }
    }

    pub fn update(&mut self) {
        let dir = self.dir.unwrap_or(self.last_dir);
        self.last_dir = dir;
        let new_head = match dir {
            Direction::Up => Point((self.head.0) % self.width, (self.head.1 - 1) % self.height),
            Direction::Down => Point((self.head.0) % self.width, (self.head.1 + 1) % self.height),
            Direction::Right => Point((self.head.0 + 1) % self.width, (self.head.1) % self.height),
            Direction::Left => Point((self.head.0 - 1) % self.width, (self.head.1) % self.height),
        };
        self.tail.insert(0, self.head);
        let last_end = self.tail.pop();
        if self.tail.contains(&new_head) {
            *self = Snake::new(self.width, self.height);
        }
        self.head = new_head;
        if self.head == self.food {
            let mut food = self.food;
            while food == self.head || self.tail.contains(&food) {
                let foodx: u32 = js! { return Math.floor(Math.random() * @{self.width}) }
                    .try_into()
                    .unwrap();
                let foody: u32 = js! { return Math.floor(Math.random() * @{self.height}) }
                    .try_into()
                    .unwrap();
                food = Point(foodx, foody);
            }
            self.food = food;
            last_end.map(|p| self.tail.push(p));
        }
        self.dir = self.next_dir.take();
    }

    pub fn draw(&self, renderer: &Renderer) {
        renderer.clear_all();
        renderer.draw(self.head.0, self.head.1, "green");
        for &Point(x, y) in &self.tail {
            renderer.draw(x, y, "brown");
        }
        renderer.draw(self.food.0, self.food.1, "red");
    }
}
