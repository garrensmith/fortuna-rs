
use crossbeam::crossbeam_channel::{select, unbounded as cross_unbounded, Receiver as CrossReceiver, Sender as CrossSender};

// use tokio::sync::mpsc as tokio_mpsc;
// use tokio::sync::mpsc::{unbounded_channel as tokio_channel, UnboundedSender as TokioSender, UnboundedReceiver as TokioReceiver};
use std::thread;
use crate::{js_engine, FortunaIsolate};
use std::sync::Arc;

type ServerTx = CrossSender<String>;
type ServerRx = CrossReceiver<Command>;

type ClientTx = CrossSender<Command>;
type ClientRx = CrossReceiver<String>;

pub enum Ops {
    REWRITE,
    EVAL,
    CALL,
    EXIT
}

pub struct Command {
    pub operation: Ops,
    pub payload: String,
    pub args: Vec<String>
}


struct JSServer {
    send: ServerTx,
    receive: ServerRx,
    isolate: FortunaIsolate
}

impl JSServer {

    fn start (send: ServerTx, receive: ServerRx) {
        thread::spawn(move || {
            let mut i = 0;
            let mut server = JSServer {
                receive,
                send,
                isolate: FortunaIsolate::new_clean()
            };

            loop {
                select! {
                    recv(server.receive) -> sent => {
                        match sent {
                            Ok(cmd) => {
                                if !server.process(cmd) {
                                    println!("exiting");
                                    break;
                                }
                            },
                            Err(RecvError) => {
                                println!("exiting RecvError");
                                break;
                            }
                            Err(error) => {
                                println!("Error {:?}", error)
                            }
                        }
                    }
                }
            }
        });
    }

    fn process(&mut self, cmd: Command) -> bool {
        match cmd.operation {
            Ops::EXIT => false,
            Ops::EVAL => {
                self.eval(cmd.payload);
                true
            },
            Ops::CALL => {
                self.call(cmd.payload, cmd.args.as_slice());
                true
            },
            Ops::REWRITE => {
                self.call(cmd.payload, cmd.args.as_slice());
                true
            }
            _ => {
                println!("unsupported cmd");
                false
            }
        }
    }

    fn eval(&mut self, script: String) {
        let resp = self.isolate.eval(script.as_str(), &[]);
        self.send.send(resp).unwrap();
    }

    fn call(&mut self, fun_name: String, args: &[String]) {
        let resp = self.isolate.call(fun_name.as_str(), args);
        self.send.send(resp).unwrap();
    }
}


#[derive(Clone)]
pub struct JSClient {
    pub tx: ClientTx,
    pub rx: ClientRx
}

impl JSClient {

    pub fn run(&self, cmd: Command) -> String {
        self.tx.send(cmd);
        self.rx.recv().unwrap()
    }
}

// impl Drop for JSClient {
//     fn drop(&mut self) {
//         // let _exit = Command {
//         //     operation: Ops::EXIT,
//         //     payload: "exit".to_string()
//         // };
//         // self.tx.send(exit).unwrap();
//         println!("> Dropping client");
//     }
// }

pub fn create_js_env() -> JSClient {
    // let (async_tx, mut async_rx) = tokio_channel::<String>();
    let (tx1, rx1) = cross_unbounded::<Command>();
    let (tx2, rx2) = cross_unbounded::<String>();

    let client = JSClient {
        tx: tx1,
        rx: rx2
    };

    JSServer::start(tx2, rx1);

    // tx1.send("Hello Server".to_string());
    client
}

