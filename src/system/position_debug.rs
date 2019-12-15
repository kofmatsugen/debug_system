use amethyst::{
    core::{
        math::Point3,
        transform::{Parent, Transform},
    },
    ecs::prelude::ComponentEvent,
    ecs::*,
    renderer::{debug_drawing::DebugLinesComponent, palette::rgb::Srgba, ActiveCamera},
};

pub struct PositionDrawSystem {
    reader: Option<ReaderId<ComponentEvent>>,
    updated: BitSet,
}

impl PositionDrawSystem {
    pub fn new() -> Self {
        PositionDrawSystem {
            reader: None,
            updated: BitSet::default(),
        }
    }
}

impl<'s> System<'s> for PositionDrawSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, ActiveCamera>,
        ReadStorage<'s, Parent>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, DebugLinesComponent>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (entities, camera, parents, mut transforms, mut debugs) = data;
        let camera_z = camera
            .entity
            .and_then(|entity| transforms.get(entity))
            .map(|transform| transform.translation().z - 1.);
        if camera_z.is_none() == true {
            return;
        }
        let position_z = camera_z.unwrap();
        if self.reader.is_none() {
            self.reader = transforms.register_reader().into();
        }

        self.updated.clear();
        for event in transforms.channel().read(self.reader.as_mut().unwrap()) {
            match event {
                ComponentEvent::Modified(id) => {
                    self.updated.add(*id);
                }
                ComponentEvent::Inserted(id) => {
                    self.updated.add(*id);
                }
                _ => {}
            }
        }

        for (_, e, _, transform) in (&self.updated, &*entities, !&parents, &transforms).join() {
            let color = Srgba::from_components((1., 0., 0., 1.));

            match debugs.get_mut(e) {
                Some(lines) => {
                    lines.clear();
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
                        position_z,
                    );

                    let down = Point3::new(
                        transform.translation().x,
                        transform.translation().y + 10.0f32,
                        position_z,
                    );
                    lines.add_line(top, down, color);
                }
                None => {
                    let mut lines = DebugLinesComponent::new();
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
                    let _ = debugs.insert(e, lines);
                }
            }
        }
    }
}
