mod exercises;

use exercises::*;

pub enum ExercisesType {
    One,
    Two,
    Three,
    Four,
    Five,
}

pub fn execute(task: ExercisesType) {
    match task {
        ExercisesType::One => one(),
        ExercisesType::Two => two(),
        ExercisesType::Three => three(),
        ExercisesType::Four => four(),
        ExercisesType::Five => five(),
    }
}
