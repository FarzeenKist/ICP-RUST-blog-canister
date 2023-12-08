#[macro_use]
extern crate serde;
use ic_stable_structures::memory_manager;
use std::cell::RefCell;
use ic_stable_structures::memory_manager::MemoryId;

mod model;
use model::*;

thread_local! {
    static MEMORY_MANAGER: RefCell<memory_manager::MemoryManager<ic_stable_structures::DefaultMemoryImpl>> = RefCell::new(
        memory_manager::MemoryManager::init(ic_stable_structures::DefaultMemoryImpl::default())
    );

    static GAMES: RefCell<StableVec<Game>> =
        RefCell::new(StableVec(vec![]));
    
    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );
}

#[ic_cdk::update]
fn create_game() -> Result<String, Error> {
    GAMES.with(|g| {
        let mut games = g.borrow_mut();
        let running_game = games.0.iter().find(|game| {
            game.creator == StableString(ic_cdk::caller().to_string())
                && (game.state == GameState::WaitingForOpponent
                    || game.state == GameState::InProgress)
        });
        match running_game {
            Some(_) => Err(Error::PendingGame {
                msg: "you have a pending game to play!".to_string(),
            }),
            None => {
                let id = ID_COUNTER
                .with(|counter| {
                    let current_value = *counter.borrow().get();
                    counter.borrow_mut().set(current_value + 1)
                })
                .expect("cannot increment id counter");
                let id = format!(
                    "{}-{}",
                    ic_cdk::caller().to_string(),
                    id
                );
                let game = Game {
                    id: StableString(id.clone()),
                    creator: StableString(ic_cdk::caller().to_string()),
                    opponent: None,
                    state: GameState::WaitingForOpponent,
                    board: vec![CoordinateState::Empty; 9],
                    moves: vec![],
                    turn: GamePlayer::Creator,
                    winner: None,
                };
                games.0.push(game);

                Ok(id)
            }
        }
    })
}

fn check_winner(board: &Vec<CoordinateState>) -> Option<GamePlayer> {
    for i in 0..3 {
        if board[i * 3] != CoordinateState::Empty
            && board[i * 3] == board[i * 3 + 1]
            && board[i * 3] == board[i * 3 + 2]
        {
            return Some(match board[i * 3] {
                CoordinateState::X => GamePlayer::Creator,
                CoordinateState::O => GamePlayer::Opponent,
                _ => unreachable!(),
            });
        }
    }

    for i in 0..3 {
        if board[i] != CoordinateState::Empty
            && board[i] == board[i + 3]
            && board[i] == board[i + 6]
        {
            return Some(match board[i] {
                CoordinateState::X => GamePlayer::Creator,
                CoordinateState::O => GamePlayer::Opponent,
                _ => unreachable!(),
            });
        }
    }

    if board[0] != CoordinateState::Empty && board[0] == board[4] && board[0] == board[8] {
        return Some(match board[0] {
            CoordinateState::X => GamePlayer::Creator,
            CoordinateState::O => GamePlayer::Opponent,
            _ => unreachable!(),
        });
    }
    if board[2] != CoordinateState::Empty && board[2] == board[4] && board[2] == board[6] {
        return Some(match board[2] {
            CoordinateState::X => GamePlayer::Creator,
            CoordinateState::O => GamePlayer::Opponent,
            _ => unreachable!(),
        });
    }

    None
}

const PAGE_LIMIT: usize = 10;

#[ic_cdk::query]
fn get_games(page: usize) -> Option<PaginatedResponse<Game>> {
    let start = page * PAGE_LIMIT;
    let end = start + PAGE_LIMIT;
    GAMES.with(|g| {
        let games = g.borrow();

        if start >= games.0.len() {
            return None;
        }

        let response_meta = PaginatedResponseMeta {
            prev: if page > 1 { Some(page - 1) } else { None },
            next: if end < games.0.len() {
                Some(page + 1)
            } else {
                None
            },
            total: games.0.len(),
            page,
            limit: PAGE_LIMIT,
        };
        let response = PaginatedResponse {
            data: games
                .0
                .iter()
                .skip(start)
                .take(PAGE_LIMIT)
                .cloned()
                .collect(),
            meta: response_meta,
        };

        Some(response)
    })
}

#[ic_cdk::query]
fn get_game(game_id: String) -> Option<Game> {
    GAMES.with(|g| {
        g.borrow()
            .0
            .iter()
            .find(|game| game.id == StableString(game_id.clone()))
            .cloned()
    })
}

