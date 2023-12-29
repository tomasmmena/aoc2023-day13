use std::env;
use std::fs;
use std::io::{self, BufRead};
use itertools::Itertools;

const SMUDGES: usize = 1;

#[derive(Debug, PartialEq)]
enum TerrainType {
    Stone,
    Ash
}

#[derive(Debug, PartialEq)]
enum Reflection {
    Vertical(usize),
    Horizontal(usize)
}

struct Terrain {
    matrix: Vec<Vec<TerrainType>>
}

impl Terrain {

    /// This function returns a vector of reflections by going over every potential reflection line, first
    /// horizontal going top to bottom and then vertical going left to right. It compares all elements in the matrix
    /// to their equivalents in the other side of the potential reflection line counting the differences and 
    /// discarding a reflection line if the number is different to the expected SMUDGES defined in this module as 
    /// a constant. Setting the value to 0 produces the answer to part one, setting it to 1 produces the answer for 
    /// part 2.
    fn find_reflections(&self) -> Vec<Reflection> {
        let mut reflections: Vec<Reflection> = vec![];
        for i in 1..self.matrix.len() {
            let mut diff: usize = 0;
            for j in 0..(i.min(self.matrix.len() - i)) {
                diff += self.matrix[i - 1 - j].iter().zip(self.matrix[i + j].iter()).filter(|(a, b)| a != b).count();
                if diff > SMUDGES { break; }
            }
            if diff == SMUDGES { reflections.push(Reflection::Horizontal(i)) }
        }
        for i in 1..self.matrix[0].len() {
            let mut diff: usize = 0;
            for j in 0..(i.min(self.matrix[0].len() - i)) {
                diff += (0..self.matrix.len()).filter(|n| self.matrix[*n][i - 1 - j] != self.matrix[*n][i + j]).count();
                if diff > SMUDGES { break; }
            }
            if diff == SMUDGES { reflections.push(Reflection::Vertical(i)) }
        }
        reflections
    }

    fn parse(pattern: Vec<String>) -> Self {
        Terrain { 
            matrix: pattern
                .into_iter()
                .map(|row| row.chars().map(|c| match c {
                        '#' => TerrainType::Stone,
                        '.' => TerrainType::Ash,
                        _ => panic!("Invalid terrain!")
                    }).collect())
                .collect::<Vec<Vec<TerrainType>>>()
        }
    }

}


fn main() {
    let path = env::args().nth(1).expect("Missing required parameter path!");

    let patterns: Vec<Terrain> = io::BufReader::new(
        fs::File::open(path).expect("Could not open file!"))
        .lines()
        .map(|l| l.expect("Failed to parse line!"))
        .group_by(|l| l == "")
        .into_iter()
        .filter_map(|l| {
            let pattern: Vec<String> = l.1.collect();
            if pattern.len() == 1 { return None }  // discard blank line separators
            Some(Terrain::parse(pattern))
        })
        .collect();

    println!("{} patterns parsed.", patterns.len());

    let total: usize = patterns
        .iter()
        .flat_map(|p| p.find_reflections())
        .map(|r| match r {
            Reflection::Horizontal(i) => 100 * i,
            Reflection::Vertical(i) => i
        })
        .sum();

    println!("Reflection total: {}", total);

}


#[cfg(test)]
mod tests {
    use crate::{Terrain, Reflection};

    #[test]
    fn test_find_reflections() {
        let terrain = Terrain::parse(
            vec![
                String::from("#..#..#"),
                String::from("#.###.#"),
                String::from("#..#..#"),
                String::from("#..#..#"),
                String::from("#.###.#"),
            ]
        );
        assert_eq!(terrain.find_reflections(), vec![Reflection::Horizontal(3)]);

        let terrain = Terrain::parse(
            vec![
                String::from("#..#..#"),
                String::from("####.##"),
                String::from("#..#..#"),
                String::from("####..#"),
                String::from("#####.#"),
            ]
        );
        assert_eq!(terrain.find_reflections(), vec![Reflection::Vertical(2)]);

        let terrain = Terrain::parse(
            vec![
                String::from("#..#..#"),
                String::from("####.##"),
                String::from("#..#..#"),
                String::from("####.##"),
                String::from("####.##"),
            ]
        );
        assert_eq!(terrain.find_reflections(), vec![
            Reflection::Horizontal(4), 
            Reflection::Vertical(2)]);

        let terrain = Terrain::parse(
            vec![
                String::from("#...##..#"),
                String::from("#....#..#"),
                String::from("..##..###"),
                String::from("#####.##."),
                String::from("#####.##."),
                String::from("..##..###"),
                String::from("#....#..#"),
            ]
        );
        assert_eq!(terrain.find_reflections(), vec![
            Reflection::Horizontal(4)]);
    }

}
