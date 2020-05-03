use crate::{
    resource::DebugFont,
    system::{
        debug_info_disp::DebugInfomationDisplaySystem, entity_count::EntityCountSystem,
        fps_disp::FpsDispSystem, position_debug::PositionDrawSystem,
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
        let system_font = DebugFont::insert_world(world);

        builder.add(EntityCountSystem::new(), "entity_count_system", &[]);
        builder.add(FpsDispSystem::new(), "fps_disp_system", &[]);
        builder.add(PositionDrawSystem::new(world), "position_draw_system", &[]);
        builder.add(
            DebugInfomationDisplaySystem::<()>::new(system_font.clone()),
            "debug_info_disp",
            &[],
        );

        Ok(())
    }
}
