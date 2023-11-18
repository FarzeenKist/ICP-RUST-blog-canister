#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_stable_structures::{memory_manager, BoundedStorable};
use std::{borrow::Cow, cell::RefCell};

#[derive(candid::CandidType, Deserialize, Serialize, Debug)]
enum Error {
    InvalidGameId { msg: String },
    InvalidCoordinates { msg: String },
    InvalidTurn { msg: String },
    PendingGame { msg: String },
    FinishedGame { msg: String },
    InvalidPermission { msg: String },
}

type Memory =
    ic_stable_structures::memory_manager::VirtualMemory<ic_stable_structures::DefaultMemoryImpl>;
type IdCell = ic_stable_structures::Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, PartialEq, Eq)]
enum GameState {
    WaitingForOpponent,
    InProgress,
    Finished,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
enum CoordinateState {
    #[default]
    Empty,
    X,
    O,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, PartialEq, Eq)]
enum GamePlayer {
    Creator,
    Opponent,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct GameMove {
    player: GamePlayer,
    x: u8,
    y: u8,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct Game {
    id: StableString,
    creator: StableString,
    opponent: Option<StableString>,
    state: GameState,
    board: Vec<CoordinateState>,
    moves: Vec<GameMove>,
    turn: GamePlayer,
}

impl ic_stable_structures::Storable for GameState {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl ic_stable_structures::BoundedStorable for GameState {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl ic_stable_structures::Storable for CoordinateState {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl ic_stable_structures::BoundedStorable for CoordinateState {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl ic_stable_structures::Storable for GamePlayer {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl ic_stable_structures::BoundedStorable for GamePlayer {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl ic_stable_structures::Storable for GameMove {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl ic_stable_structures::BoundedStorable for GameMove {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl ic_stable_structures::Storable for Game {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl ic_stable_structures::BoundedStorable for Game {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(candid::CandidType, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Eq, Ord)]
struct StableString(String);

// impl From<&str> for StableString {
//     fn from(value: &str) -> Self {
//         StableString(value.to_string())
//     }
// }
//
// impl From<String> for StableString {
//     fn from(value: String) -> Self {
//         StableString(value.to_string())
//     }
// }
//
// impl From<candid::Principal> for StableString {
//     fn from(principal: candid::Principal) -> Self {
//         StableString::from(principal.to_string())
//     }
// }

impl ic_stable_structures::Storable for StableString {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl ic_stable_structures::BoundedStorable for StableString {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(candid::CandidType, Serialize, Deserialize, Clone)]
struct StableVec<T: BoundedStorable>(Vec<T>);

// impl IntoIterator for StableVec<StableString> {
//     type Item = StableString;
//     type IntoIter = IntoIter<Self::Item>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         self.0.into_iter()
//     }
// }

impl ic_stable_structures::Storable for StableVec<StableString> {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl ic_stable_structures::BoundedStorable for StableVec<StableString> {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl ic_stable_structures::Storable for StableVec<Game> {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl ic_stable_structures::BoundedStorable for StableVec<Game> {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl ic_stable_structures::Storable for StableVec<u8> {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl ic_stable_structures::BoundedStorable for StableVec<u8> {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<memory_manager::MemoryManager<ic_stable_structures::DefaultMemoryImpl>> = RefCell::new(
        memory_manager::MemoryManager::init(ic_stable_structures::DefaultMemoryImpl::default())
    );

    static GAMES: RefCell<StableVec<Game>> =
        RefCell::new(StableVec(vec![]));
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
                let id = format!(
                    "{}-{}",
                    ic_cdk::caller().to_string(),
                    games.0.len().to_string()
                );
                let game = Game {
                    id: StableString(id.clone()),
                    creator: StableString(ic_cdk::caller().to_string()),
                    opponent: None,
                    state: GameState::WaitingForOpponent,
                    board: vec![CoordinateState::Empty; 9],
                    moves: vec![],
                    turn: GamePlayer::Creator,
                };
                games.0.push(game);

                Ok(id)
            }
        }
    })
}

#[ic_cdk::query]
fn get_games() -> StableVec<Game> {
    GAMES.with(|g| g.borrow().clone())
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
fn get_player_games() -> StableVec<Game> {
    GAMES.with(|g| {
        let games = g
            .borrow()
            .0
            .iter()
            .cloned()
            .filter(|game| game.creator == StableString(ic_cdk::caller().to_string()))
            .collect::<Vec<Game>>();

        StableVec(games)
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

        let game_coordinate = CoordinateState::X;
        let game_move = GameMove {
            player: GamePlayer::Creator,
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

        Ok(())
    })
}

#[ic_cdk::update]
fn delete_game(game_id: String) -> Result<(), Error> {
    GAMES.with(|g| {
        let mut games = g.borrow_mut();
        for (index, game) in games.clone().0.iter().enumerate() {
            if game.id == StableString(game_id.clone()) {
                if game.creator != StableString(ic_cdk::caller().to_string()) {
                    return Err(Error::InvalidPermission {
                        msg: "You don't have permission to delete this game".to_string(),
                    });
                }

                games.0.remove(index);
            }
        }

        return Err(Error::InvalidGameId {
            msg: "Invalid game id".to_string(),
        });
    })
}

ic_cdk::export_candid!();
