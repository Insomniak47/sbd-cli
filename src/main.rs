use bitflags::bitflags;
use std::env;
use std::io::Read;
use std::time::{Duration, Instant};
use std::{collections::HashMap, io, vec};
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref DASH_LOOKUP: HashMap<DirFlags, char> = vec![
        (DirFlags::TOP | DirFlags::RIGHT | DirFlags::BOTTOM, '├'),
        (DirFlags::TOP | DirFlags::LEFT | DirFlags::BOTTOM, '┤'),
        (DirFlags::LEFT | DirFlags::RIGHT | DirFlags::BOTTOM, '┬'),
        (DirFlags::LEFT | DirFlags::RIGHT | DirFlags::TOP, '┴'),
        (DirFlags::TOP | DirFlags::LEFT, '┘'),
        (DirFlags::TOP | DirFlags::RIGHT, '└'),
        (DirFlags::LEFT | DirFlags::BOTTOM, '┐'),
        (DirFlags::RIGHT | DirFlags::BOTTOM, '┌')
    ]
    .iter()
    .copied()
    .collect();
}

lazy_static! {
    static ref PIPE_LOOKUP: HashMap<DirFlags, char> = vec![
        (DirFlags::TOP | DirFlags::RIGHT | DirFlags::BOTTOM, '├'),
        (DirFlags::TOP | DirFlags::LEFT | DirFlags::BOTTOM, '┤'),
        (DirFlags::TOP | DirFlags::LEFT, '┘'),
        (DirFlags::TOP | DirFlags::RIGHT, '└'),
        (DirFlags::LEFT | DirFlags::BOTTOM, '┐'),
        (DirFlags::RIGHT | DirFlags::BOTTOM, '┌'),
    ]
    .iter()
    .copied()
    .collect();
}

fn main() {
    let total_time_anchor = Instant::now();
    let mut lines = read_all_lines_vec();
    let read_all_time = total_time_anchor.elapsed();

    let max_len = lines.iter().map(|x| x.len()).max().unwrap() + 1;
    let ingest_time = total_time_anchor.elapsed();
    let now = Instant::now();

    for line in lines.iter_mut() {
        line.resize(max_len, ' ')
    }

    let intake_time = now.elapsed();
    debug_assert!(lines.iter().all(|x| x.len() == max_len));
    let mut lookup_time = Duration::default();
    let mut jiggy_time = Duration::default();
    let exec_anchor = Instant::now();
    let mut new_lines = Vec::with_capacity(lines.len());

    for (index, line) in lines.iter().enumerate() {
        let mut new_line = String::new();
        for (i, c) in line.iter().enumerate() {
            let now = Instant::now();
            let new_val = if !c.is_whitespace() {
                let pt = Point { row: index, col: i };
                let loc = get_location(&lines, pt);
                let lookup_ts = now.elapsed();
                lookup_time += lookup_ts;
                let jiggy = loc.get_jiggy();
                jiggy_time += now.elapsed() - lookup_ts;
                // println!("{:?}", loc);
                // println!("{:?} - jiggy", jiggy);
                if let Some(v) = jiggy {
                    v
                } else {
                    '🔥'
                }
            } else {
                ' '
            };

            new_line.push(new_val);
        }
        new_lines.push(new_line);
    }
    let exec_time = exec_anchor.elapsed();
    let write_anchor = Instant::now();
    for line in new_lines.iter() {
        println!("│{}│", line);
    }

    if let Ok(x) = env::var("SBD_DIAG") {
        if x != "true" {
            return;
        }
        println!("Write time: {}ms", write_anchor.elapsed().as_millis());
        println!("Ingest time {}ms", ingest_time.as_millis());
        println!("Total time to prep inputs: {}ms", intake_time.as_millis());
        println!("Time in exec loop {}ms", exec_time.as_millis());
        println!("Lookup Time = {}ms", lookup_time.as_millis());
        println!("Jiggy Time = {}ms", jiggy_time.as_millis());
        println!("Total time = {}ms", total_time_anchor.elapsed().as_millis());
        println!("Read time: {}ms", read_all_time.as_millis());
    }
}

fn get_location(vec: &Vec<Vec<char>>, anchor: Point) -> Location {
    // println!("Current point: {:?} - CanSample? top {} btm {}", anchor, can_sample_top, can_sample_bottom);
    // println!("Current character: {}", vec[anchor.row][anchor.col]);
    let family = kidnap(vec, &anchor);

    return Location {
        value: vec[anchor.row][anchor.col],
        family,
    };
}

fn kidnap(vec: &Vec<Vec<char>>, anchor: &Point) -> Vec<char> {
    let mut family = Vec::new();

    if anchor.row != 0 {
        family.push(vec[anchor.row - 1][anchor.col]); //top
    } else {
        family.push(' ')
    }

    family.push(vec[anchor.row][anchor.col - 1]); //left
    family.push(vec[anchor.row][anchor.col + 1]); //right

    if anchor.row != vec.len() - 1 {
        family.push(vec[anchor.row + 1][anchor.col]); //btm
    } else {
        family.push(' ')
    }

    return family;
}

fn read_all_lines_vec() -> Vec<Vec<char>> {
    let mut buf = String::new();
    buf.reserve(4096);
    let stdin = io::stdin();

    _ = stdin.lock().read_to_string(&mut buf);

    buf.split('\n')
        .map(|s| format!(" {}", s.trim_end()).chars().collect())
        .collect()
}

