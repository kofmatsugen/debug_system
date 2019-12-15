use crate::resource::DebugFont;
use amethyst::{
    ecs::*,
    ui::{Anchor, UiText, UiTransform},
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
    type SystemData = (
        ReadExpect<'s, DebugFont>,
        WriteStorage<'s, UiTransform>,
        WriteStorage<'s, UiText>,
        Read<'s, FpsCounter>,
        Entities<'s>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (debug_font, mut transforms, mut texts, fps, entities) = data;

        let system_font = debug_font.system_font.clone();
        let count_text = format!("fps: {:.2}", fps.sampled_fps());

        if let Some(ui) = &self.fps_disp_ui {
            let text = texts.get_mut(*ui).unwrap();
            text.text = count_text;
        } else {
            let entity = entities.create();
            let transform = UiTransform::new(
                "fps_disp".to_string(),
                Anchor::TopLeft,
                Anchor::TopLeft,
                0.,
                -32.,
                0.,
                200.,
                16.,
            );
            let text = UiText::new(system_font, count_text, [0., 0., 0., 1.], 16.);

            let _transform = transforms.insert(entity, transform);
            let _text = texts.insert(entity, text);
            self.fps_disp_ui = Some(entity);
        }
    }
}
