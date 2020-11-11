use legion::*;
use uuid::{
    Uuid
};
use std::{
    sync::{
        mpsc,
        mpsc::{SyncSender, Receiver},
        Arc,
        RwLock,
    },
    collections::HashMap,
    process::exit,
    net::{SocketAddr, TcpStream},
    thread::spawn,
};

use websocket::{
    OwnedMessage,
    sender::{Writer},
    sync::Server
};

use clap::{Arg, App};
use samosbor_core::{
    eval_event,
    block_tiles_from_location,
    ClientId,
    location::{
        Unit,
        Location,
    },
    protocol::{
        SamosborMessage,
        SamosborMessage::{
            SmsbrIntention,
        },
        Event,
        Event::{ClientConnect, ClientDisconnect},
        Intention,
    },
};
use serde_json;

pub struct Client {
    pub unit: Unit,
    pub sender: Writer<TcpStream>,
}

impl Client {
    pub fn send(&mut self, msg: SamosborMessage) {
        let _ = self.sender.send_message(
            &OwnedMessage::Text(serde_json::json!(msg).to_string())
        );
    }
}

type Clients = HashMap<ClientId, Client>;

type ClientsAsync = Arc<RwLock<Clients>>;


fn send_target_message(
    client_id: ClientId,
    msg: SamosborMessage,
    clients_async: &ClientsAsync,
){
    match clients_async.write().unwrap().get_mut(&client_id) {
        Some (client) => client.send(msg),
        None => eprintln!("No client with id {:#?}", client_id),
    }
}

fn send_broadcast_message (
    client_id_except: Option<ClientId>,
    msg: SamosborMessage,
    clients_async: &ClientsAsync,
) {
    let mut clients = clients_async.write().unwrap();
    match client_id_except {
        Some(client_id) => clients.iter_mut()
            .filter(
                |(client_id_, _)| **client_id_ != client_id
            ).for_each(
                move |(_, client)| client.send(msg.clone())
            ),
        None => clients.values_mut().for_each(
            move |client| client.send(msg.clone())
        ),
    }
}

fn intention_to_event(intention:Intention) -> Event {
    match intention {
        Intention::Step {unit, direction} => Event::Step{unit, direction},
    }
}
fn main () {
    // Command line args
    let matches = App::new("Samosbor server")
        .author("Tempe team <https://github.com/tempe-team>")
        .about("Server for samosbor game")
        .arg(Arg::with_name("port")
             .short("p")
             .long("port")
             .takes_value(true)
             .required(true)
             .default_value("8000")
             .help("TCP port on which server will listen connections"))
        .arg(Arg::with_name("host")
             .short("h")
             .long("host")
             .takes_value(true)
             .required(true)
             .default_value("127.0.0.1")
             .help("On which address lister. Typically it is either 127.0.0.1 or 0.0.0.0 (for docker runs)"))
        .get_matches();
    let (host, port) = match (matches.value_of("host"), matches.value_of("port")) {
        (Some(host ), Some(port)) => (host, port),
        _ => unreachable!(),
    };

    // Global storage of clients
    let clients_async: ClientsAsync = Arc::new(RwLock::new(Clients::new()));
    let clients_async_global = clients_async.clone();

    // Reciever + sender for global event loop
    let (global_tx, global_rc): (SyncSender<(ClientId, Event)>, Receiver<(ClientId, Event)>) = mpsc::sync_channel(8); // 8 is buffer size

    // Global event loop
    spawn(move || {
        let mut world = World::default();
        let mut resources = Resources::default();
        resources.insert(Location::new(64, 32));
        block_tiles_from_location(&mut world, &resources);
        loop {
            if let Ok((client_id, evt)) = global_rc.recv() {
                println!("Recieved message {:?} from {:?}", evt, client_id);
                match eval_event(
                    &mut world,
                    &mut resources,
                    evt,
                ) {
                    (Some(to_target), Some(to_others)) => {
                        send_target_message(
                            client_id,
                            to_target,
                            &clients_async_global,
                        );
                        send_broadcast_message(
                            Some (client_id),
                            to_others,
                            &clients_async_global,
                        );
                    },
                    (None, Some(to_others)) => send_broadcast_message(
                        Some (client_id),
                        to_others,
                        &clients_async_global,
                    ),
                    (Some(to_target), None) => send_target_message(
                        client_id,
                        to_target,
                        &clients_async_global,
                    ),
                    (None, None) => (),
                }
            }
        }
    });

    let mb_socket_addr:Result<SocketAddr, _> = format!("{}:{}", host, port).parse();

    match mb_socket_addr {
        Ok(socket_addr) => {
            let server = Server::bind(socket_addr).unwrap();
            for request in server.filter_map(Result::ok) {
                let global_tx_n = global_tx.clone();
                let clients_async_n = clients_async.clone();
		            // Spawn a new thread for each connection.
		            spawn(move || {
			              let client = request
                        .accept()
                        .unwrap();
                    let ip = client.peer_addr().unwrap();
			              println!("Connection from {}", ip);
			              let (mut receiver, sender) = client.split().unwrap();

                    let unit = Unit (Uuid::new_v4());
                    let client_id = ClientId::new(Uuid::new_v4());
                    let client_internal = Client {
                        unit: unit.clone(),
                        sender: sender,
                    };
                    {
                        let mut clients = clients_async_n.write().unwrap ();
                        clients.insert(
                            client_id,
                            client_internal,
                        )
                    };
                    let _ = global_tx_n.send((
                        client_id,
                        ClientConnect(unit.clone())),
                    );
			              for message in receiver.incoming_messages() {
				                let message = message;
				                match message {
					                  Ok (OwnedMessage::Close(_)) => {
						                    println!("Client {} disconnected", ip);
                                let _ = global_tx_n.send((
                                    client_id,
                                    ClientDisconnect(unit.clone())
                                ));
						                    return;
					                  }
					                  Ok (OwnedMessage::Text(val)) => match serde_json::from_str(&val) {
                                Ok(SmsbrIntention(intent)) =>
                                    global_tx_n.send((
                                        client_id, intention_to_event(intent)
                                    )).unwrap(),
                                Ok(unexpected) => println!("Unexpected(but succesfully parsed) message type from client  {:?}", unexpected),
                                Err (err) => eprintln!("error on client: {:#?}", err),
                            },
                            unexpected => println!("Unexpected message type from client {:?}", unexpected)
				                }
			              };
                    let _ = global_tx_n.send((
                        client_id,
                        ClientDisconnect(unit.clone()),
                    ));
		            });
	          };
        },
        Err(err) => {
            eprintln!("can not build socket address from provided arguments: {:#?}", err);
            exit(2)
        },
    }
}
