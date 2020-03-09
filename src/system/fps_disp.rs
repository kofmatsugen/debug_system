use amethyst::{
    ecs::*,
    ui::{UiFinder, UiText},
    utils::fps_counter::FpsCounter,
};

pub struct FpsDispSystem {
    fps_disp_ui: Option<Entity>,
}

impl FpsDispSystem {
    pub fn new() -> Self {
        FpsDispSystem { fps_disp_ui: None }
    }
}

impl<'s> System<'s> for FpsDispSystem {
    type SystemData = (WriteStorage<'s, UiText>, Read<'s, FpsCounter>, UiFinder<'s>);

    fn run(&mut self, data: Self::SystemData) {
        self.update_ui(data);
    }
}

impl FpsDispSystem {
    fn update_ui(&mut self, data: <FpsDispSystem as System>::SystemData) -> Option<()> {
        let (mut texts, fps, finder) = data;

        if self.fps_disp_ui.is_none() {
            self.fps_disp_ui = finder.find("debug_fps_sample");
        }
        let ui = self.fps_disp_ui?;

        let text_ui = texts.get_mut(ui)?;
        text_ui.text = format!("fps: {:.2}", fps.sampled_fps());

        Some(())
    }
}
