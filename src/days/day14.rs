use crate::color::get_rgb_pixel;
use crate::visualizer::CharVisualizationOption;
use crate::{solution::Solution, types::Coords, visualizer::Visualizer};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Rock,
    Air,
    Sand,
}

enum ArrivedFrom {
    Above,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Cave {
    map: Vec<Vec<Cell>>,
    top_left: Coords<usize>,
    width: usize,
    height: usize,
}

impl Cave {
    fn new(top_left: Coords<usize>) -> Cave {
        Cave {
            map: Vec::new(),
            top_left,
            width: 0,
            height: 0,
        }
    }

    fn coords2indices(&self, c: &Coords<usize>) -> (usize, usize) {
        let c_rel = *c - self.top_left;
        (c_rel.y, c_rel.x)
    }

    fn set(&mut self, c: &Coords<usize>, cell: Cell) {
        let (i, j) = self.coords2indices(c);
        if j >= self.width {
            for row in self.map.iter_mut() {
                row.extend(vec![Cell::Air; j - self.width + 1]);
            }
            self.width = j + 1;
        }
        if i >= self.height {
            (0..(i - self.height + 1)).for_each(|_| {
                self.map.push(vec![Cell::Air; self.width]);
            });
            self.height = i + 1;
        }
        self.map[i][j] = cell;
    }

    fn at(&self, c: &Coords<usize>) -> Cell {
        let c_rel = *c - self.top_left;
        let i = c_rel.y;
        let j = c_rel.x;
        self.map[i][j]
    }

    fn setup_visualizer(v: &mut dyn Visualizer) {
        v.add_char_visualization_option(CharVisualizationOption {
            char: 'o',
            is_bold: false,
            color: get_rgb_pixel(46, 100, 55),
        });
        v.add_char_visualization_option(CharVisualizationOption {
            char: '#',
            is_bold: true,
            color: get_rgb_pixel(0, 100, 100),
        });
    }

    fn visualize(
        &self,
        v: &mut dyn Visualizer,
        arrived_from_map: Option<&Vec<Vec<Option<ArrivedFrom>>>>,
    ) {
        if !v.is_enabled() {
            return;
        }
        for (i, row) in self.map.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                match cell {
                    Cell::Rock => v.write_char('#'),
                    Cell::Sand => v.write_char('o'),
                    Cell::Air => {
                        if let Some(arrived_from_map_) = arrived_from_map {
                            match arrived_from_map_[i][j] {
                                None => v.write_char(' '),
                                Some(ArrivedFrom::Above) => v.write_char('↓'),
                                Some(ArrivedFrom::Right) => v.write_char('↙'),
                                Some(ArrivedFrom::Left) => v.write_char('↘'),
                            }
                        } else {
                            v.write_char(' ');
                        }
                    }
                };
            }
            v.write_newline();
        }
        v.end_frame();
    }
}

pub struct RegolithReservoir;

impl Solution for RegolithReservoir {
    type InputT = Cave;
    type OutputT = u32;

    fn parse_input(&self, input_raw: String) -> Self::InputT {
        let mut rock_line_defs: Vec<Vec<Coords<usize>>> = Vec::new();
        for line in input_raw.lines() {
            let mut rock_line_def: Vec<Coords<usize>> = Vec::new();
            for coord in line.split(" -> ") {
                rock_line_def.push(coord.parse().unwrap());
            }
            rock_line_defs.push(rock_line_def);
        }
        let top_left_x = rock_line_defs
            .iter()
            .flat_map(|r| r.iter().map(|c| c.x))
            .min()
            .unwrap()
            .clone()
            - 1; // -1 lets sand fall to the abyss
        let top_left_y = 0;
        let mut cave = Cave::new(Coords {
            x: top_left_x,
            y: top_left_y,
        });

        for line_def in rock_line_defs {
            line_def
                .windows(2)
                .map(|window| window[0].line_between(&window[1]))
                .flatten()
                .for_each(|c| cave.set(&c, Cell::Rock));
        }

        cave
    }

