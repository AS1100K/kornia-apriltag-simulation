use bevy::prelude::*;
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};
#[cfg(feature = "dev")]
use bevy_rapier3d::render::RapierDebugRenderPlugin;
use kornia_apriltag_simulation::scene::setup_basic_scene;

fn main() -> AppExit {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_systems(Startup, setup_basic_scene);

    #[cfg(feature = "dev")]
    app.add_plugins(RapierDebugRenderPlugin::default());

    app.run()
}
