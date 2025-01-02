use crate::{AttemptParams, Position, DIRECTIONS, ZERO_POS};

pub fn search(
    params: &mut AttemptParams,
    size: usize,
    verbose: bool,
    _depth: usize,
) -> Option<Vec<(char, u8, Position)>> {
    // If the input queue is empty, we have found a solution.
    if params.input_queue.is_empty() {
        return Some(params.solution.clone());
    }

    let element = params.input_queue.pop().unwrap();

    'outer: for (dir_name, dir_vector) in DIRECTIONS.iter() {
        // Only check directions that are not the same as or the cardinal direction to
        // the previously used one. Check all directions for the first iteration.
        if params.direction.is_none() || (*dir_vector * params.direction.unwrap()).abs() != 1 {
            // Movement vector as dot product of direction and distance,
            // e.g. (2, 0, 0) for a move to the right by 2.
            let offset = *dir_vector * element as i8;
            let new_pos = params.position + offset;

            // Get the coordinate the would change with the candidate move
            let relevant_coord = new_pos.coord_by_dir(dir_name);

            // Check if we are violating bounds
            let opposing_direction = *dir_vector * -1;
            let bound = params.bounds.get(dir_vector).unwrap();
            let opposing_bound = params.bounds.get(&opposing_direction).unwrap();
            if (relevant_coord - opposing_bound).unsigned_abs() >= size as u8 {
                continue;
            }

            // Backup & update bounds
            let original_bounds = params.bounds.clone();
            let dir_sign = dir_vector.sign();
            if (dir_sign > 0 && relevant_coord > *bound)
                || (dir_sign < 0 && relevant_coord < *bound)
            {
                params.bounds.insert(*dir_vector, relevant_coord);
            }

            // Aggregate newly occupied coordinates in steps of 1 and
            // check if any of those coordinates is already occupied.
            // Add them to state if not.
            let original_state = params.state.clone();
            for dist in 1..=element {
                let candidate = params.position + *dir_vector * dist as i8;
                if candidate == ZERO_POS || params.state.is_visited(candidate) {
                    continue 'outer;
                }
                params.state.mark_visited(candidate);
            }

            // Add moves to state and solution
            // for m in &moves {
            //     params.state.insert(*m);
            // }
            params
                .solution
                .push((dir_vector.abbreviation(), element, new_pos));

            // Backup pos and direction
            let original_pos = params.position;
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
            // params.state.retain(|k| !moves.contains(k));
            params.state = original_state;
            params.position = original_pos;
            params.direction = original_dir;
            params.bounds = original_bounds;
        }
    }

    // Backtrack the input queue
    params.input_queue.push(element);
    None
}
