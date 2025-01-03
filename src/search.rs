use crate::{AttemptParams, Bounds, Direction, Position, DIRECTIONS, ZERO_POS};

pub fn search(params: &mut AttemptParams, size: usize, _depth: usize) -> Option<Vec<Position>> {
    // If the input queue is empty, we have found a solution.
    if params.input_queue.is_empty() {
        return Some(params.solution.clone());
    }

    let element = params.input_queue.pop().unwrap();

    // for _ in 1..=depth {
    //     print!("*");
    // }
    // println!();

    // We don't have to check bounds or collisions on the first move
    // if depth == 1 {
    //     initial_move(params, element);
    //     depth += 1;
    // }

    for (dir_idx, &dir_vector) in DIRECTIONS.iter().enumerate() {
        // Only check directions that are not the same as or the opposite direction to
        // the previously used one. Check all directions for the first iteration.
        if (dir_vector * params.direction).abs() != 1 {
            // Movement vector as dot product of direction and distance,
            // e.g. (2, 0, 0) for a move right by 2.
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

            let original_bounds = params.bounds; // Backup bounds
            params.bounds.set_by_idx(dir_idx, relevant_coord, dir_sign); // Update bounds

            // Iterate over potentially occupied coordinates in steps of 1 and
            // check if any of those coordinates are already occupied.
            // params.state.backup();
            let original_state = params.state.bits.clone();
            if !check_and_apply_moves(params, dir_vector, element, new_pos) {
                params.state.backtrack(original_state);
                continue;
            }

            // Backup and update pos and direction
            let original_pos = params.position;
            let original_dir = params.direction;
            params.position = new_pos;
            params.direction = dir_vector;

            // Recursion
            if let Some(attempt) = search(params, size, _depth + 1) {
                return Some(attempt);
            }

            backtrack(
                params,
                original_state,
                original_pos,
                original_dir,
                original_bounds,
            );
        }
    }

    // Backtrack the input queue
    params.input_queue.push(element);
    None
}

// fn initial_move(params: &mut AttemptParams, element: u8) {
//     let offset = params.direction * element as i8;
//     let new_pos = params.position + offset;
//     // println!(
//     //     "Making initial move {} from {} to {}",
//     //     params.direction, params.position, new_pos
//     // );
//     for step in 1..=element {
//         // Get the next coordinate
//         let candidate = params.position + params.direction * step as i8;
//         params.state.mark_visited(candidate);
//     }
//     params.bounds.set_by_idx(0, new_pos.y, 1); // Update bounds
//     params.solution.push(new_pos); // Add moves to solution
//     params.position = new_pos;
// }

fn check_and_apply_moves(
    params: &mut AttemptParams,
    dir: Direction,
    element: u8,
    new_pos: Position,
) -> bool {
    for step in 1..=element {
        // Get the coordinate 1 step further
        let candidate = params.position + dir * step as i8;
        // If that coordinate is (0, 0, 0) or has been visited, our move is illegal.
        if candidate == ZERO_POS || params.state.is_visited(candidate) {
            return false;
        }
        params.state.mark_visited(candidate); // Mark now to avoid iterating again
    }

    params.solution.push(new_pos); // Add moves to solution
    true
}

fn backtrack(
    params: &mut AttemptParams,
    original_state: Vec<u64>,
    original_pos: Position,
    original_dir: Direction,
    original_bounds: Bounds,
) {
    params.solution.pop();
    params.state.backtrack(original_state);
    params.position = original_pos;
    params.direction = original_dir;
    params.bounds = original_bounds;
}
