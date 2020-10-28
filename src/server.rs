use cgmath::Vector2;
use crate::{ClientData, state::{Actor, Player, State}};

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
            if cd.shoot
            {
                let client_id = cd.client_id;
                let mut spawn = true;
                for (_, e) in self.current.entities.iter()
                {
                    if let Actor::Player(p) = e.actor
                    {
                        if p.client_id == client_id
                        {
                            spawn = false;
                        }
                    }
                }

                if spawn
                {
                    let (id, e) = self.current.entities.new_entity_replicated().expect("could not spawn entity");
                    e.pos = Vector2::new(10.0, 10.0);
                    e.actor = Actor::Player(Player {
                        client_id:client_id
                    });
                    println!("spawning player entity {:?}", id);
                }
            }
        }
    }

    pub fn update(&mut self, client_data:&[ClientData]) -> State
    {
        self.update_client_data(client_data);
        self.current.clone()
    }
    
    pub fn restart(&mut self) {
        self.current.entities.clear();
        /*let (id, e) = self.current.entities.new_entity_replicated().expect("could not create entity");
        e.pos = Vector2::new(10.0, 10.0);
        e.actor = Actor::Player(Player {
            player_id:0
        });*/
    }
}