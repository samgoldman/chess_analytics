
pub fn has_opening(game: crate::chess_flatbuffers::chess::Game, opening: Vec<(crate::chess_flatbuffers::chess::File, u8)>) -> bool {
    // Extract files - if none, game has no opening, so it doesn't have this opening
    let files = match game.to_files() {
        Some(files) => files,
        None => return false
    };

    // Do the same with ranks (although if a game has ranks but not files, something's wrong)
    let ranks = match game.to_ranks() {
        Some(ranks) => ranks,
        None => return false
    };

    // Verify this game has enough moves for the given opening
    if files.len() < opening.len() {
        return false;
    }

    // Create iterables to make the next step cleaner
    let mut file_iter = files.iter();
    let mut rank_iter = ranks.iter();

    // For each expected moving in the opening, if the game moves don't match, just return false
    for (expected_file, expected_rank) in opening {
        if expected_file != file_iter.next().unwrap() || expected_rank != *rank_iter.next().unwrap() {
            return false
        }
    }

    true
}