use bevy::{color::palettes::tailwind::*, prelude::*, utils::hashbrown::HashMap};
use generational_arena::{Arena, Index};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ToastConfig::default()).add_systems(
            Update,
            (
                toast_background_opacity,
                toast_text_opacity,
                handle_toast_fades,
            ),
        );
    }
}

#[derive(Resource)]
pub(crate) struct ToastConfig {
    pub(crate) arena: Arena<u16>,
    pub(crate) current_index: u16,
    pub(crate) opacity_map: HashMap<Index, f32>,
    pub(crate) ordering_vec: Vec<Index>,
}

impl ToastConfig {
    pub fn insert(&mut self) -> Index {
        let index = self.arena.insert(self.current_index);
        self.current_index += 1;
        self.opacity_map.insert(index, 0.0);
        self.ordering_vec.push(index);
        index
    }

    pub fn remove(&mut self, index: Index) {
        self.arena.remove(index);
        self.opacity_map.remove(&index);
        if let Some(location) = self.ordering_vec.iter().position(|i| *i == index) {
            self.ordering_vec.remove(location);
        }
    }

    pub fn opacity(&self, index: Index) -> Option<f32> {
        self.opacity_map.get(&index).copied()
    }

    pub fn set_opacity(&mut self, index: Index, value: f32) {
        if let Some(opacity) = self.opacity_map.get_mut(&index) {
            *opacity = value;
        }
    }

    pub fn top(&self, index: Index) -> f32 {
        if let Some(location) = self.ordering_vec.iter().position(|i| *i == index) {
            5.0 + (12.5 * location as f32)
        } else {
            5.0
        }
    }
}

impl Default for ToastConfig {
    fn default() -> Self {
        Self {
            arena: Arena::new(),
            current_index: 0,
            opacity_map: HashMap::new(),
            ordering_vec: Vec::new(),
        }
    }
}

#[derive(Component)]
struct ToastElement(Index);
#[derive(Component)]
struct ToastBody;
#[derive(Component)]
struct ToastHeader;
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum ToastState {
    #[default]
    FadeIn,
    OnScreen,
    FadeOut,
}

#[derive(Component)]
struct ToastContainer {
    fade: f32,
    fade_in_timer: Timer,
    index: Index,
    on_screen_timer: Timer,
    fade_out_timer: Timer,
    state: ToastState,
}

impl ToastContainer {
    fn new(fade: f32, on_screen: f32, index: Index) -> Self {
        Self {
            fade,
            fade_in_timer: Timer::from_seconds(fade, TimerMode::Once),
            index,
            on_screen_timer: Timer::from_seconds(on_screen, TimerMode::Once),
            fade_out_timer: Timer::from_seconds(fade, TimerMode::Once),
            state: ToastState::FadeIn,
        }
    }

    fn tick(&mut self, delta: std::time::Duration) {
        use ToastState as T;
        match self.state {
            T::FadeIn => {
                self.fade_in_timer.tick(delta);
                if self.fade_in_timer.finished() {
                    self.state = T::OnScreen;
                }
            }
            T::OnScreen => {
                self.on_screen_timer.tick(delta);
                if self.on_screen_timer.finished() {
                    self.state = T::FadeOut;
                }
            }
            T::FadeOut => {
                self.fade_out_timer.tick(delta);
            }
        }
    }

    fn finished(&self) -> bool {
        use ToastState as T;
        match self.state {
            T::FadeOut => self.fade_out_timer.finished(),
            _ => false,
        }
    }

    fn tween(&self) -> f32 {
        use ToastState as T;
        match self.state {
            T::FadeIn => self.fade_in_timer.elapsed_secs() / self.fade,
            T::OnScreen => 1.0,
            T::FadeOut => self.fade / self.fade_out_timer.elapsed_secs(),
        }
    }
}

pub struct ToastNotification {
    title: String,
    body: String,
}

impl ToastNotification {
    pub fn new(title: &str, body: &str) -> Self {
        Self {
            title: title.to_string(),
            body: body.to_string(),
        }
    }
}