#[derive(Debug)]
struct Location {
    // position: Point,
    value: char,
    family: Vec<char>,
}

#[derive(Debug)]
struct Point {
    row: usize,
    col: usize,
}

trait IntoJiggy {
    fn into_jiggy(&self) -> Option<char>;
}

impl Location {
    fn get_jiggy(&self) -> Option<char> {
        match self.value {
            '-' => translate_dashy_lookup(self),
            '|' => translate_pipey_lookup(self),
            '>' | '<' | '^' => translate_pointy(self),
            'v' => translate_pointy(self).or(Some('v')),
            x => Some(x),
        }
    }
}

fn connecty_bits_flags(loc: &Location) -> DirFlags {
    loc.family
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let dirn = match i {
                //CN: Directions to center
                0 => DirFlags::BOTTOM,
                1 => DirFlags::RIGHT,
                2 => DirFlags::LEFT,
                3 => DirFlags::TOP,
                _ => unreachable!(),
            };

            //  if loc.value == '|'{
            //     println!("Target: {} to the {:?} of {} -- Can connect from: {}, should bridge? {}", v, dirn.opposite(), loc.value,v.can_connect_from(dirn),v.should_bridge_from(dirn) || loc.value.should_bridge_from(dirn.opposite()));
            //  }
            let as_direction = dirn.into();
            if v.can_connect_from(as_direction)
                && (v.should_bridge_from(as_direction)
                    || loc.value.should_bridge_from(as_direction.opposite()))
            {
                Some(as_direction.opposite().into())
            } else {
                None
            }
        })
        .filter_map(|x| x)
        .fold(DirFlags::default(), |x, v| x | v)
}

bitflags! {
    #[derive(Default)]
    struct DirFlags : u8 {
        const TOP    = 0b0001;
        const LEFT   = 0b0010;
        const RIGHT  = 0b0100;
        const BOTTOM = 0b1000;
    }
}

fn translate_dashy_lookup(loc: &Location) -> Option<char> {
    match DASH_LOOKUP.get(&connecty_bits_flags(loc)) {
        // Some(x) => x.into::<char>(),
        // None => '🔥'
        Some(x) => Some(*x),
        _ => Some('─'),
    }
}

fn translate_pipey_lookup(loc: &Location) -> Option<char> {
    match PIPE_LOOKUP.get(&connecty_bits_flags(loc)) {
        Some(x) => Some(*x),
        _ => Some('│'),
    }
}

fn translate_pointy(loc: &Location) -> Option<char> {
    match (&loc.family[..], loc.value) {
        ([_, '-', ..], '>') => Some('►'),
        ([.., '|'], '^') => Some('▲'),
        ([.., '-', _], '<') => Some('◄'),
        (['|', ..], 'v') => Some('▼'), //Robert
        _ => None,
    }

    //Some('►')
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord)]
enum Direction {
    Top,
    Left,
    Right,
    Bottom,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Top => Self::Bottom,
            Direction::Left => Self::Right,
            Direction::Bottom => Self::Top,
            Direction::Right => Self::Left,
        }
    }
}

impl Into<DirFlags> for Direction {
    fn into(self) -> DirFlags {
        match self {
            Direction::Top => DirFlags::TOP,
            Direction::Left => DirFlags::LEFT,
            Direction::Right => DirFlags::RIGHT,
            Direction::Bottom => DirFlags::BOTTOM,
        }
    }
}

impl Into<Direction> for DirFlags {
    fn into(self) -> Direction {
        match self {
            DirFlags::TOP => Direction::Top,
            DirFlags::LEFT => Direction::Left,
            DirFlags::RIGHT => Direction::Right,
            DirFlags::BOTTOM => Direction::Bottom,
            _ => unreachable!(),
        }
    }
}

trait Connectable {
    fn is_connectable(&self) -> bool;
    fn can_connect_from(&self, dirn: Direction) -> bool;
    fn should_bridge_from(&self, dirn: Direction) -> bool;
}

trait ConnectableFlags {
    fn is_connectable(&self) -> bool;
    fn can_connect_from(&self, dirn: DirFlags) -> bool;
    fn should_bridge_from(&self, dirn: DirFlags) -> bool;
}

//The concept of creating a branch is different than terminating a branch.
//     |
//  ----Thing
// makes sense that there would be a T-junc before Thing but
//   BBBBBBBBBBBBBBB    BBBBBBBBBBBBBBB
//   --------------- != ┼┼┼┼┼┼┼┼┼┼┼┼┼┼┼
//   AAAAAAAAAAAAAAA    AAAAAAAAAAAAAAA
//
//
// should_bridge?
// letters -> Never
// pipes => top + bottom
// dashes => left and right
// pointies => never

impl Connectable for char {
    fn is_connectable(&self) -> bool {
        return !self.is_whitespace();
    }

    fn can_connect_from(&self, dirn: Direction) -> bool {
        match self {
            '-' => true,
            '|' => true,
            '>' => dirn == Direction::Left,
            '<' => dirn == Direction::Right,
            '^' => dirn == Direction::Bottom,
            'v' => dirn == Direction::Top,
            x if x.is_whitespace() => false,
            _ => true,
        }
    }

    fn should_bridge_from(&self, dirn: Direction) -> bool {
        match self {
            '-' => dirn == Direction::Left || dirn == Direction::Right,
            '|' => dirn == Direction::Top || dirn == Direction::Bottom,
            _ => false,
        }
    }
}
