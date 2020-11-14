use crate::{event::Event, state::World};

pub struct Server {
    pub current:World,
    pub iterations:i32
}

impl Server {
    pub fn new() -> Self
    {
        Self {
            current:World::new(),
            iterations:0
        }
    }

    pub fn update(&mut self, delta:f32, external_events:&[Event]) -> World
    {
        let mut events = Vec::new();
        self.iterations += 1;
        events.push(Event::Tick(self.iterations, delta));
        for e in external_events {
            events.push(*e);
        }
    
        super::systems::step(&mut self.current, true, &mut events);
        self.current.clone()
      /*  systems::cleanup::cleanup(&mut self.current);
        
        let mut wish_to_join:Vec<u128> = client_data.into_iter().map(|x| {
            if x.shoot {
                return x.client_id;
            }

            return 0;
        }).collect();
        wish_to_join.retain(|x| *x != 0);
        
        systems::spawn::spawn(&mut self.current, delta, self.iterations, &wish_to_join);
        
        //update::update_clients(&mut self.current, delta, client_data);

        
        let collisions = update::update_movement(&mut self.current, delta);
        update::update_dodge_ball(&mut self.current, delta, &collisions);
        update::update_actors(&mut self.current, delta);

        self.iterations += 1;
        self.current.clone()*/
    }
    
    pub fn restart(&mut self) {
        self.current.entities.clear();
    }
}