#[ic_cdk::query]
fn get_player_games(page: usize) -> Option<PaginatedResponse<Game>> {
    let start = page * PAGE_LIMIT;
    let end = start + PAGE_LIMIT;
    GAMES.with(|g| {
        let games = g
            .borrow()
            .0
            .iter()
            .filter(|game| game.creator == StableString(ic_cdk::caller().to_string()))
            .cloned()
            .collect::<Vec<_>>();

        if start >= games.len() {
            return None;
        }

        let response_meta = PaginatedResponseMeta {
            prev: if page > 1 { Some(page - 1) } else { None },
            next: if end < games.len() {
                Some(page + 1)
            } else {
                None
            },
            total: games.len(),
            page,
            limit: PAGE_LIMIT,
        };
        let response = PaginatedResponse {
            data: games.iter().skip(start).take(PAGE_LIMIT).cloned().collect(),
            meta: response_meta,
        };

        Some(response)
    })
}

#[ic_cdk::update]
fn play_move(game_id: String, game_move: String) -> Result<(), Error> {
    GAMES.with(|g| {
        let mut games = g.borrow_mut();
        let game = match games
            .0
            .iter_mut()
            .find(|game| game.id == StableString(game_id.clone()))
        {
            Some(game) => game,
            None => {
                return Err(Error::InvalidGameId {
                    msg: "invalid game id".to_string(),
                })
            }
        };

        match game.state {
            GameState::WaitingForOpponent => {
                game.opponent = Some(StableString(ic_cdk::caller().to_string()));
                game.state = GameState::InProgress;
            }
            GameState::Finished => {
                return Err(Error::FinishedGame {
                    msg: "game has finished".to_string(),
                })
            }
            _ => (),
        }

        if StableString(ic_cdk::caller().to_string()) != game.creator
            && Some(StableString(ic_cdk::caller().to_string())) != game.opponent
        {
            return Err(Error::InvalidPermission {
                msg: "You don't have permission to play this game".to_string(),
            });
        }

        if game.turn == GamePlayer::Creator
            && game.creator != StableString(ic_cdk::caller().to_string())
            || game.turn == GamePlayer::Opponent
                && game.opponent != Some(StableString(ic_cdk::caller().to_string()))
        {
            return Err(Error::InvalidTurn {
                msg: "It's not your turn yet".to_string(),
            });
        }

        let invalid_coordinates_err = Error::InvalidCoordinates {
            msg: "Invalid coordinates".to_string(),
        };

        let mut iter = game_move.split(',').map(|s| s.parse::<u8>());
        let x = match iter.next() {
            Some(x) => match x {
                Ok(x) => x,
                Err(_) => return Err(invalid_coordinates_err),
            },
            None => return Err(invalid_coordinates_err),
        };
        let y = match iter.next() {
            Some(x) => match x {
                Ok(x) => x,
                Err(_) => return Err(invalid_coordinates_err),
            },
            None => return Err(invalid_coordinates_err),
        };

        if x > 2 || y > 2 {
            return Err(invalid_coordinates_err);
        }

        let game_coordinate = match game.turn {
            GamePlayer::Creator => CoordinateState::X,
            GamePlayer::Opponent => CoordinateState::O,
        };

        let game_move = GameMove {
            player: game.turn.clone(),
            x,
            y,
        };
        if game.board[(3 * y + x) as usize] != CoordinateState::Empty {
            return Err(Error::InvalidCoordinates {
                msg: "Coordinate has already been played in".to_string(),
            });
        }

        game.board[(3 * y + x) as usize] = game_coordinate;
        game.moves.push(game_move);

        if game.board.iter().any(|x| *x == CoordinateState::Empty) {
            game.turn = match game.turn {
                GamePlayer::Creator => GamePlayer::Opponent,
                GamePlayer::Opponent => GamePlayer::Creator,
            };
        } else {
            game.state = GameState::Finished;
        }

        if let Some(winner) = check_winner(&game.board) {
            game.state = GameState::Finished;
            game.winner = Some(winner);
        }

        Ok(())
    })
}

#[ic_cdk::update]
fn delete_game(game_id: String) -> Result<(), Error> {
    GAMES.with(|g| {
        let mut games = g.borrow_mut();
        let index = match games
            .0
            .iter()
            .position(|game| game.id == StableString(game_id.clone()))
        {
            Some(index) => index,
            None => {
                return Err(Error::InvalidGameId {
                    msg: "Invalid game id".to_string(),
                })
            }
        };
        let game = games.0.get(index).unwrap();

        if game.creator != StableString(ic_cdk::caller().to_string()) {
            return Err(Error::InvalidPermission {
                msg: "You don't have permission to delete this game".to_string(),
            });
        }

        games.0.remove(index);

        Ok(())
    })
}

ic_cdk::export_candid!();
