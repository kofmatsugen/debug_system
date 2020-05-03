use amethyst::ecs::{Component, NullStorage};
use std::marker::PhantomData;

// デバッグ表示を行うためのマーカーコンポーネント
#[derive(Default)]
pub struct DebugInfomationDisplay<T> {
    marker: PhantomData<T>,
}

impl<T> DebugInfomationDisplay<T> {
    pub fn new() -> Self {
        DebugInfomationDisplay {
            marker: PhantomData,
        }
    }
}

impl<T> Component for DebugInfomationDisplay<T>
where
    T: 'static + Send + Sync + Default,
{
    type Storage = NullStorage<Self>;
}
