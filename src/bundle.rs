use crate::{
    resource::DebugFont,
    system::{
        entity_count::EntityCountSystem, fps_disp::FpsDispSystem,
        position_debug::PositionDrawSystem,
    },
};
use amethyst::{
    core::SystemBundle,
    ecs::{DispatcherBuilder, World},
};

pub struct DebugSystemBundle;

impl DebugSystemBundle {
    pub fn new() -> Self {
        DebugSystemBundle
    }
}

impl<'a, 'b> SystemBundle<'a, 'b> for DebugSystemBundle {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder,
    ) -> Result<(), amethyst::Error> {
        DebugFont::insert_world(world);

        builder.add(EntityCountSystem::new(), "entity_count_system", &[]);
        builder.add(FpsDispSystem::new(), "fps_disp_system", &[]);
        builder.add(PositionDrawSystem::new(), "position_draw_system", &[]);

        Ok(())
    }
}
