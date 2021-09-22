use std::ops::{Add, AddAssign, Sub};

use super::tile::{Tile, TileComponent, TileState};
use rand::prelude::IteratorRandom;
use rand::thread_rng;
use rand::{rngs::ThreadRng, Rng};
use yew::services::console;
use yew::services::keyboard::KeyListenerHandle;
use yew::utils::document;
use yew::{prelude::*, services::KeyboardService};

#[derive(Debug, Copy, Clone)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Vec2 {
        Vec2 { x, y }
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}
impl From<Vec2> for Direction {
    fn from(vec: Vec2) -> Self {
        if vec.x.abs() > vec.y.abs() {
            if vec.x > 0 {
                Direction::Right
            } else {
                Direction::Left
            }
        } else {
            if vec.y > 0 {
                Direction::Down
            } else {
                Direction::Up
            }
        }
    }
}
impl Direction {
    pub fn as_pair(self) -> (i32, i32) {
        match self {
            Direction::Down => (1, 0),
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Left => (0, -1),
        }
    }
    fn build_traversal(self) -> Vec<Position> {
        let i_traversal: Vec<usize> = match self {
            Direction::Down => (0..4).rev().collect(),
            _ => (0..4).collect(),
        };
        let j_traversal: Vec<usize> = match self {
            Direction::Right => (0..4).rev().collect(),
            _ => (0..4).collect(),
        };
        i_traversal
            .iter()
            .flat_map(|i| j_traversal.iter().map(move |j| Position::new(*i, *j)))
            .collect()
    }
}

pub enum Msg {
    KeyboardEvent(KeyboardEvent),
    TouchStart(TouchEvent),
    TouchEnd(TouchEvent),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Position {
    i: usize,
    j: usize,
}
impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Position { i: x, j: y }
    }
    pub fn from_index(index: usize) -> Self {
        Position {
            i: index / 4,
            j: index % 4,
        }
    }
    pub fn index(self) -> usize {
        self.i * 4 + self.j
    }
    pub fn is_out_of_bounds(self) -> bool {
        self.i >= 4 || self.j >= 4
    }
}
impl Add<Direction> for Position {
    type Output = Position;
    fn add(self, direction: Direction) -> Self::Output {
        let (i, j) = direction.as_pair();
        Position {
            i: (self.i as i32 + i) as usize,
            j: (self.j as i32 + j) as usize,
        }
    }
}
impl AddAssign<Direction> for Position {
    fn add_assign(&mut self, rhs: Direction) {
        *self = *self + rhs
    }
}

type Cell = Option<Tile>;
#[derive(Debug, Copy, Clone)]
pub struct Grid {
    cells: [Cell; 16],
    rng: ThreadRng,
    enable_new_tiles: bool,
}

impl Default for Grid {
    fn default() -> Self {
        let mut grid = Grid::new([None; 16]);
        for _ in 0..2 {
            grid.add_random_tile();
        }
        grid
    }
}

impl Grid {
    pub fn new(cells: [Cell; 16]) -> Grid {
        Grid {
            cells,
            rng: thread_rng(),
            enable_new_tiles: true,
        }
    }
    fn get(&self, pos: Position) -> Option<Tile> {
        self.cells.get(pos.index()).and_then(|tile| *tile)
    }

