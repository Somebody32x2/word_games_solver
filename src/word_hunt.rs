use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Coord {
    x: u32,
    y: u32,
}

impl Coord {
    fn from(x: u32, y: u32) -> Coord {
        return Coord { x, y };
    }
}

pub(crate) fn main() {
    const USE_INPUT: bool = false;
    let letters_strs = [
        "SORL",
        "HOSL",
        "EWUN",
        "NICE"
    ];
    let mut letter_pos: HashMap<char, Vec<Coord>> = HashMap::new();
    for (y, row) in letters_strs.iter().map(|row| row.to_lowercase().as_bytes().iter().map(|x| char::from(*x)).collect::<Vec<char>>()).enumerate() {
        for x in 0..row.len() {
            // println!("{}", x);
            letter_pos.entry(row[x]).or_insert_with(|| Vec::with_capacity(1));
            letter_pos.entry(row[x]).and_modify(|coord| coord.push(Coord::from(x as u32, y as u32)));
        }
    }

    let words_str = fs::read_to_string("words_alpha_three.txt").unwrap();
    let words: Vec<&str> = words_str.split_terminator(&['\r', '\n'][..]).collect();

    let mut word_paths: HashMap<&str, Vec<Vec<Coord>>> = HashMap::new();

    for &word in words.iter() {
        if !word.is_empty() {
            // println!("Checking {}", word);
            let char_vec = word.as_bytes().iter().map(|x| char::from(*x)).collect::<Vec<char>>();
            if let Some(word_path) =
                search_word_paths(&char_vec,
                                  &mut Vec::<Coord>::new(), &letter_pos) {
                word_paths.insert(word, word_path);
                // println!("{}", word)
            }

        }
    }

    let mut valid_words: Vec<&str> = word_paths.keys().copied().collect();
    valid_words.sort_by(sort_by_length);
    valid_words.reverse();
    println!("{:#?}", valid_words)
}

fn search_word_paths(word: &Vec<char>, path: &mut Vec<Coord>, word_pos_dict: &HashMap<char, Vec<Coord>>) -> Option<Vec<Vec<Coord>>> {
    if !word_pos_dict.contains_key(&word[path.len()]) {
        None
    } else {
        // construct a new vec of this + all subpaths to rest of word
        let mut paths_back: Vec<Vec<Coord>> = Vec::new();
        for char_pos in word_pos_dict.get(&word[path.len()]).unwrap() {
            // If this is the first entry in the path or this position of the needed char is in range and hasn't been seen before
            if path.is_empty() || ((char_pos.x as i32 - path.last().unwrap().x as i32).abs() <= 1 && (char_pos.y as i32 - path.last().unwrap().y as i32).abs() <= 1 && !path.contains(char_pos)) {
                let mut new_path: Vec<Coord> = path.to_vec();
                new_path.push(*char_pos);
                if new_path.len() == word.len() {
                    paths_back.push(new_path);
                } else if let Some(mut new_paths) = search_word_paths(&word, &mut new_path, word_pos_dict) {
                    // println!("found paths back to {:#?}:, {:#?}", word, new_paths);
                    // Add the new paths to the paths back
                    paths_back.append(&mut new_paths);

                }
            }
        }
        if !paths_back.is_empty() {
            return Some(paths_back);
        }
        None
    }
}
fn sort_by_length<'a, 'b>(a: &'a &str, b: &'b &str) -> Ordering {
   if a.len() > b.len() {
       Ordering::Less
   } else if a.len() == b.len() {
       return Ordering::Equal
   } else {
       return Ordering::Greater
   }
}
// 3l = 100pts
// 4l = 400pts