    fn solve_pt1(
        &self,
        mut input: Self::InputT,
        v: &mut dyn crate::visualizer::Visualizer,
    ) -> Self::OutputT {
        Cave::setup_visualizer(v);

        let down = Coords { x: 0, y: 1 };
        let right = Coords { x: 1, y: 0 };

        let mut sand_particles: u32 = 0;
        loop {
            sand_particles += 1;
            let mut current_sand_coords = Coords { x: 500, y: 0 };
            let mut falling_frames = 0;
            loop {
                if current_sand_coords.y == input.top_left.y + input.height - 1 {
                    return sand_particles - 1;
                }
                falling_frames += 1;
                if falling_frames % 5 == 0 {
                    falling_frames = 0;
                    input.visualize(v, None);
                }
                input.set(&current_sand_coords, Cell::Sand);
                let mut sand_moved = false;
                for next_sand_coords in [
                    current_sand_coords + down,
                    current_sand_coords + down - right,
                    current_sand_coords + down + right,
                ] {
                    if input.at(&next_sand_coords) == Cell::Air {
                        input.set(&current_sand_coords, Cell::Air);
                        current_sand_coords = next_sand_coords;
                        input.set(&current_sand_coords, Cell::Sand);
                        sand_moved = true;
                        break;
                    }
                }
                if !sand_moved {
                    input.visualize(v, None);
                    break;
                }
            }
        }
    }

    fn solve_pt2(
        &self,
        input: Self::InputT,
        v: &mut dyn crate::visualizer::Visualizer,
    ) -> Self::OutputT {
        let new_top_left = Coords {
            x: 500 - input.height - 2,
            y: 0,
        };
        let mut modified_input = Cave::new(new_top_left);
        for (i, row) in input.map.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                modified_input.set(
                    &Coords {
                        x: input.top_left.x + j,
                        y: input.top_left.y + i,
                    },
                    *cell,
                );
            }
        }

        for x in (500 - input.height - 2)..(500 + input.height + 2) {
            modified_input.set(
                &Coords {
                    x,
                    y: input.height + 1,
                },
                Cell::Rock,
            );
        }

        let mut last_arrived_from: Vec<Vec<Option<ArrivedFrom>>> = Vec::new();
        for _i in 0..modified_input.height {
            let mut row: Vec<Option<ArrivedFrom>> = Vec::new();
            for _j in 0..modified_input.width {
                row.push(None);
            }
            last_arrived_from.push(row);
        }

        // modified_input.visualize(v);

        // actual solution here

        Cave::setup_visualizer(v);

        let down = Coords { x: 0, y: 1 };
        let right = Coords { x: 1, y: 0 };

        let mut sand_particles: u32 = 0;
        let sand_source_coords = Coords { x: 500, y: 0 };
        let mut current_sand_coords = sand_source_coords;
        loop {
            let (i, j) = modified_input.coords2indices(&current_sand_coords);
            current_sand_coords = match last_arrived_from[i][j] {
                None => sand_source_coords,
                Some(ArrivedFrom::Above) => current_sand_coords - down,
                Some(ArrivedFrom::Left) => current_sand_coords - down - right,
                Some(ArrivedFrom::Right) => current_sand_coords - down + right,
            };
            sand_particles += 1;
            loop {
                modified_input.set(&current_sand_coords, Cell::Sand);
                let mut sand_moved = false;
                for (next_sand_coords, arrived_from) in [
                    (current_sand_coords + down, ArrivedFrom::Above),
                    (current_sand_coords + down - right, ArrivedFrom::Right),
                    (current_sand_coords + down + right, ArrivedFrom::Left),
                ] {
                    if modified_input.at(&next_sand_coords) == Cell::Air {
                        modified_input.set(&current_sand_coords, Cell::Air);
                        current_sand_coords = next_sand_coords;
                        modified_input.set(&current_sand_coords, Cell::Sand);
                        let (i, j) = modified_input.coords2indices(&current_sand_coords);
                        last_arrived_from[i][j] = Some(arrived_from);
                        sand_moved = true;
                        break;
                    }
                }
                if !sand_moved {
                    modified_input.visualize(v, Some(&last_arrived_from));
                    if current_sand_coords.x == 500 && current_sand_coords.y == 0 {
                        return sand_particles;
                    } else {
                        break;
                    }
                }
            }
        }
    }
}