    pub fn disable_new_tiles(&mut self) {
        self.enable_new_tiles = false;
    }
    fn prepare_for_move(&mut self) {
        for i in 0..16 {
            self.cells
                .get_mut(i)
                .and_then(|cell| cell.as_mut())
                .map(|tile| {
                    console::ConsoleService::log(&format!("traversal {:?}", tile));
                    tile.state = TileState::Static;
                    tile.previous_position = Some(Position::from_index(i))
                });
        }
    }
    pub fn move_in(&mut self, direction: Direction) {
        self.prepare_for_move();
        let traversal = direction.build_traversal();
        let mut moved = false;
        for start_position in traversal {
            moved |= self.traverse_from(start_position, direction);
        }
        if moved {
            self.add_random_tile();
        }
    }
    fn traverse_from(&mut self, start_position: Position, in_direction: Direction) -> bool {
        let mut start_tile = match self.get(start_position) {
            Some(tile) => tile,
            None => return false,
        };
        let mut new_position = start_position;
        loop {
            let next_position = new_position + in_direction;
            console::ConsoleService::log(&format!("traverse_from {:?}", next_position));
            if next_position.is_out_of_bounds() {
                break;
            }
            if let Some(tile) = self.get(next_position) {
                if tile == start_tile && tile.state != TileState::Merged {
                    start_tile.number *= 2;
                    start_tile.state = TileState::Merged;
                    new_position = next_position;
                }
                break;
            }
            new_position = next_position;
        }
        if start_position == new_position {
            return false;
        }
        self.cells[start_position.index()] = None;
        self.cells[new_position.index()] = Some(start_tile);
        return true;
    }
    fn add_random_tile(&mut self) {
        if (!self.enable_new_tiles) {
            return;
        }
        let rng = &mut self.rng;

        let empty_cells = self.cells.iter_mut().filter(|x| x.is_none());

        if let Some(empty) = empty_cells.choose(rng) {
            let number = match self.rng.gen::<f64>() {
                x if x > 0.9 => 4,
                _ => 2,
            };
            *empty = Some(Tile::new(number))
        }
    }
    fn is_end(&self) -> bool {
        !self.enable_new_tiles
    }
    fn tiles(&self) -> impl Iterator<Item = (Position, Tile)> + '_ {
        self.cells
            .iter()
            .enumerate()
            .filter_map(|(i, cell)| match cell {
                None => None,
                Some(tile) => Some((Position::from_index(i), *tile)),
            })
            .flat_map(|(position, tile)| match tile.state {
                TileState::Merged => vec![
                    (position, tile),
                    (
                        position,
                        Tile {
                            state: TileState::Static,
                            previous_position: tile.previous_position,
                            number: tile.number / 2,
                        },
                    ),
                ],
                _ => vec![(position, tile)],
            })
    }
}

impl PartialEq for Grid {
    fn eq(&self, other: &Self) -> bool {
        self.cells == other.cells
    }
}
impl ToString for Grid {
    fn to_string(&self) -> String {
        format!("{:?}", self.cells)
    }
}

pub struct GridComponent {
    link: ComponentLink<Self>,
    grid: Grid,
    current_render: i32,
    #[allow(dead_code)]
    keyboard_event_listener: KeyListenerHandle,
    touch_start: Option<TouchEvent>,
}
impl GridComponent {
    fn move_in(&mut self, direction: Direction) {
        self.grid.move_in(direction);
    }
}

impl Component for GridComponent {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let keyboard_event_listener = KeyboardService::register_key_down(
            &document(),
            (&link).callback(|e: KeyboardEvent| Msg::KeyboardEvent(e)),
        );
        Self {
            link,
            grid: Grid::default(),
            keyboard_event_listener,
            current_render: 0,
            touch_start: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::KeyboardEvent(e) => match e.key_code() {
                37 => self.move_in(Direction::Left),
                38 => self.move_in(Direction::Up),
                39 => self.move_in(Direction::Right),
                40 => self.move_in(Direction::Down),
                _ => return false,
            },
            Msg::TouchStart(e) => {
                e.prevent_default();

                self.touch_start = Some(e);

                return false;
            }
            Msg::TouchEnd(touches_end) => {
                let touch_start = self
                    .touch_start
                    .as_ref()
                    .and_then(|e| e.changed_touches().item(0))
                    .map(|touch| Vec2::new(touch.client_x(), touch.client_y()));

                let touch_end = touches_end
                    .changed_touches()
                    .item(0)
                    .map(|touch| Vec2::new(touch.client_x(), touch.client_y()));

                match (touch_start, touch_end) {
                    (Some(start), Some(end)) => self.move_in((end - start).into()),
                    _ => return false,
                };
            }
        }
        if self.grid.is_end() {
            console::ConsoleService::log(&"game is end");
            // return false;
        }
        self.current_render += 1;
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        console::ConsoleService::log(&self.grid.to_string());
        html! {
            <div class="grid-wrapper" ontouchstart=self.link.callback(Msg::TouchStart) ontouchend=self.link.callback(Msg::TouchEnd)>
                <div class="grid" key=self.current_render>
                { for (0..16).map(|_| { html! { <div class="cell"></div> }}) }
                { for self.grid.tiles().map(|(position, tile)| html! { <TileComponent position=position tile=tile />} ) }
                </div>
            </div>
        }
    }
}
