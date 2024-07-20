mod exercises;

use exercises::*;

pub enum ExercisesType {
    One,
}

pub fn execute(task: ExercisesType) {
    match task {
        ExercisesType::One => one(),
    }
}
