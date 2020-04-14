extern crate rand;
use std::io::Write;

pub fn run() {
    println!("Welcome to Rustsweeper!");
    let mut width = 10;
    let mut height = 10;
    match prompt("\nChoose your size:\n1. 10 x 10\n2. 20 x 10\n3. 20 x 20\n4. 40 x 20\n> ", 1, 4) {
        1 => {
            width = 10;
            height = 10;
        },
        2 => {
            width = 20;
            height = 10;
        },
        3 => {
            width = 20;
            height = 20;
        },
        4 => {
            width = 40;
            height = 20;
        },
        _ => {}
    };
    let mut difficulty = Difficulty::Easy;
    match prompt("\nChoose your difficulty:\n1. Easy - 10% bombs\n2. Medium - 20% bombs\n3. Hard - 40% bombs\n> ", 1, 3) {
        1 => {
            difficulty = Difficulty::Easy;
        },
        2 => {
            difficulty = Difficulty::Medium;
        },
        3 => {
            difficulty = Difficulty::Hard;
        },
        _ => {}
    };

    print!("\n");
    let mut game = Game::new(width, height, difficulty);
    while game.running {
        println!("{}", game.get_board());
        match prompt("\nWhat would you like to do?\n1. Uncover a tile\n2. Flag a tile\n> ", 1, 2) {
            1 => {
                let pos = prompt_coords(0, game.board.width - 1, 0, game.board.height - 1);
                println!("Uncovering ({}, {})", pos.x, pos.y);
                game.uncover(pos.x as usize, pos.y as usize);
            },
            2 => {
                let pos = prompt_coords(0, game.board.width - 1, 0, game.board.height - 1);
                println!("Flagging ({}, {})", pos.x, pos.y);
                game.flag(pos.x as usize, pos.y as usize);
            },
            _ => {}
        };
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum Cell {
    Bomb,
    Empty,
}
#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug, Clone, Copy)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
    pub valid: bool
}
impl Coord {
    pub fn new(x: isize, y: isize) -> Coord {
        Coord {
            x,
            y,
            valid: true
        }
    }
}

#[allow(dead_code)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub values: Vec<Vec<Cell>>,
}
impl Board {
    pub fn new(width: usize, height: usize) -> Board {
        let mut values = Vec::new();
        for _y in 0..height {
            let mut row = Vec::new();
            for _x in 0..width {
                row.push(Cell::Empty);
            }
            values.push(row);
        }
        let values = values;
        Board {
            width,
            height,
            values,
        }
    }
    pub fn get<'a>(&'a self, x: usize, y: usize) -> &'a Cell {
        &self.values[y][x]
    }
    pub fn set(&mut self, x: usize, y: usize, value: Cell) {
        self.values[y][x] = value;
    }
}

