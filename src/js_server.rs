use crossbeam::crossbeam_channel::{
    select, unbounded as cross_unbounded, Receiver as CrossReceiver, RecvError,
    Sender as CrossSender,
};

use crate::{FortunaIsolate, JSEnv};
use std::fmt::Debug;
use std::thread;

type ServerTx = CrossSender<String>;
type ServerRx = CrossReceiver<Command>;

type ClientTx = CrossSender<Command>;
type ClientRx = CrossReceiver<String>;

#[derive(Debug)]
pub enum Ops {
    REWRITE,
    EVAL,
    CALL,
    EXIT,
}

#[derive(Debug)]
pub struct Command {
    pub operation: Ops,
    pub payload: String,
    pub args: Vec<String>,
}

struct JSServer {
    send: ServerTx,
    receive: ServerRx,
    isolate: FortunaIsolate,
}

impl JSServer {
    fn start(js_env: &JSEnv, send: ServerTx, receive: ServerRx) {
        let data = js_env.startup_data.clone();
        thread::spawn(move || {
            let mut server = JSServer {
                receive,
                send,
                isolate: FortunaIsolate::new_from_snapshot(data.as_slice()),
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
            }
            Ops::CALL => {
                self.call(cmd.payload, cmd.args.as_slice());
                true
            }
            Ops::REWRITE => {
                self.call(cmd.payload, cmd.args.as_slice());
                true
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
    pub rx: ClientRx,
}

impl JSClient {
    pub fn run(&self, cmd: Command) -> String {
        self.tx.send(cmd).unwrap();
        self.rx.recv().unwrap()
    }
}

pub fn create_js_env(js_env: &JSEnv) -> JSClient {
    let (tx1, rx1) = cross_unbounded::<Command>();
    let (tx2, rx2) = cross_unbounded::<String>();

    let client = JSClient { tx: tx1, rx: rx2 };

    JSServer::start(js_env, tx2, rx1);

    client
}
