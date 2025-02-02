use bevy::{color::palettes::tailwind::*, prelude::*, utils::hashbrown::HashMap};
use generational_arena::{Arena, Index};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ToastConfig::default());
    }
}

#[derive(Resource)]
struct ToastConfig {
    arena: Arena<u16>,
    current_index: u16,
    opacity_map: HashMap<Index, f32>,
    ordering_vec: Vec<Index>,
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

struct UiPos {
    left: Val,
    top: Val,
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
    target_position: UiPos,
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
            target_position: UiPos {
                left: Val::Percent(75.0),
                top: Val::Percent(10.0),
            },
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

    fn state(&self) -> ToastState {
        self.state
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

impl Command for ToastNotification {
    fn apply(self, world: &mut World) {
        let mut toast_arena = world.get_resource_mut::<ToastConfig>().unwrap();
        let index = toast_arena.insert();
        world
            .spawn((
                Node {
                    width: Val::Percent(20.0),
                    height: Val::Percent(10.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    top: Val::Percent(90.0),
                    left: Val::Percent(75.90),
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
                            height: Val::Percent(20.0),
                            ..default()
                        },
                        BackgroundColor(BLUE_600.into()),
                        ToastHeader,
                        ToastElement(index),
                    ))
                    .with_children(|p| {
                        p.spawn((Text(self.title), ToastElement(index)));
                    });

                parent
                    .spawn((
                        Node {
                            display: Display::Flex,
                            align_content: AlignContent::Center,
                            justify_content: JustifyContent::Center,
                            width: Val::Percent(100.0),
                            height: Val::Percent(80.0),
                            ..default()
                        },
                        ToastBody,
                        ToastElement(index),
                    ))
                    .with_children(|p| {
                        p.spawn((Text(self.body), ToastElement(index)));
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
            use ToastState as T;
            match toast.state {
                T::FadeIn => {
                    node.top = Val::Percent(
                        100.0, /*TODO: Fully implement the ordering vec in toast config to always find the proper position for each notification */
                    )
                }
                T::OnScreen => {}
                T::FadeOut => {}
            }
        }
    }
}
