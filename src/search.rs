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

    'outer: for (dir_idx, (dir_name, dir_vector)) in DIRECTIONS.iter().enumerate() {
        // Only check directions that are not the same as or the opposite direction to
        // the previously used one. Check all directions for the first iteration.
        if params.direction.is_none() || (*dir_vector * params.direction.unwrap()).abs() != 1 {
            // Movement vector as dot product of direction and distance,
            // e.g. (2, 0, 0) for a move to the right by 2.
            let offset = *dir_vector * element as i8;
            let new_pos = params.position + offset;

            // Get the coordinate the would change with the candidate move
            let relevant_coord = new_pos.coord_by_dir(dir_name);

            // Check if we are violating bounds
            let dir_sign = dir_vector.sign();
            let opposing_dir_idx = (dir_idx as i8 + dir_sign) as usize;
            let opposing_bound = params.bounds.get_by_index(opposing_dir_idx);
            if (relevant_coord - opposing_bound).unsigned_abs() >= size as u8 {
                continue;
            }

            // Backup & update bounds
            let original_bounds = params.bounds;
            params.bounds.set_by_idx(dir_idx, relevant_coord, dir_sign);

            // Iterate over potentially occupied coordinates in steps of 1 and
            // check if any of those coordinates are already occupied.
            // Add them to state if not.
            let original_state = params.state.clone();
            for step in 1..=element {
                let candidate = params.position + *dir_vector * step as i8;
                if candidate == ZERO_POS || params.state.is_visited(candidate) {
                    params.state = original_state;
                    continue 'outer;
                }
                params.state.mark_visited(candidate);
            }

            // Add moves to solution
            params.solution.push((dir_vector.abbr(), element, new_pos));

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
