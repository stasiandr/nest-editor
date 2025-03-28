use std::path::PathBuf;

use bevy::prelude::*;
use bevy_file_dialog::prelude::*;


struct ByteContents;

#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct SceneObject;

pub struct EditorSceneManager;

impl Plugin for EditorSceneManager {
    fn build(&self, app: &mut App) {
        app
            .register_type::<EditorMeshWrapper>()
            .register_type::<EditorStandardMaterialWrapper>()
            .register_type::<SceneObject>()
            .add_event::<SaveScene>()
            .add_plugins(FileDialogPlugin::new().with_save_file::<ByteContents>())
            .add_systems(Startup, load_scene_system)
            .add_systems(Update, load_meshes)
            .add_systems(Update, load_standard_materials)
            .add_systems(Update, save_scene_system)
            .add_systems(Update, save_on_cmd_s);
    }
}

#[derive(Debug, Event)]
pub struct SaveScene;


#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct EditorMeshWrapper {
    pub mesh_path: String,
}

impl EditorMeshWrapper {
    pub fn new<T : Into<String>>(mesh_path: T) -> Self {
        Self { mesh_path: mesh_path.into() }
    }
}


#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct EditorStandardMaterialWrapper {
    pub color: Color,
}

impl EditorStandardMaterialWrapper {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

pub fn load_meshes(
    mut commands: Commands,
    mut query: Query<(Entity, &EditorMeshWrapper), Without<Mesh3d>>,
    asset_server: Res<AssetServer>,
) {
    for (entity, editor_mesh) in query.iter_mut() {
        let mesh = asset_server.load(editor_mesh.mesh_path.clone());
        commands.entity(entity).insert(Mesh3d(mesh));
    }
}

pub fn load_standard_materials(
    mut commands: Commands,
    mut query: Query<(Entity, &EditorStandardMaterialWrapper), Without<MeshMaterial3d<StandardMaterial>>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, editor_material) in query.iter_mut() {
        let material = materials.add(editor_material.color);
        commands.entity(entity).insert(MeshMaterial3d(material));
    }
}


pub fn load_scene_system(
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
) {

    if !PathBuf::from("scenes/main.scn.ron").exists() && !PathBuf::from("examples/example_bevy_project/assets/scenes/main.scn.ron").exists() {
        println!("Loading default");

        let scene = asset_server.load::<DynamicScene>("scenes/default.scn.ron");
        scene_spawner.spawn_dynamic(scene);

        return;
    }
    
    let scene = asset_server.load::<DynamicScene>("scenes/main.scn.ron");
    scene_spawner.spawn_dynamic(scene);
}


pub fn save_scene_system(world: &mut World) {
    let mut events = world.resource_mut::<Events<SaveScene>>();
    let drained = events.drain().collect::<Vec<_>>();

    if drained.is_empty() {
        return;
    }
    
    let scene_data =  {

        let mut query = world.query_filtered::<Entity, With<SceneObject>>();

        let scene = DynamicSceneBuilder::from_world(world)
            .deny_component::<Mesh3d>()
            .deny_component::<MeshMaterial3d<StandardMaterial>>()
            .deny_component::<Camera>()
            .deny_component::<bevy::render::camera::CameraRenderGraph>()
            .deny_component::<bevy::render::camera::Exposure>()
            .deny_component::<bevy::render::camera::CameraMainTextureUsages>()
            .deny_component::<bevy::render::view::ColorGrading>()
            .extract_entities(query.iter(world))
            .build();

        let type_registry = world.resource::<AppTypeRegistry>();
        let type_registry = type_registry.read();

        scene.serialize(&type_registry).unwrap()
    };
    
    world.commands().dialog().save_file::<ByteContents>(scene_data.as_bytes().to_vec());

    log::info!("Scene saved");
}

fn save_on_cmd_s(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut save_scene_events: ResMut<Events<SaveScene>>,
) {
    if keyboard_input.pressed(KeyCode::SuperLeft) && keyboard_input.just_pressed(KeyCode::KeyS) {
        save_scene_events.send(SaveScene);
    }
}