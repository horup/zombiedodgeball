use cgmath::Vector2;
use crate::{ClientData, systems::movement, state::{Actor,  Player, Sprite, State}};

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
                match self.current.entities.clone().iter().find(|(id, e)| 
                {
                    if let Some(p) = e.player {
                        return client_id == p.client_id;
                    }
                 
                    false
                })
                {
                    None => {
                        let (id, e) = self.current.entities.new_entity_replicated().expect("could not spawn entity");
                        e.pos = Vector2::new(10.0, 10.0);
                        e.actor = Some(Actor {
                            speed:1.0,
                            ..Actor::default()
                        });
                        e.player = Some(Player {client_id:client_id});
                        e.sprite = Some(Sprite::default());
                        println!("spawning player entity {:?}", id);
                    },
                    Some((id, player_entity)) => {
                        if let Some(actor) = player_entity.actor {
                            if actor.cooldown <= 0.0 {
                                let (id, dodge_ball_entity) = self.current.entities.new_entity_replicated().expect("could not spawn ball");
                                dodge_ball_entity.pos = player_entity.pos;
                                dodge_ball_entity.vel = Vector2::new(1.0, 1.0);
                                dodge_ball_entity.sprite = Some(Sprite {
                                    x:2.0,
                                    ..Sprite::default()
                                })
                            }
                        }
                    }
                }
            }

            for (_, e) in self.current.entities.iter_mut() {
                if let Some(p) = e.player {
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
            e.actor = Some(Actor {
                speed:1.0,
                ..Actor::default()
            });
        }
        self.iterations += 1;
        self.update_client_data(client_data);

        movement(&mut self.current, delta);

        self.current.clone()
    }
    
    pub fn restart(&mut self) {
        self.current.entities.clear();
    }
}