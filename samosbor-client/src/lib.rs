mod utils;
extern crate web_sys;

use serde::{Deserialize, Serialize};
use serde_json::{
    from_value
};
use wasm_bindgen::prelude::*;
use legion::*;

use samosbor_core::*;
use samosbor_core::protocol::{
    SamosborError,
    SamosborError::{UnexpectedInput},
    SamosborMessage::{
        SmsbrEvent,
        SmsbrState,
        SmsbrIntention,
    },
    Event,
    Intention,
};
use samosbor_core::serialization::{deserialize_state};
use samosbor_core::location::{
    Direction,
    Direction::{N,E,S,W},
    Unit,
    Position,
    eval_direction,
};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct ClientState {
    unit: Unit,
    world: World,
    resources: Resources,
    view_point: Position,
    window_radius: usize
}

/// short alias for serialize
pub fn ser<T:Serialize>(v: T) -> String {
    match serde_json::to_string(&v) {
        Ok (v) => v,
        Err (err) => panic!(err),
    }
}

/// short alias for deserialize
pub fn deser<'a, T:Deserialize<'a>>(source: &'a str) -> Result<T, SamosborError> {
    match serde_json::from_str(source) {
        Ok (v) => Ok (v),
        Err (err) => {
            eprintln!("{}", err);
            Err(UnexpectedInput)
        },
    }
}

#[wasm_bindgen]
impl ClientState {
    // Commands from JS to webasm.
    // Webasm recieves comand from js io, evaluates
    // then tell what JS should send to server.
    pub fn step(&mut self, d:char) -> String {
        let unit = self.unit;
        fn char2dir (c: char) -> Result<Direction, String> {
            match c {
                'u' => Ok(N),
                'r' => Ok(E),
                'l' => Ok(W),
                'd' => Ok(S),
                _   => Err(String::from("Wrong key kode for step")),
            }
        }
        match char2dir(d) {
            Ok(direction) => ser (
                SmsbrIntention(
                    Intention::Step {
                        unit: unit,
                        direction: direction,
                    }
                )
            ),
            Err(err) => {
                ser(err)
            },
        }
    }

    pub fn zoom(&mut self, zoom: i32) {
        if (self.window_radius as i32 + zoom) >= 5 {
            self.window_radius = (self.window_radius as i32 + zoom) as usize
        }
    }

    pub fn from_server_response(inp: &str) -> ClientState {
        utils::set_panic_hook();
        match deser(inp) {
            Ok(SmsbrState(v)) => match from_value (v) {
                Ok((unit, state)) => {
                    match deserialize_state (state) {
                        Ok((world, resources)) => {
                            let mut query = <(&Unit, &Position)>::query();
                            let my_pos = *query.iter(&world)
                                .filter(|(unit_, _)| **unit_ == unit)
                                .next().unwrap().1;
                            ClientState {
                                unit: unit,
                                world: world,
                                resources: resources,
                                view_point: my_pos,
                                window_radius: 5,
                            }
                        },
                        Err (err) => panic!(err), // FIXME disconnect
                    }
                },
                Err (err) => panic!(err), // FIXME disconnect
            }
            ,
            Err(err) => panic!(err), // FIXME disconnect
            Ok(err) => panic!(err), // FIXME disconnect
        }
    }

    pub fn render(&self) -> String {
        display_world_segment(&self.world, &self.resources, self.view_point, self.window_radius)
    }

    // Eval message from server
    pub fn eval_message(&mut self, inp: &str) -> String {
        match deser(inp) {
            Ok(SmsbrEvent(e)) => match e {
                Event::Step {unit, direction} => {
                    eval_event(
                        &mut self.world,
                        &mut self.resources,
                        e
                    );
                    if unit.clone() == self.unit {
                        self.view_point = eval_direction(
                            self.view_point,
                            direction
                        )
                    };
                    return "".to_string()
                },
                _ =>  {
                    eval_event(
                        &mut self.world,
                        &mut self.resources,
                        e
                    );
                    return "".to_string()
                }
            },
            _ => return "".to_string(),
        }
    }
}
