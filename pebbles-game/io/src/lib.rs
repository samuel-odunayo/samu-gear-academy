use gstd::prelude::*;

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub struct PebblesInit {
    pub difficulty: DifficultyLevel,
    pub pebbles_count: u32,
    pub max_pebbles_per_turn: u32,
}

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub enum DifficultyLevel {
    #[default]
    Easy,
    Hard,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum PebblesAction {
    Turn(u32),
    GiveUp,
    Restart {
        difficulty: DifficultyLevel,
        pebbles_count: u32,
        max_pebbles_per_turn: u32,
    },
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum PebblesEvent {
    CounterTurn(u32),
    Won(Player),
}

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub struct GameState {
    pub pebbles_count: u32,
    pub max_pebbles_per_turn: u32,
    pub pebbles_remaining: u32,
    pub difficulty: DifficultyLevel,
    pub first_player: Player,
    pub winner: Option<Player>,
}

impl Metadata for PebblesMetadata {
    type Init = In<PebblesInit>;
    type Handle = InOut<PebblesAction, PebblesEvent>;
    type State = Out<GameState>;
    type Reply = ();
    type Others = ();
    type Signal = ();
}

pub fn init(msg: &PebblesInit) -> GameState {
    // Check input data for validness
    assert!(msg.pebbles_count > 0 && msg.max_pebbles_per_turn > 0, "Invalid input data");

    // Choose the first player randomly
    let first_player = if get_random_u32() % 2 == 0 {
        Player::User
    } else {
        Player::Program
    };

    // Process the first turn if the first player is Program
    let mut pebbles_remaining = msg.pebbles_count;
    if first_player == Player::Program {
        let program_turn = get_program_turn(msg.difficulty, pebbles_remaining);
        pebbles_remaining -= program_turn;
    }

    // Fill the GameState structure
    GameState {
        pebbles_count: msg.pebbles_count,
        max_pebbles_per_turn: msg.max_pebbles_per_turn,
        pebbles_remaining,
        difficulty: msg.difficulty,
        first_player,
        winner: None,
    }
}

pub fn handle(game_state: &mut GameState, action: &PebblesAction) -> PebblesEvent {
    match action {
        PebblesAction::Turn(turn_count) => {
            // Check input data for validness
            assert!(*turn_count > 0 && *turn_count <= game_state.max_pebbles_per_turn, "Invalid turn count");

            // Process the User's turn
            game_state.pebbles_remaining -= turn_count;
            if game_state.pebbles_remaining == 0 {
                return PebblesEvent::Won(Player::User);
            }

            // Process the Program's turn
            let program_turn = get_program_turn(game_state.difficulty, game_state.pebbles_remaining);
            game_state.pebbles_remaining -= program_turn;
            if game_state.pebbles_remaining == 0 {
                return PebblesEvent::Won(Player::Program);
            }

            PebblesEvent::CounterTurn(program_turn)
        }
        PebblesAction::GiveUp => {
            // User gives up, Program wins
            PebblesEvent::Won(Player::Program)
        }
        PebblesAction::Restart { difficulty, pebbles_count, max_pebbles_per_turn } => {
            // Restart the game with new parameters
            *game_state = init(&PebblesInit {
                difficulty: *difficulty,
                pebbles_count: *pebbles_count,
                max_pebbles_per_turn: *max_pebbles_per_turn,
            });
            PebblesEvent::CounterTurn(0) // No pebbles removed
        }
    }
}

pub fn state() -> GameState {
    // Return the current game state
    // This function will be called by the IDEA portal to get the game state
    // It will be used for the assessment and further processing
    GameState { .. }
}


// Helper function for getting a random 32-bit number
fn get_random_u32() -> u32 {
    let salt = msg::id();
    let (hash, _num) = exec::random(salt.into()).expect("get_random_u32(): random call failed");
    u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
}

// Unit tests
#[cfg(test)]
mod tests {
    // Write your unit tests here
}
