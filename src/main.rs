use std::{io::{self}};

fn main() {
    let lines = read_all_lines();
    let max_len = lines.iter().map(|x| x.len()).max().unwrap() + 1;

    let line_vectors = lines
        .iter()
        .map(move |s| s.clone() + &str::repeat(" ", max_len - s.len()))
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    debug_assert!(line_vectors.iter().all(|x| x.len() == max_len));

    for (index, line) in line_vectors.iter().enumerate() {
        let mut new_line = String::new();
        for (i, c) in line.iter().enumerate() {
            let new_val = if !c.is_whitespace() {
                let pt = Point { row: index, col: i };
                let loc = get_location(&line_vectors, pt);
                let jiggy = loc.get_jiggy();
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

        println!("|{}|", new_line);
    }

    // for line in z {
    //     println!("Original:")
    //     println!("|{:?}|", line);
    // }
}

fn get_location(vec: &Vec<Vec<char>>, anchor: Point) -> Location {
    // println!("Current point: {:?} - CanSample? top {} btm {}", anchor, can_sample_top, can_sample_bottom);
    // println!("Current character: {}", vec[anchor.row][anchor.col]);
    let family = kidnap(vec, &anchor);

    return Location {
       // position: Point { ..anchor },
        value: family[2],
        family,
    };
}

fn kidnap(vec: &Vec<Vec<char>>, anchor: &Point) -> Vec<char> {
    let mut family = Vec::new();
    
    if anchor.row != 0 {
        family.push(vec[anchor.row - 1][anchor.col]); //top
    }
    else {
        family.push(' ')
    }

    family.push(vec[anchor.row][anchor.col -1]); //left
    family.push(vec[anchor.row][anchor.col]); //value
    family.push(vec[anchor.row][anchor.col +1]); //right

    if anchor.row != vec.len() - 1 {
        family.push(vec[anchor.row + 1][anchor.col]); //btm
    }
    else {
        family.push(' ')
    }

    return  family;
}

fn read_all_lines() -> Vec<String> {
    let mut buf = String::new();
    buf.reserve(4096);
    let stdin = io::stdin();

    let mut vec = Vec::<String>::new();

    while let Ok(_x @ 1..) = stdin.read_line(&mut buf) {
        vec.push(format!(" {}", buf.trim_end()));
        buf.clear();
    }

    return vec;
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
            '-' => translate_dashy(self),
            '|' => translate_pipe(self),
            '>' | '<' | '^' | 'v' => translate_pointy(self),
            //'"' => Some('ʺ'), //term output or w/e 
            x  => Some(x),
        }
    }
}

fn translate_dashy(loc: &Location) -> Option<char> {
    match loc.family[..] {
        [' ', '-', '-', ' ', '|'] => Some('┐'),
        ['|', '-', '-', ' ', ' '] => Some('┘'),
        [' ', ' ', '-', '-', '|'] => Some('┌'),
        ['|', ' ', '-', '-', ' '] => Some('└'),
        [' ', '-', '-', '-', '|'] => Some('┬'),
        ['|', '-', '-', '-', ' '] => Some('┴'),
        ['|', '-', '-', '-', '|'] => Some('┼'),
        _ => Some('─')
    }
}

fn translate_pointy(loc: &Location) -> Option<char> {
    match loc.family[..]{
        [_,'-','>',..] => Some('►'),
        [..,'^',_,'|'] => Some('▲'),
        [..,'<','-',_] => Some('►'),
        ['|',_,'v',..] => Some('►'),
        _ => None
    }
    
    //Some('►')
}

fn translate_pipe(loc: &Location) -> Option<char> {
    match loc.family[..] {
        [ _ , ' ', '|', '-', y ] if !y.is_whitespace()=> Some('├'),
        [ ' ' , ' ', '|', '-', y] if y.is_whitespace() => Some('┘'),
        [ y , '-', '|', ' ', x ] if !x.is_whitespace() && !y.is_whitespace()=> Some('┤'),
        [ y , '-', '|', ' ', ' '] if !y.is_whitespace() => Some('┘'),
        [' ' , '-', '|', ' ', y] if !y.is_whitespace() => Some('┐'),
        _ => Some('│')
    }
}
