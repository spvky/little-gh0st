use bevy::prelude::*;

#[derive(Component)]
pub struct ToastContainer;
#[derive(Component)]
pub struct ToastHeader;
#[derive(Component)]
pub struct ToastBody;

pub struct ToastNotifcation {
    title: String,
    body: String,
}
