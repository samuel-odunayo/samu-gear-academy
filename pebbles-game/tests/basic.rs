// pebbles-game/io/tests/unit_tests.rs

#[cfg(test)]
mod tests {
    use super::*;
    use pebbles_game_io::*;

    #[test]
    fn test_init() {
        // Test initialization of the game
        let init_msg = PebblesInit {
            difficulty: DifficultyLevel::Easy,
            pebbles_count: 15,
            max_pebbles_per_turn: 2,
        };
        let game_state = init(&init_msg);

        // Check if the game state is correctly initialized
        assert_eq!(game_state.pebbles_count, init_msg.pebbles_count);
        assert_eq!(game_state.max_pebbles_per_turn, init_msg.max_pebbles_per_turn);
        assert_eq!(game_state.difficulty, init_msg.difficulty);
        assert_eq!(game_state.pebbles_remaining, init_msg.pebbles_count);
        assert_ne!(game_state.first_player, Player::Unknown);
        assert_eq!(game_state.winner, None);
    }

    #[test]
    fn test_handle_user_turn() {
        // Test handling of user's turn
        let mut game_state = GameState {
            pebbles_count: 15,
            max_pebbles_per_turn: 2,
            pebbles_remaining: 15,
            difficulty: DifficultyLevel::Easy,
            first_player: Player::User,
            winner: None,
        };

        // Simulate user's turn to remove 1 pebble
        let user_action = PebblesAction::Turn(1);
        let event = handle(&game_state, &user_action);

        // Check if the event reflects the correct state after user's turn
        match event {
            PebblesEvent::CounterTurn(pebbles_removed) => {
                assert_eq!(pebbles_removed, 1);
                assert_eq!(game_state.pebbles_remaining, 14);
            }
            _ => panic!("Expected CounterTurn event"),
        }
    }

    #[test]
    fn test_handle_program_turn() {
        // Test handling of program's turn
        let mut game_state = GameState {
            pebbles_count: 15,
            max_pebbles_per_turn: 2,
            pebbles_remaining: 15,
            difficulty: DifficultyLevel::Easy,
            first_player: Player::Program,
            winner: None,
        };

        // Simulate program's turn
        let event = handle(&game_state, &PebblesAction::Restart {
            difficulty: DifficultyLevel::Easy,
            pebbles_count: 15,
            max_pebbles_per_turn: 2,
        });

        // Check if the event reflects the correct state after program's turn
        match event {
            PebblesEvent::CounterTurn(pebbles_removed) => {
                assert!(pebbles_removed > 0 && pebbles_removed <= game_state.max_pebbles_per_turn);
                assert!(game_state.pebbles_remaining <= game_state.pebbles_count);
            }
            _ => panic!("Expected CounterTurn event"),
        }
    }

    // Add more test cases as needed for additional functionalities and error handling
}
