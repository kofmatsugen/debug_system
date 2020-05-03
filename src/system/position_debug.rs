use amethyst::{
    core::{
        math::Point3,
        transform::{Parent, Transform},
    },
    ecs::{Builder, Entity, Join, Read, ReadStorage, System, World, WorldExt, WriteStorage},
    renderer::{debug_drawing::DebugLinesComponent, palette::rgb::Srgba, ActiveCamera},
};

pub struct PositionDrawSystem {
    positions: Entity,
}

impl PositionDrawSystem {
    pub fn new(world: &mut World) -> Self {
        PositionDrawSystem {
            positions: world.create_entity().build(),
        }
    }
}

impl<'s> System<'s> for PositionDrawSystem {
    type SystemData = (
        Read<'s, ActiveCamera>,
        ReadStorage<'s, Parent>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, DebugLinesComponent>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (camera, parents, transforms, mut debugs) = data;
        let camera_z = camera
            .entity
            .and_then(|entity| transforms.get(entity))
            .map(|transform| transform.translation().z - 1.);
        if camera_z.is_none() == true {
            return;
        }
        let position_z = camera_z.unwrap();

        match update_position(
            self.positions,
            &parents,
            &transforms,
            &mut debugs,
            position_z,
        ) {
            Ok(_) => {}
            Err(err) => log::error!("error: {:?}", err),
        }
    }
}

fn update_position(
    positions: Entity,
    parents: &ReadStorage<Parent>,
    transforms: &ReadStorage<Transform>,
    debugs: &mut WriteStorage<DebugLinesComponent>,
    position_z: f32,
) -> amethyst::Result<()> {
    let lines = debugs
        .entry(positions)?
        .or_insert(DebugLinesComponent::new());
    lines.clear();
    let color = Srgba::from_components((1., 0., 0., 1.));
    for (_, transform) in (!parents, transforms).join() {
        let left = Point3::new(
            transform.translation().x - 10.0f32,
            transform.translation().y,
            position_z,
        );

        let right = Point3::new(
            transform.translation().x + 10.0f32,
            transform.translation().y,
            position_z,
        );

        lines.add_line(left, right, color);

        let top = Point3::new(
            transform.translation().x,
            transform.translation().y - 10.0f32,
            0.0f32,
        );

        let down = Point3::new(
            transform.translation().x,
            transform.translation().y + 10.0f32,
            position_z,
        );
        lines.add_line(top, down, color);
    }
    Ok(())
}
