use crate::utils::build_client;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use structopt::StructOpt;

pub mod middleware;
pub mod types;
use middleware::Middleware;

type EventHandler = fn() -> ();

#[derive(Hash, PartialEq, Eq)]
pub enum EventType {
    SignerLack,
    TestEvent,
    RpcTimeout
}

#[derive(Clone)]
pub struct EventManager {
    handlers: Arc<RwLock<HashMap<EventType, Vec<EventHandler>>>>,
}

impl EventManager {
    pub fn on(&mut self, e_type: EventType, handler: EventHandler) {
        let mut map = self.handlers.write().expect("map poisoned.");
        match map.get_mut(&e_type) {
            Some(v) => {
                v.push(handler);
            }
            _ => {
                map.insert(e_type, vec![handler]);
            }
        }
    }

    pub fn emit(&self, e_type: EventType) {
        let hmap = self.handlers.read().unwrap();
        if let Some(v) = hmap.get(&e_type) {
            v.iter().for_each(|f| f());
        }
    }
}

#[derive(Debug, StructOpt, Clone)]
pub struct MonitorOption {
    #[structopt(long, default_value = "ws://127.0.0.1:8087")]
    node_url: String,
    #[structopt(long, default_value = "2000")]
    timeout: u32,
}

#[derive(Clone)]
pub struct Monitor {
    option: MonitorOption,
    middlewares: Arc<RwLock<Vec<Middleware>>>,
    pub em: EventManager,
}

impl Monitor {
    pub fn new() -> Monitor {
        let option = MonitorOption::from_args();
        let em = EventManager {
            handlers: Arc::new(RwLock::new(HashMap::default())),
        };
        Monitor {
            option,
            middlewares: Arc::new(RwLock::new(vec![])),
            em,
        }
    }

    pub fn on(&mut self, ev: EventType, handler: EventHandler) -> &mut Monitor {
        self.em.on(ev, handler);
        self
    }

    pub fn emit(&self, ev: EventType) -> () {
        self.em.emit(ev);
    }

    pub fn chain(&mut self, middleware: Middleware) -> &mut Monitor {
        {
            let mut middlewares = self.middlewares.write().unwrap();
            middlewares.push(middleware);
        }
        self
    }

    #[allow(irrefutable_let_patterns)]
    pub async fn run(&self) {
        loop  {
            let url = self.option.node_url.clone();
            match build_client(url).await {
                Ok(client) => {
                    let mut sub = client.subscribe_blocks().await.unwrap();
                    while let block = sub.next().await {
                        let middlewares = self.middlewares.read().unwrap();
                        middlewares.iter().for_each(|f| f(client.clone(), self.clone(), block.clone()));
                    }
                }
                _ => {
                    self.emit(EventType::RpcTimeout);
                },
            }
            log::info!("No responding from server. Retry scheduled after 3s...");
            std::thread::sleep(std::time::Duration::from_millis(3000));
        }
    }
}