#[allow(dead_code)]
pub struct Game {
    pub running: bool,
    pub board: Board,
    pub difficulty: Difficulty,
    pub uncovered: Vec<Vec<bool>>,
    pub flags: Vec<(usize, usize)>
}
impl Game {
    pub fn new(width: usize, height: usize, difficulty: Difficulty) -> Game {
        let mut board = Board::new(width, height);
        let mut uncovered = Vec::new();
        for _y in 0..height {
            let mut row = Vec::new();
            for _x in 0..width {
                row.push(false);
            }
            uncovered.push(row);
        }
        let num_bombs = difficulty_lookup(width, height, difficulty.clone());
        for _ in 0..num_bombs {
            loop {
                let x = random(width);
                let y = random(height);
                if board.get(x, y) == &Cell::Empty {
                    board.set(x, y, Cell::Bomb);
                    break;
                }
            }
        }
        Game {
            running: true,
            board,
            difficulty,
            uncovered,
            flags: Vec::new()
        }
    }
    pub fn get_valid_neighbors(&self, x: usize, y: usize) -> Vec<Coord> {
        let x = x as isize;
        let y = y as isize;
        let mut cells_to_check = vec![
            Coord::new(x - 1, y - 1),
            Coord::new(x,     y - 1),
            Coord::new(x + 1, y - 1),
            Coord::new(x - 1, y    ),
            Coord::new(x,     y    ),
            Coord::new(x + 1, y    ),
            Coord::new(x - 1, y + 1),
            Coord::new(x,     y + 1),
            Coord::new(x + 1, y + 1)
        ];
        let mut valid_cells = Vec::new();
        for i in 0..cells_to_check.len() {
            if cells_to_check[i].x < 0 || cells_to_check[i].y < 0 {
                cells_to_check[i].valid = false;
            }
            if cells_to_check[i].x >= self.board.width as isize || cells_to_check[i].y >= self.board.height as isize {
                cells_to_check[i].valid = false;
            }
            if cells_to_check[i].valid {
                valid_cells.push(cells_to_check[i]);
            }
        }
        valid_cells
    }
    pub fn get_bomb_count(&self, x: usize, y: usize) -> usize {
        let mut bomb_count = 0;
        for cell in self.get_valid_neighbors(x, y) {
            if self.board.get(cell.x as usize, cell.y as usize) == &Cell::Bomb {
                bomb_count += 1;
            }
        }
        bomb_count
    }
    pub fn check_uncovered(&self, x: usize, y: usize) -> bool {
        self.uncovered[y][x]
    }
    pub fn uncover(&mut self, x: usize, y: usize) {
        if !self.check_uncovered(x, y) || !self.check_flagged(x, y) {
            self.uncovered[y][x] = true;
            if self.get_bomb_count(x, y) == 0 {
                for neighbor in self.get_valid_neighbors(x, y) {
                    if !self.check_uncovered(neighbor.x as usize, neighbor.y as usize) {
                        self.uncover(neighbor.x as usize, neighbor.y as usize);
                    }
                }
            }
            if self.board.get(x, y) == &Cell::Bomb {
                println!("Uncovered a bomb!");
                self.uncover_all();
                println!("{}", self.get_board());
                self.running = false;
            }
        }
    }
    pub fn uncover_all(&mut self) {
        let mut uncovered = Vec::new();
        for _y in 0..self.board.height {
            let mut row = Vec::new();
            for _x in 0..self.board.width {
                row.push(true);
            }
            uncovered.push(row);
        }
        self.uncovered = uncovered;
    }
    pub fn check_flagged(&self, x: usize, y: usize) -> bool {
        for flag in &self.flags {
            if *flag == (x, y) {
                return true;
            }
        }
        false
    }
    pub fn flag(&mut self, x: usize, y: usize) {
        if !self.check_flagged(x, y) {
            self.flags.push((x, y));
        } else {
            for i in 0..self.flags.len() {
                if self.flags[i] == (x, y) {
                    self.flags.remove(i);
                    break;
                }
            }
        }
    }
    pub fn get_board(&self) -> String {
        let mut out = String::new();
        out.push_str("  ");
        if self.board.height > 10 {
            out.push_str(" ");
        }
        for x in 0..self.board.width {
            out.push_str(&format!("{} ", x));
            if x < 10 {
                out.push_str(" ");
            }
        }
        out.push_str("\n");
        if self.board.height > 10 {
            out.push_str(" ");
        }
        out.push_str(" +");
        for _ in 0..self.board.width {
            out.push_str("---");
        }
        out.push_str("\n");
        for y in 0..self.board.height {
            let mut temp = String::new();
            temp.push_str(&format!("{}", y));
            if y < 10 && self.board.height > 10 {
                temp.push_str(" ");
            }
            temp.push_str("|");
            out.push_str(&temp);
            for x in 0..self.board.width {
                if self.check_uncovered(x, y) {
                    let bomb_count = self.get_bomb_count(x, y);
                    let count = &format!("{}", bomb_count);
                    out.push_str(match self.board.get(x, y) {
                        Cell::Empty => if bomb_count == 0 { "." } else { count },
                        Cell::Bomb => "@",
                    });
                } else {
                    if self.check_flagged(x, y) {
                        out.push_str("F");
                    } else {
                        out.push_str("_");
                    }
                }
                out.push_str("  ");
            }
            out.push_str("\n");
        }
        out.pop(); // Remove trailing newline.
        out
    }
}

pub fn difficulty_lookup(width: usize, height: usize, difficulty: Difficulty) -> usize {
    let area = (width * height) as f32;
    let num_bombs = match difficulty {
        Difficulty::Easy => 0.1 * area,
        Difficulty::Medium => 0.2 * area,
        Difficulty::Hard => 0.4 * area,
    };
    num_bombs as usize
}
pub fn random(max: usize) -> usize {
    let rand_amt: f64 = rand::random();
    ((max as f64) * rand_amt) as usize
}
pub fn read_num() -> isize {
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_bytes_read) => {
            input.pop(); // Remove trailing newline
            std::io::stdout().flush().unwrap();
            return input.parse::<isize>().expect("input was not isize");
        }
        Err(err) => {
            eprintln!("Error reading input: {}", err);
            return 0;
        }
    }
}
pub fn read() -> String {
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_bytes_read) => {
            input.pop(); // Remove trailing newline
            std::io::stdout().flush().unwrap();
            return input;
        }
        Err(err) => {
            eprintln!("Error reading input: {}", err);
            return String::new();
        }
    }
}
pub fn prompt(options: &str, min: isize, max: isize) -> usize {
    loop {
        print!("{}", options);
        let num = read_num();
        if num >= min && num <= max {
            return num as usize;
        } else {
            println!("Please choose between {} and {}", min, max);
        }
    }
}
pub fn prompt_coords(xmin: usize, xmax: usize, ymin: usize, ymax: usize) -> Coord {
    loop {
        print!("Which coordinates? ");
        let s = read();
        let mut current = 0;
        while char_at(&s, current) != ' ' && char_at(&s, current) != '\0' {
            current += 1;
        }
        if current >= str_len(&s) {
            println!("X and Y must be separated by a space.");
            continue;
        }
        let x = substring(&s, 0, current).parse::<usize>().expect("input was not usize");
        let y = substring(&s, current + 1, str_len(&s)).parse::<usize>().expect("input was not usize");
        if x < xmin || x > xmax || y < ymin || y > ymax {
            println!("X and Y must be within range.");
            continue;
        }
        return Coord::new(x as isize, y as isize);
    }
}
fn str_len(s: &String) -> usize {
    s.chars().count()
}
fn char_at(s: &String, pos: usize) -> char {
    if pos >= str_len(&s) {
        '\0'
    } else {
        s.chars().skip(pos).next().unwrap()
    }
}
fn substring(s: &String, start: usize, mut end: usize) -> String {
    if start > end || start >= str_len(&s) {
        "".to_owned()
    } else {
        if end > str_len(&s) {
            end = str_len(&s);
        }
        let text: String = s.chars().skip(start).take(end - start).collect();
        text
    }
}