impl Command for ToastNotification {
    fn apply(self, world: &mut World) {
        let mut toast_arena = world.get_resource_mut::<ToastConfig>().unwrap();
        let index = toast_arena.insert();
        world
            .spawn((
                Name::from("Toast Notification"),
                Node {
                    width: Val::Percent(20.0),
                    height: Val::Percent(10.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    top: Val::Percent(90.0),
                    left: Val::Percent(75.90),
                    overflow: Overflow::clip(),
                    ..default()
                },
                ToastElement(index),
                ToastContainer::new(0.5, 3.0, index),
                BackgroundColor(BLUE_900.into()),
                BorderRadius::all(Val::Px(10.0)),
            ))
            .with_children(|parent| {
                parent
                    .spawn((
                        Node {
                            display: Display::Flex,
                            align_content: AlignContent::Center,
                            justify_content: JustifyContent::Center,
                            width: Val::Percent(100.0),
                            height: Val::Percent(30.0),
                            ..default()
                        },
                        BackgroundColor(BLUE_600.into()),
                        ToastHeader,
                        ToastElement(index),
                    ))
                    .with_children(|p| {
                        p.spawn((
                            Text(self.title),
                            ToastElement(index),
                            BackgroundColor(BLUE_600.into()),
                        ));
                    });

                parent
                    .spawn((
                        Node {
                            display: Display::Flex,
                            align_content: AlignContent::Center,
                            justify_content: JustifyContent::Center,
                            width: Val::Percent(100.0),
                            height: Val::Percent(70.0),
                            ..default()
                        },
                        ToastBody,
                        ToastElement(index),
                        BackgroundColor(BLUE_900.into()),
                    ))
                    .with_children(|p| {
                        p.spawn((
                            Text(self.body),
                            ToastElement(index),
                            BackgroundColor(BLUE_900.into()),
                        ));
                    });
            });
    }
}

fn handle_toast_fades(
    mut commands: Commands,
    time: Res<Time>,
    mut toast_config: ResMut<ToastConfig>,
    mut query: Query<(Entity, &mut ToastContainer, &mut Node)>,
) {
    for (entity, mut toast, mut node) in &mut query {
        let index = toast.index;
        toast.tick(time.delta());
        if toast.finished() {
            toast_config.remove(index);
            commands.entity(entity).despawn_recursive();
        } else {
            toast_config.set_opacity(index, toast.tween());

            let target_value = toast_config.top(index);
            let fade_out_value = -20.0;
            let mut new_value = if let Val::Percent(percent) = node.top {
                percent
            } else {
                target_value
            };

            use ToastState as T;
            match toast.state {
                T::FadeIn => {
                    new_value.smooth_nudge(&target_value, 10.0, time.delta_secs());
                    node.top = Val::Percent(new_value);
                }
                T::OnScreen => {
                    new_value.smooth_nudge(&target_value, 10.0, time.delta_secs());
                    node.top = Val::Percent(new_value);
                }
                T::FadeOut => {
                    new_value.smooth_nudge(&fade_out_value, 10.0, time.delta_secs());
                    node.top = Val::Percent(new_value);
                }
            }
        }
    }
}

fn toast_background_opacity(
    toast_config: Res<ToastConfig>,
    mut query: Query<(&mut BackgroundColor, &ToastElement)>,
) {
    for (mut background_color, toast) in &mut query {
        if let Some(opacity) = toast_config.opacity(toast.0) {
            background_color.0 = background_color.0.with_alpha(opacity);
        }
    }
}

fn toast_text_opacity(
    toast_config: Res<ToastConfig>,
    mut query: Query<(&mut TextColor, &ToastElement)>,
) {
    for (mut text_color, toast) in &mut query {
        if let Some(opacity) = toast_config.opacity(toast.0) {
            text_color.0 = text_color.0.with_alpha(opacity);
        }
    }
}

#[cfg(test)]
mod test {
    use super::ToastConfig;

    #[test]
    fn test_top_values() {
        let mut config = ToastConfig::default();
        config.insert();
        config.insert();
        let a = config.insert();
        let a_top = config.top(a);
        assert_eq!(a_top, 30.0);
    }
}
