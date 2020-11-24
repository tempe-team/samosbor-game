use serde::{Deserialize, Serialize};
use serde_json;
use std::clone::Clone;
use std::marker::{Copy, Sync, Send};

use crate::location::{
    Unit,
    Direction,
    Position,
};

/// Enum for all exceptions. Actually need only for logging.
/// Should cause reask of valid state (if raised on client while evaluating event from server, which considered successful by server)
/// or just log(if raised as result of io from user)
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum SamosborError {
    OutOfBorders,
    Collision,
    NoSuchUnit,
    NoSuchClient, // Server error. Somebody trying to connect under not existing id.
    NoEmptyTiles, // Whole location filled with units
    UnexpectedInput,
    InternalLogicError, // Should never happen
    AlreadyHere,
    TilesNotANeighbors,
}

unsafe impl Send for SamosborError {}
unsafe impl Sync for SamosborError {}

/// Primitive for communication beetween components of Samosbor infrastructure
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SamosborMessage {
    SmsbrEvent(Event),
    SmsbrIntention(Intention),
    SmsbrState(serde_json::value::Value), // State itself.
    SmsbrError(SamosborError),
}
unsafe impl Send for SamosborMessage {}
unsafe impl Sync for SamosborMessage {}

/// What client want to do.
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Intention {
    GoToPosition { unit: Unit, position: Position },
    ClientConnect (Unit),
    ClientDisconnect (Unit),
}

/// Thing which going deterministic mutate state.
/// What is considered as happened by game server.
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Event {
    Step {
        unit: Unit,
        direction: Direction,
    },
    AddUnit {
        unit: Unit,
        position: Position,
    }, // add unit spawned in other state
    RemoveUnit (Unit),
    ClientConnect (Unit),
    ClientDisconnect (Unit),
}

unsafe impl Send for Event {}
unsafe impl Sync for Event {}
