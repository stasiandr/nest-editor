use bevy::prelude::*;
use bevy_file_dialog::prelude::*;


struct ByteContents;

#[derive(Component, Debug, Reflect, Default)]
pub struct SceneObject {
    name: String,
}

pub struct EditorSceneManager;

impl Plugin for EditorSceneManager {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SaveScene>()
            .add_plugins(FileDialogPlugin::new().with_save_file::<ByteContents>())
            .add_systems(Update, save_scene_system)
            .add_systems(Update, save_on_cmd_s);
    }
}

#[derive(Debug, Event)]
pub struct SaveScene;

pub fn save_scene_system(world: &mut World) {
    let mut events = world.resource_mut::<Events<SaveScene>>();
    let drained = events.drain().collect::<Vec<_>>();

    if drained.is_empty() {
        return;
    }
    
    let scene_data =  {

        let mut query = world.query_filtered::<Entity, With<SceneObject>>();

        let scene = DynamicSceneBuilder::from_world(world)
            .deny_all_resources()
            .extract_entities(query.iter(world))
            .build();

        let type_registry = world.resource::<AppTypeRegistry>();
        let type_registry = type_registry.read();

        scene.serialize(&type_registry).unwrap()
    };
    
    world.commands().dialog().save_file::<ByteContents>(scene_data.as_bytes().to_vec());
}

fn save_on_cmd_s(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut save_scene_events: ResMut<Events<SaveScene>>,
) {
    if keyboard_input.pressed(KeyCode::SuperLeft) && keyboard_input.just_pressed(KeyCode::KeyS) {
        save_scene_events.send(SaveScene);
    }
}