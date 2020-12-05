use std::vec;

#[derive(PartialEq)]
enum Tile {
    Open,
    Tree,
}

impl Tile {
    fn parse(input: char) -> Self {
        match input {
            '.' => Tile::Open,
            '#' => Tile::Tree,
            _ => panic!("Unknown terrain"),
        }
    }

    fn is_tree(&self) -> bool {
        *self == Tile::Tree
    }
}

struct Map {
    // double vector of Tile for 2D pattern
}

impl Map {
    // logic for infinite wrapping on x-axis
    // counting type of tile given a 2D slope
}

fn main() {
    println!("Hello day3!");
}

pub fn get_input() -> Vec<&'static str> {
    vec![
        "........#..#.##.#..............",
        "...#...............#.#.........",
        "...#..#...#..##....#...........",
        "...#.............#....#.....#..",
        "..#......#..#...#.......#......",
        "..............##...............",
        "#.......#.........#......#....#",
        ".#.....###.....#...#.#.#...#...",
        "#.....................#....#.#.",
        ".......#...................#...",
        "...#.#...................#....#",
        "....#....#.......#...#.........",
        "..##.#............#..#.........",
        ".....##.#..............##..###.",
        "...........#....#....#.........",
        "#.....#...#...#.#.#.#.##.#...#.",
        ".#...............#....##.......",
        ".....#..#......#....#.......##.",
        ".....#........#.......#........",
        "...#...##...#..##...#.....##...",
        ".....#.........#.###...##...#..",
        ".#.##...#........#.#.#.#....#..",
        "....#......##.#...#.....#....#.",
        ".......###..........#..#..#....",
        "......#...#.##.................",
        "....#...#...#.........#......#.",
        ".....#...........#...###....#..",
        ".....#...#.#.#....##.#......#.#",
        "......#...#.....#..#..#........",
        "#......#..#...##........###....",
        "##.....#....##..#.#.###.#...#..",
        "........#....#.......#.....#..#",
        "#.#.#.##.#.#...................",
        "..#...##....#......#.....##....",
        ".......#.##..#........##..#....",
        ".#.#....##......#.#..........#.",
        "#..............#............#..",
        ".#.#.#.#.#.####.#.#...##.......",
        ".#..#.....##.#.......#.##...#..",
        "..#.#........#.............#.#.",
        "..#.#..........#..#........#...",
        "..#..#...#.......##...#.#....##",
        "...#.....#.#.#.....#....#....#.",
        ".#...#......#.....#..##........",
        "...#.......##.#.#.....#......#.",
        "...........#.....#.#.......#...",
        "#...........#...#..#.#........#",
        "....#......#..##........#..###.",
        ".#..#........................#.",
        "#.......#......#...#...#..#....",
        "....#.#...#..#.#....#....##.#..",
        ".....#......#..#..........##.#.",
        ".#.....#...........#.........#.",
        "...###.#...#.......#.#.........",
        ".......#....#..........#..#...#",
        "......##..#.......#...##.......",
        "..#..........#.......#.........",
        "..........#..#..#..#..#........",
        ".#.................####...#.#.#",
        "..##.....#............#........",
        "....#.....###...#......#....#.#",
        "...##.#...........#.##......#..",
        "#..##..#..#....#...#..#........",
        "......#....#........#.......#..",
        "......#.....#......###.........",
        ".#.....#.#......#.......#......",
        "..#.........#..#..#........##.#",
        "..#.#....#.....#....##....#.#..",
        "...#.............##............",
        "........#..#..#......#...#.....",
        ".....#.#...#...##.....#.....#..",
        ".#..#.#..........##...##.....#.",
        "......##.#..........#...#.....#",
        "#.#.##......#....#..........#..",
        "................#.......#.##...",
        "#.......#.....#.......#....#...",
        "#..#.....#.##..##...........#..",
        ".....#....#.#.##..........#..##",
        "#.......#.....#.##...........#.",
        "........#.##........###..#.#...",
        "........#..................#...",
        "#.........................#...#",
        "....#.........#...#.#..#.....#.",
        ".#............#....#...........",
        "..#.#...#..##...#.#.......#....",
        ".#.#....#...........#.........#",
        "...#.#..........#.....#...#....",
        "......#....#.#...............##",
        "....##......###...##.##.....##.",
        "............#.#....#.#.....#..#",
        ".....#..#.....#.#...###....#...",
        ".......##....##..#...##..#...##",
        ".....#.......##..#...#...#....#",
        "#.........##....#........###.#.",
        "...#..##...#...#.........#.#.#.",
        "....#.#.....#.....#............",
        "#........#....#..#........#....",
        ".......#....#...#..............",
        "#...#.........##.....###.#.....",
        ".#....##..#...#..##.........#..",
        "....#.....#......##..#..#....#.",
        "#.#..#.........#........#......",
        "..#.......#.........#.....###..",
        "..#..........#...........#....#",
        "..#...............#......#..#..",
        "....#..#...#....###.....#..#..#",
        "#...#...#..#...........#....#..",
        ".#....#.#..#....#.#...........#",
        ".....#.....#..#....#..#....#...",
        "#.#..#...........#.#...........",
        "..................#.#.......#..",
        "...#.........#.....#..##....#..",
        ".........#.#...#.........##....",
        "...#..#....#.....#...#..#......",
        ".#.##.....#....#....#......##..",
        "##..#.........#.#....#...#.....",
        "#......#.#...#....#.#..#.......",
        ".......#.....#.....###....#.#..",
        ".#....##.#.....#...#.......#...",
        ".#.......#..#...#......#..#..##",
        "...............#...#...........",
        "#..............#....#.#.#....#.",
        "...........#..#.......#.##..#..",
        "..#......#.#....#...#.#.....#..",
        "#..............................",
        "#..#....#..........#...#.......",
        "......#.............#####......",
        ".#...###......#.#.#.##..#......",
        "............#.##.....#.........",
        ".........#....##....#..........",
        "###....#......#.......#........",
        ".#.......##..........#..#....#.",
        "#..#.....................#....#",
        "........#...........#..........",
        "..#..........#...#..#.........#",
        "..#..#......##................#",
        ".....##..#...#..#..............",
        ".......#...##..#...............",
        ".......##..#.####....#....#.#..",
        "#.#..#..........#........##....",
        "....##....#.#..#....#.#...#....",
        "......#.......#...#.....#...#..",
        "..#..#...#.....#.......###.....",
        "...#.......#.#.#.......#.##....",
        "...............#..#.#........#.",
        ".#....###.#......#.............",
        ".#..#...#....#.#..#.....#......",
        ".......#.##....#.#.##.##...#.#.",
        "..#...#....#.#..##.#.....#...##",
        "..#...#......#...#......#...#..",
        "....#..#...#.#..#......#.......",
        "#..#...............#......#.##.",
        ".#....#...#..........#.#.....#.",
        ".#..#.#.#................#..#..",
        ".#....#.#...#..##.###..#...###.",
        "#.............#.....#.........#",
        "...#.........#...#.......#..#..",
        "......#..#.........#..........#",
        "........##................#..#.",
        "......#...#.#.....#......##....",
        "...............#...#....#......",
        "...#.#..#..#.....##.###..##..#.",
        ".#....##......#...#..##..#.....",
        ".....#.........##.##....#...#..",
        ".....#.#..................####.",
        "#.....#...#.............##....#",
        "#.#..........#...#..#..#.......",
        "#..#.#.........#...............",
        "....#...#.........#...##.......",
        "...........#.....#..##..#......",
        "#.....#.......#.#........#.....",
        "..##..#.....#...##......#......",
        "....#....#.....................",
        "............#......#.........##",
        ".....##.............#.....##..#",
        ".......#.............#..#.#.##.",
        ".###...#......#..#........##.#.",
        "..#.#...#.#....#.....#..#......",
        "..#.#..#.##........#...#.......",
        "........#.#...............#..#.",
        "........##.......#...#.......#.",
        "...#........##.#..........#.#.#",
        "..#..###.#.#.......#.#......#..",
        "....#..........#...#..#........",
        "...#..#...#...#.#....#...#..#..",
        "...#...#........#......##...#.#",
        "#...........#..........#..#.##.",
        "...#..##..................#.#..",
        "...##.#...#....#.#...#.####....",
        ".....#...#.#.#..#..............",
        ".....#..#.#.#..#...............",
        "..#..#..##...#.#..#.....##....#",
        ".......#.#..#.....#....#.......",
        "...#..#....#.........#...#.....",
        "..............#.#...#...##.....",
        "...................#...........",
        ".#......#.#...................#",
        ".##.....#........#.........#..#",
        ".##..##...#...................#",
        "...#....#.#..#.#.#..#.....##...",
        ".......#..#....#......####.#...",
        ".##..#..##....#.......#........",
        ".#...#...........##............",
        ".....#.....#........#..........",
        "....##..#....#.....#...........",
        ".#...#....................#....",
        "....#.........#.......##.....#.",
        ".#....#..#.....#.##....#.......",
        "....#..#.........#.#....#.#....",
        ".......#.........##....#.......",
        "..#......#....#....#...#.......",
        "........#..#.......#.##......#.",
        "..#.....#......#...#..#.......#",
        "#..#.....##...#...#............",
        ".......##.......#........#...#.",
        "..#......................#...#.",
        "....##.#.............#......#..",
        "#.#............................",
        "...##.#.....#.#............#.##",
        "......#...#..#.........##......",
        ".#.......#.....##.......#.#....",
        "...........#.#.........#..##...",
        "...#..........#.##....#........",
        "........#..#..#...#....#....#..",
        "........##....#.#....#........#",
        "..#........##....###....#......",
        "#................###...#...#...",
        "................#.#..###......#",
        "..#.....##.#................#..",
        ".....#...............#..#......",
        "..#.......####.....#..#.#....##",
        "..#.....#..#....#..............",
        "#.#...........#.#.....#..##....",
        "#.#..........#.......#...#.###.",
        "........#....#...#..#.#........",
        ".#.....#......#..#..#..###..#..",
        ".#.........#.##.#.#......##....",
        "..#.........#...##..#........#.",
        ".#...................#.........",
        "...#.#........#................",
        "............#.....#..##........",
        "..#.....#.#......#.......#...#.",
        "........#....##..##...#.....##.",
        ".#........#.#....#.#....#.#..#.",
        "#.#.......#....................",
        ".#..#...##.........#..#........",
        ".........#...............#.....",
        "...#...#.....#......#.......#..",
        "###......................#.#..#",
        "...#.....####........#..#.....#",
        "#.#...#.#...................##.",
        ".........#.....................",
        "#..........##..#.....#....#....",
        ".......#...#.#.##.#..##........",
        "..........#..#.#..#.#.......#.#",
        ".....................#.#...#...",
        "...........#.#........#.#.#....",
        ".......#......#........#...#.#.",
        ".........#....................#",
        ".##.##....#...#.#.#.#..........",
        "#....##..#.##....#....#.......#",
        ".##.#...#...............#....#.",
        ".......#...#.###....#..........",
        ".....#....#...#..#.............",
        "#.........#.##....#.#.#........",
        "..#...#.............##..#..#...",
        "#..##.......#..........#...#.#.",
        ".#..#.....#...........#......#.",
        "......#......#..............##.",
        ".#...#..#...#..####.....#.....#",
        "....##.......#..........##.....",
        ".#.....#.......#.....#.#...#...",
        "..#..#..#.#...#......#.........",
        "......#.#....#........#.......#",
        "........#.......#..............",
        "..#...#.#....#........#.......#",
        "............#....#...##.#......",
        ".........#.............#..#....",
        "#.............#.#..##.......#..",
        "#....#...........###....#......",
        "...#.....................#.....",
        "....#.#..........#...#.......#.",
        "......#..#.......#...#...#....#",
        ".#.#..#.....##.#........#......",
        "...........#...#.#.............",
        "...###............#...#..#.....",
        "..#.#.......#...#.#..#.........",
        ".#......##...........#.....#.##",
        ".....##.....#....##...##.#.#...",
        "..........#.#.#......#........#",
        "..#.#........#....##....#.#....",
        ".#....#...##...........#....#..",
        "##......#...#.......#..........",
        ".##...###..#...#......#..##.#.#",
        "...........##.#..##...#.......#",
        "..#..............##............",
        "........#..#........#...#..#.#.",
        "..#.............#......#...##..",
        "#...##....#...#....#....#.#....",
        ".#.#......#..##............#.#.",
        ".....###.#....##....#....#.....",
        "#.#.#..........#...#...#.#.#...",
        ".....#.#...........####........",
        ".....#....##...#.##..#......#..",
        "#....#.......#.##.......#..#...",
        ".....#.....#........#..........",
        ".......#.......#...#.##......#.",
        "...#.........##...#.#.#......##",
        "#........#........#...#..#.....",
        ".#......#.#......#.#...#....#..",
        "#..#....##.....##..............",
        "...#.##............#..........#",
        ".....#.#....#..#.#............#",
        "..#......#...###.##.......###..",
        "........#....#.#.#.#...........",
        "............#..#........#.....#",
        "....#...............#..........",
        "......#....#....###..#.......##",
        "#...#...##....#.........#...#..",
        "...........#.#.............#...",
        "...#..#.....#..##.#....#......#",
        "..#...#..#...#......#..........",
        "....#..#....#.......#........#.",
    ]
}
