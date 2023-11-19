use candid::{Decode, Encode};
use std::borrow::Cow;

#[derive(candid::CandidType, Deserialize, Serialize, Debug)]
pub enum Error {
    InvalidGameId { msg: String },
    InvalidCoordinates { msg: String },
    InvalidTurn { msg: String },
    PendingGame { msg: String },
    FinishedGame { msg: String },
    InvalidPermission { msg: String },
    InvalidPage { msg: String }
}

pub type Memory =
    ic_stable_structures::memory_manager::VirtualMemory<ic_stable_structures::DefaultMemoryImpl>;
pub type IdCell = ic_stable_structures::Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum GameState {
    WaitingForOpponent,
    InProgress,
    Finished,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub enum CoordinateState {
    #[default]
    Empty,
    X,
    O,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum GamePlayer {
    Creator,
    Opponent,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
pub struct GameMove {
    pub player: GamePlayer,
    pub x: u8,
    pub y: u8,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
pub struct Game {
    pub id: StableString,
    pub creator: StableString,
    pub opponent: Option<StableString>,
    pub state: GameState,
    pub board: Vec<CoordinateState>,
    pub moves: Vec<GameMove>,
    pub turn: GamePlayer,
    pub winner: Option<GamePlayer>,
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
pub struct StableString(pub String);

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
pub struct StableVec<T: ic_stable_structures::BoundedStorable>(pub Vec<T>);

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

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
pub struct PaginatedResponseMeta {
    pub prev: Option<usize>,
    pub next: Option<usize>,
    pub total: usize,
    pub page: usize,
    pub limit: usize
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub meta: PaginatedResponseMeta,
}
