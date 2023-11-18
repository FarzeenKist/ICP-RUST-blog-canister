#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_stable_structures::{memory_manager, BoundedStorable};
use std::{borrow::Cow, cell::RefCell, slice::SliceIndex, vec::IntoIter};

#[derive(candid::CandidType, Deserialize, Serialize, Debug)]
enum Error {
    InvalidCoordinates { msg: String },
    PendingGame { msg: String },
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

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
enum CoordinateState {
    #[default]
    Empty,
    X,
    O,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
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
    creator: StableString,
    opponent: Option<StableString>,
    state: GameState,
    board: Vec<CoordinateState>,
    // moves: Vec<GameMove, Memory>,
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

// #[derive(candid::CandidType)]
//  struct StableVec<T: BoundedStorable, M: ic_stable_structures::Memory>(ic_stable_structures::StableVec<T, M>);
//
// impl ic_stable_structures::Storable for StableVec<StableString, Memory> {
//     fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
//         Cow::Owned(Encode!(self).unwrap())
//     }
//
//     fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
//         Decode!(bytes.as_ref(), Self).unwrap()
//     }
// }
//
// impl ic_stable_structures::BoundedStorable for StableVec<StableString, Memory> {
//     const MAX_SIZE: u32 = 1024;
//     const IS_FIXED_SIZE: bool = false;
// }

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

    static PLAYER_GAMES: RefCell<ic_stable_structures::StableBTreeMap<StableString, StableVec<u8>, Memory>> =
        RefCell::new(ic_stable_structures::StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(memory_manager::MemoryId::new(1)))
        ))
}

#[ic_cdk::query]
fn get_games() -> StableVec<Game> {
    GAMES.with(|g| g.borrow().clone())
}

#[ic_cdk::query]
fn get_player_games() -> StableVec<Game> {
    let games = PLAYER_GAMES.with(|pg| {
        pg.borrow()
            .get(&StableString(ic_cdk::caller().to_string()))
            .unwrap()
            .0
            .iter()
            .map(|game_index| {
                GAMES.with(|g| {
                    let _game = g.borrow();
                    let game = _game.0.get(*game_index as usize).unwrap();
                    game.clone()
                })
            })
            .collect::<Vec<Game>>()
    });

    StableVec(games)
}

#[ic_cdk::update]
fn create_game() -> Result<bool, Error> {
    match PLAYER_GAMES.with(|g| g.borrow().get(&StableString(ic_cdk::caller().to_string()))) {
        Some(games) => {
            let running_game = games.0.iter().find(|game_index| {
                GAMES.with(|g| {
                    let _game = g.borrow();
                    let game = _game.0.get(**game_index as usize).unwrap();
                    game.state == GameState::WaitingForOpponent
                        || game.state == GameState::InProgress
                })
            });
            match running_game {
                Some(_) => {
                    return Err(Error::PendingGame {
                        msg: "you have a pending game to play!".to_string(),
                    })
                }
                None => (),
            }
        }
        None => {
            let games = StableVec(vec![]);
            PLAYER_GAMES.with(|g| {
                g.borrow_mut()
                    .insert(StableString(ic_cdk::caller().to_string()), games)
            });
        }
    };

    let game = Game {
        creator: StableString(ic_cdk::caller().to_string()),
        opponent: None,
        state: GameState::WaitingForOpponent,
        board: vec![CoordinateState::Empty; 9],
    };
    GAMES.with(|g| {
        g.borrow_mut().0.push(game);
        PLAYER_GAMES.with(|pg| {
            pg.borrow_mut()
                .get(&StableString(ic_cdk::caller().to_string()))
                .unwrap()
                .0
                .push(g.borrow().0.len() as u8 - 1);
        });
    });

    Ok(true)
}

ic_cdk::export_candid!();
