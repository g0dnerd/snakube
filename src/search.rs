use std::collections::HashSet;

use crate::{intersects, AttemptParams, Position, DIRECTIONS, ZERO_POS};

pub fn search(
    params: &mut AttemptParams,
    size: usize,
    verbose: bool,
    _depth: usize,
) -> Option<Vec<(char, u8, Position)>> {
    if params.input_queue.is_empty() {
        return Some(params.solution.clone());
    }

    let element = params.input_queue.pop().unwrap();

    for (dir_name, dir_vector) in DIRECTIONS.iter() {
        if params.direction.is_none() || (*dir_vector * params.direction.unwrap()).abs() != 1 {
            let offset = *dir_vector * element as i8;
            let new_pos = params.position + offset;

            let relevant_coord = match *dir_name {
                "UP" | "DOWN" => new_pos.y,
                "LEFT" | "RIGHT" => new_pos.x,
                "OUT" | "IN" => new_pos.z,
                _ => unreachable!(),
            };

            let opposing_direction = *dir_vector * -1;

            let bound = params.bounds.get(dir_vector).unwrap();
            let opposing_bound = params.bounds.get(&opposing_direction).unwrap();

            if (relevant_coord - opposing_bound).unsigned_abs() >= size as u8 {
                continue;
            }

            // Aggregate newly occupied coordinates
            let moves: HashSet<Position> = (1..=element)
                .map(|dist| params.position + *dir_vector * dist as i8)
                .collect();

            // Check if any of those coordinates is already occupied
            if moves.contains(&ZERO_POS) || intersects([moves.clone(), params.state.clone()].iter())
            {
                continue;
            }

            // Backup & update bounds
            let bounds = params.bounds.clone();
            let dir_sign = dir_vector.sign();
            if (dir_sign > 0 && relevant_coord > *bound)
                || (dir_sign < 0 && relevant_coord < *bound)
            {
                params.bounds.insert(*dir_vector, relevant_coord);
            }

            // Add moves to state
            for m in &moves {
                params.state.insert(*m);
            }

            params.solution.push((
                dir_vector.to_string().chars().next().unwrap(),
                element,
                new_pos,
            ));

            // Backup pos and direction
            let pos = params.position;
            let original_dir = params.direction;
            params.position = new_pos;
            params.direction = Some(*dir_vector);

            if verbose {
                for s in &params.solution {
                    print!("{}", s.0);
                }
                println!();
            }

            if let Some(attempt) = search(params, size, verbose, _depth + 1) {
                return Some(attempt);
            }

            // Backtrack
            params.solution.pop();
            params.state.retain(|k| !moves.contains(k));
            params.position = pos;
            params.direction = original_dir;
            params.bounds = bounds;
        }
    }
    params.input_queue.push(element);
    None
}
