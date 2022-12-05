use std::{str::FromStr};
use std::fmt;

pub struct RockPaperScissors {
    data: Vec<(String, String)>
}

#[derive(Debug, Clone)]
struct InvalidStringError {
    s: String
}

impl fmt::Display for InvalidStringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid string: {}", self.s)
    }
}


enum Outcome {
    Win,
    Loss,
    Draw
}

impl Outcome {
    fn value(&self) -> i32 {
        match *self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0
        }
    }

    fn expected_shape(&self, opponent_played: &Shape) -> Shape {
        match self {
            Self::Loss => opponent_played.wins_vs(),
            Self::Win => opponent_played.looses_vs(),
            Self::Draw => opponent_played.clone()
        }
    }
}

impl FromStr for Outcome {
    type Err = InvalidStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Loss),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(InvalidStringError { s: s.to_string() })
        }
    }
}


#[derive(PartialEq, Debug, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

impl Shape {
    fn value(&self) -> i32 {
        match *self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3
        }
    }

    fn wins_vs(&self) -> Shape {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper
        }
    }

    fn looses_vs(&self) -> Shape {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock
        }
    }

    fn outcome(&self, other: &Self) -> Outcome {
        if self == other {
            Outcome::Draw
        } else if self.wins_vs() == *other {
            Outcome::Win
        } else {
            Outcome::Loss
        }
    }

    fn score(&self, outcome: &Outcome) -> i32 {
        self.value() + outcome.value()
    }
}

impl FromStr for Shape {
    type Err=InvalidStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Shape::Rock),
            "B" => Ok(Shape::Paper),
            "C" => Ok(Shape::Scissors),
            "X" => Ok(Shape::Rock),
            "Y" => Ok(Shape::Paper),
            "Z" => Ok(Shape::Scissors),
            _ => Err(InvalidStringError { s: s.to_string() })
        }
    }
}

impl crate::Advent for RockPaperScissors {
    fn new(data: &str) -> Self {
        let data = data
            .lines()
            .map(|l| {
                let (lhs, rhs) = l.split_once(" ").unwrap();
                (String::from(lhs), String::from(rhs))
            }).collect();
        // println!("Data: {:?}", data);
        RockPaperScissors { data }
    }

    fn part_01(&self) -> String {
        self.data.iter()
            .map(|(lhs, rhs)| {
                let lhs = Shape::from_str(lhs).unwrap();
                let rhs = Shape::from_str(rhs).unwrap();
                let outcome = rhs.outcome(&lhs);
                let score = rhs.score(&outcome);           
                // println!("{:?} vs {:?} score: {}", rhs, lhs, score);
                score
            }).sum::<i32>().to_string()
    }

    fn part_02(&self) -> String {
        self.data.iter()
            .map(|(lhs, rhs)| {
                let opponent_played = Shape::from_str(lhs).unwrap();
                let expected_outcome = Outcome::from_str(rhs).unwrap();
                let shape = expected_outcome.expected_shape(&opponent_played);
                let score = shape.value() + expected_outcome.value();
                // println!("{:?} vs {:?} score: {:?}", shape, opponent_played, score);
                score
            }).sum::<i32>().to_string()
    }
}