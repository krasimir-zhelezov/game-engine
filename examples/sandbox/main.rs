use skalora_game_engine::{
   engine::Skalora,
   components::{
       renderable::{Color, Renderable},
       transform::{Position, Scale, Transform},
   },
};

fn main() {
   let mut engine = Skalora::new();
   let _camera = engine.world.spawn_camera();
   let player = engine.world.entity_manager.create_entity();
   engine.world.components.add_component(player, Transform {
       position: Position { x: 0.0, y: 0.0 },
       scale: Scale { x: 1.0, y: 1.0 },
       rotation: 0.0,
   });
   engine.world.components.add_component(player,
       Renderable::new_rectangle(Color::from_rgba8(255.0, 0.0, 0.0, 255.0))
   );
   engine.run();
}