use std::fmt;

struct Task {
    id : i32,
    name: String,
    compTime : i32,
    parent: vec<i32>,
    child: vec<i32>>
    deadline: u32,
    isLeaf: bool,
    deadline: u32,
    level: i32
}

impl Task {
    fn new (&self, id:i32, compTime: i32, level: i32) {
        self.id = id;
        self.compTime = compTime;
        self.level = level;
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}