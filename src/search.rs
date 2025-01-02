use crate::{AttemptParams, Position, DIRECTIONS, ZERO_POS};

pub fn search(params: &mut AttemptParams, size: usize, _depth: usize) -> Option<Vec<Position>> {
    // If the input queue is empty, we have found a solution.
    if params.input_queue.is_empty() {
        return Some(params.solution.clone());
    }

    let element = params.input_queue.pop().unwrap();

    'outer: for (dir_idx, dv) in DIRECTIONS.iter().enumerate() {
        let dir_vector = *dv;

        // Only check directions that are not the same as or the opposite direction to
        // the previously used one. Check all directions for the first iteration.
        if params.direction.is_none() || (dir_vector * params.direction.unwrap()).abs() != 1 {
            // Movement vector as dot product of direction and distance,
            // e.g. (2, 0, 0) for a move to the right by 2.
            let offset = dir_vector * element as i8;
            let new_pos = params.position + offset;

            // Get the coordinate that would be changed by this move
            let relevant_coord = new_pos.coord_by_dir_idx(dir_idx);

            // The sign of the direction we are going in (-1 for down, 1 for up etc.)
            let dir_sign = dir_vector.sign();

            // Because of the way we index directions, we move from UP to DOWN by adding 1, which
            // is the direction sign for UP.
            let opposing_dir_idx = (dir_idx as i8 + dir_sign) as usize;

            // Check if we are violating bounds, e.g. if we are further away than the size of our
            // cube from the furthest tile in the opposite direction.
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
            let original_state = params.state.bits.clone();
            for step in 1..=element {
                // Get the coordinate 1 step further
                let candidate = params.position + dir_vector * step as i8;
                // If that coordinate is (0, 0, 0) or has been visited, our move is illegal.
                if candidate == ZERO_POS || params.state.is_visited(candidate) {
                    params.state.backtrack(original_state); // Restore state
                    continue 'outer;
                }
                params.state.mark_visited(candidate); // Mark now to avoid iterating again
            }

            params.solution.push(new_pos); // Add moves to solution

            // Backup pos and direction
            let original_pos = params.position;
            let original_dir = params.direction;
            params.position = new_pos;
            params.direction = Some(dir_vector);

            if let Some(attempt) = search(params, size, _depth + 1) {
                return Some(attempt);
            }

            // Backtrack
            params.solution.pop();
            params.state.backtrack(original_state); // Restore state
            params.position = original_pos;
            params.direction = original_dir;
            params.bounds = original_bounds;
        }
    }

    // Backtrack the input queue
    params.input_queue.push(element);
    None
}
