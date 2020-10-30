use cgmath::Vector2;
use crate::{ClientData, state::{Actor, Common, Player, Sprite, State, Zombie}};

pub struct Server {
    pub current:State,
    pub iterations:i32
}

impl Server {
    pub fn new() -> Self
    {
        Self {
            current:State::new(),
            iterations:0
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
                        e.sprite = Some(Sprite::default());
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
        if self.iterations % 20 == 0 {
            let (id, e) = self.current.entities.new_entity_replicated().expect("could not create entity");
            e.pos = Vector2::new(rand::random::<f32>() * 20.0, rand::random::<f32>() * 20.0);
            e.sprite = Some(Sprite {
                x:1.0,
                ..Sprite::default()
            });
            e.actor = Actor::Zombie(Common::default(), Zombie {});
        }
        self.iterations += 1;
        self.update_client_data(client_data);
        self.current.clone()
    }
    
    pub fn restart(&mut self) {
        self.current.entities.clear();
    }
}