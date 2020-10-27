use std::{thread::{self, JoinHandle, Thread}, time::Duration, sync::mpsc};

use crate::state::State;

pub struct Server {
    pub state:State
}

impl Server {
    pub fn start(rx:mpsc::Receiver<()>, tx:mpsc::Sender<()>) -> JoinHandle<()>
    {
        let server_thread = thread::spawn(move || {
            let mut server = Server {
                state:State::new()
            };

            loop {
                server.state.update();
                thread::sleep(Duration::from_millis(100));

                // if signal received, exit the loop
                match rx.try_recv() {
                    Ok(_)=>{break;}
                    _=>{  }
                }
            }
        });

        server_thread
    }

    
}