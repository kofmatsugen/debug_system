use crate::{
    resource::DebugFont,
    system::{
        debug_info_disp::DebugInfomationDisplaySystem, entity_count::EntityCountSystem,
        fps_disp::FpsDispSystem, position_debug::PositionDrawSystem,
    },
    traits::DebugDisplayFormat,
};
use amethyst::{
    core::SystemBundle,
    ecs::{DispatcherBuilder, World},
};
use std::marker::PhantomData;

pub struct DebugSystemBundle<D> {
    display_info: PhantomData<D>,
}

impl<D> DebugSystemBundle<D> {
    pub fn new() -> Self {
        DebugSystemBundle {
            display_info: PhantomData,
        }
    }
}

impl<'a, 'b, D> SystemBundle<'a, 'b> for DebugSystemBundle<D>
where
    D: for<'c> DebugDisplayFormat<'c>,
{
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder,
    ) -> Result<(), amethyst::Error> {
        let system_font = DebugFont::insert_world(world);

        builder.add(EntityCountSystem::new(), "entity_count_system", &[]);
        builder.add(FpsDispSystem::new(world), "fps_disp_system", &[]);
        builder.add(PositionDrawSystem::new(world), "position_draw_system", &[]);
        builder.add(
            DebugInfomationDisplaySystem::<D>::new(system_font.clone()),
            "debug_info_disp",
            &[],
        );

        Ok(())
    }
}
