use cgmath::Vector2;
use crate::{ClientData, state::{Actor, Common, Player, State}};

pub struct Server {
    pub current:State
}

impl Server {
    pub fn new() -> Self
    {
        Self {
            current:State::new()
        }
    }

    pub fn update_client_data(&mut self, client_data:&[ClientData])
    {
        for cd in client_data
        {
            let client_id = cd.client_id;

            if cd.shoot
            {
                match self.current.entities.iter().find(|(id, e)| 
                {
                    if let Actor::Player(c, p) = e.actor {
                        return client_id == p.client_id;
                    }

                    false
                })
                {
                    None => {
                        let (id, e) = self.current.entities.new_entity_replicated().expect("could not spawn entity");
                        e.pos = Vector2::new(10.0, 10.0);
                        e.actor = Actor::Player(Common::default(), Player {
                            client_id:client_id
                        });
                        println!("spawning player entity {:?}", id);
                    },
                    _ => {}
                }
            }

            for (_, e) in self.current.entities.iter_mut() {
                if let Actor::Player(c, p) = e.actor
                {
                    if p.client_id == client_id
                    {
                        e.pos += cd.vel;
                    }
                }
            }
        }
    }

    pub fn update(&mut self, delta:f32, client_data:&[ClientData]) -> State
    {
        self.update_client_data(client_data);
        self.current.clone()
    }
    
    pub fn restart(&mut self) {
        self.current.entities.clear();
    }
}