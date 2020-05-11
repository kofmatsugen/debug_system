use amethyst::{
    core::{math::Point3, Time, Transform},
    ecs::{Builder, Entity, Read, ReadExpect, ReadStorage, System, World, WorldExt, WriteStorage},
    renderer::{debug_drawing::DebugLinesComponent, palette::rgb::Srgba, ActiveCamera},
    ui::{UiFinder, UiText},
    utils::{circular_buffer::CircularBuffer, fps_counter::FpsCounter},
    window::ScreenDimensions,
};

pub struct FpsDispSystem {
    fps_disp_ui: Option<Entity>,
    fps_graph: Entity,
    fps_buffer: CircularBuffer<f32>,
}

impl FpsDispSystem {
    pub fn new(world: &mut World) -> Self {
        FpsDispSystem {
            fps_disp_ui: None,
            fps_graph: world.create_entity().build(),
            fps_buffer: CircularBuffer::new(120),
        }
    }
}

impl<'s> System<'s> for FpsDispSystem {
    type SystemData = (
        Read<'s, ActiveCamera>,
        WriteStorage<'s, UiText>,
        Read<'s, FpsCounter>,
        UiFinder<'s>,
        WriteStorage<'s, DebugLinesComponent>,
        ReadStorage<'s, Transform>,
        ReadExpect<'s, ScreenDimensions>,
        ReadExpect<'s, Time>,
    );

    fn run(
        &mut self,
        (camera, mut texts, fps, finder, mut debug_lines, transforms, dim, time): Self::SystemData,
    ) {
        self.update_ui(&mut texts, &fps, &time, &finder);
        self.fps_buffer.push(fps.frame_fps());

        let camera_z = camera
            .entity
            .and_then(|entity| transforms.get(entity))
            .map(|transform| transform.translation().z - 1.);
        if camera_z.is_none() == true {
            return;
        }
        let position_z = camera_z.unwrap();
        match update_graph(
            self.fps_graph,
            &mut debug_lines,
            position_z,
            &dim,
            &self.fps_buffer,
        ) {
            _ => {}
        }
    }
}

impl FpsDispSystem {
    fn update_ui(
        &mut self,
        texts: &mut WriteStorage<UiText>,
        fps: &FpsCounter,
        time: &Time,
        finder: &UiFinder,
    ) -> Option<()> {
        if self.fps_disp_ui.is_none() {
            self.fps_disp_ui = finder.find("debug_fps_sample");
        }
        let ui = self.fps_disp_ui?;

        let text_ui = texts.get_mut(ui)?;
        text_ui.text = format!(
            "fps: {:.2} (delta: {:.4} s)",
            fps.frame_fps(),
            time.delta_seconds()
        );

        Some(())
    }
}

fn update_graph(
    graph: Entity,
    debugs: &mut WriteStorage<DebugLinesComponent>,
    position_z: f32,
    dimension: &ScreenDimensions,
    fps_buffer: &CircularBuffer<f32>,
) -> amethyst::Result<()> {
    let lines = debugs.entry(graph)?.or_insert(DebugLinesComponent::new());

    lines.clear();
    let color = Srgba::from_components((0., 1., 0., 1.));

    let base_x = -dimension.width() / 2. + 10.;
    let base_y = -dimension.height() / 2. + 10.;

    let fps = fps_buffer.queue().iter().collect::<Vec<_>>();

    for i in 0..fps.len() - 1 {
        let left = i;
        let right = i + 1;

        let left_x = base_x + left as f32 * 10.;
        let right_x = base_x + right as f32 * 10.;
        let left_y = base_y + fps[left];
        let right_y = base_y + fps[right];

        let left = Point3::new(left_x, left_y, position_z);
        let right = Point3::new(right_x, right_y, position_z);

        lines.add_line(left, right, color);
    }

    Ok(())
}
