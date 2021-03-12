mod utils;
extern crate web_sys;

use std::cmp::{max};

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

#[wasm_bindgen]
impl ClientState {
    // Commands from JS to webasm.
    // Webasm recieves comand from js io, evaluates
    // then tell what JS should send to server.

    pub fn handle_client_input(
        &mut self,
        evt_raw: &str,
    ) -> String {
        match deser(evt_raw) {
            Ok (ClientInput::MouseDown {position}) => {
                let abs_position = self.pos_relative2absolute (position);
                ser (
                    SmsbrIntention (
                        Intention::GoToPosition {
                            unit: self.unit,
                            position: abs_position,
                        }
                    )
                )
            },
            Ok (
                ClientInput::Wheel {
                    position,
                    delta,
                }
            ) => {
                self.zoom(delta);
                "".to_string()
            },
            _ => "".to_string(),
        }
    }

    fn pos_relative2absolute(
        &self,
        rel_pos: Position,
    ) -> Position {
        let top = max(0, self.view_point.y as i32 - self.window_radius as i32);
        let left = max(0, self.view_point.x as i32 - self.window_radius as i32);
        Position {
            x: left as usize + rel_pos.x,
            y: top as usize + rel_pos.y,
        }
    }

    fn zoom(&mut self, zoom: i32) {
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
                        if let Some (new_view_point) = eval_direction(
                            self.view_point,
                            direction
                        ) {
                            self.view_point = new_view_point;
                        }
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

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum ClientInput {
    MouseUp {position: Position},
    MouseDown {position: Position},
    Wheel {position: Position, delta: i32},
    KeyDown {key_code: usize},
    KeyUp {key_code: usize},
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


