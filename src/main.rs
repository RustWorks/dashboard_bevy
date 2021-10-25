use std::fmt::Debug;
extern crate dashboard_derive;
use bevy::{
    prelude::*,
    render::{
        camera::OrthographicProjection,
        pipeline::{PipelineDescriptor, RenderPipeline, RenderPipelines},
        shader::ShaderStages,
    },
};
// use bimap::BiMap;
use std::collections::HashMap;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

trait KnobControl<T: PartialOrd + Copy> {
    fn set_position(&mut self, value: T);
    fn get_value(&self) -> T;
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum KnobState {
    Moving,
    Rotating,
    Idle,
}

// sorting: sort_by(|x,y| x.partial_cmp(y))
//////////////////// LINEAR KNOB ///////////////////
#[derive(Debug, Clone, Copy, PartialEq, Component)]
struct LinearKnob<T: PartialOrd + Copy> {
    position: T,
    previous_position: T,
    bounds: (T, T),
    previous_canvas_position: Vec2,

    id: KnobId,
    radius: f32,
    state: KnobState,
}

impl<T: PartialOrd + Copy> LinearKnob<T> {}

impl<T: PartialOrd + Copy> KnobControl<T> for LinearKnob<T> {
    fn set_position(&mut self, value: T) {
        self.position = value;
    }

    fn get_value(&self) -> T {
        self.position
    }
}
//////////////////// LINEAR KNOB ///////////////////

use num::Float;
//////////////////// LOGARITHMIC KNOB ///////////////////
#[derive(Debug, Clone, Copy, PartialEq)]
struct LogarithmicKnob<U: Float> {
    position: U,
}

impl<U: Float> LogarithmicKnob<U> {
    fn new(val: U) -> Self {
        Self { position: val }
    }
}
impl<U: Float> KnobControl<U> for LogarithmicKnob<U> {
    fn set_position(&mut self, value: U) {
        self.position = value;
    }

    fn get_value(&self) -> U {
        (self.position + num::one()).log2()
    }
}
//////////////////// LOGARITHMIC KNOB ///////////////////

#[derive(Debug, Clone, Component)]
enum Knob<T: PartialOrd + Copy = f32, U: Float = f32> /* defaults to f32 */ {
    Linear(LinearKnob<T>),
    Logarithmic(LogarithmicKnob<U>),
}

use serde::Serialize;
#[derive(Copy, Clone, PartialEq, EnumIter, Debug, Reflect, Hash, Serialize)]
#[reflect(Hash, Serialize, PartialEq)]
pub enum MyEnum {
    A,
    B,
    C,
}

// impl Into<f64> for MyEnum {
//     fn into(self) -> f64 {
//         // let length = MyEnum::iter().count();
//         let mut k = 0;
//         let mut my_enum = Self::iter();

//         // assumes non-empty enum
//         let mut e = my_enum.next().unwrap();

//         while e != self {
//             e = my_enum.next().unwrap();
//             k = k + 1;
//         }
//         k as f64
//     }
// }

impl From<MyEnum> for f64 {
    fn from(my: MyEnum) -> f64 {
        // let length = MyEnum::iter().count();
        let mut k = 0;
        let mut my_enum_iter = MyEnum::iter();

        // assumes non-empty enum
        let mut e = my_enum_iter.next().unwrap();

        while e != my {
            e = my_enum_iter.next().unwrap();
            k = k + 1;
        }
        k as f64
    }
}

// TODO: the case where num < -0.5
impl From<f64> for MyEnum {
    fn from(num: f64) -> Self {
        let mut my_enum_iter = MyEnum::iter();
        let mut e = my_enum_iter.next().unwrap();
        let mut e_f64: f64 = e.into();
        while !(e_f64 > num - 0.5 && e_f64 < num + 0.5) {
            e = my_enum_iter.next().unwrap();
            e_f64 = e.into();
        }
        e
    }
}

// #[derive(Debug, Clone, Reflect, Serialize, PartialEq, Hash)]
// #[reflect(Hash, Serialize, PartialEq)]
#[derive(Reflect, Debug)]
pub struct Globals {
    pub var1: f32,
    pub var2: u16,
    pub var3: MyEnum,
}

// #[derive(Debug, Clone, Reflect, Serialize, PartialEq, Hash)]
// #[reflect(Hash, Serialize, PartialEq)]
#[derive(Reflect, Debug)]
pub struct OtherGlobals {
    pub var1: MyEnum,
    pub var2: f64,
    pub var3: u8,
}

#[derive(Debug)]
struct AllVars<'a> {
    globals: &'a mut Globals,
    other_globals: &'a mut OtherGlobals,
}

struct SpawnKnobEvent(Vec2);

pub struct Maps {
    pub mesh_handles: HashMap<&'static str, Handle<Mesh>>,
    pub pipeline_handles: HashMap<&'static str, Handle<PipelineDescriptor>>,
    // pub sounds: HashMap<&'static str, Handle<AudioSource>>,
}

impl Default for Maps {
    fn default() -> Self {
        Maps {
            mesh_handles: HashMap::new(),
            pipeline_handles: HashMap::new(),
            // sounds: HashMap::new(),
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<SpawnKnobEvent>()
        .add_event::<ClickedOnKnob>()
        .add_event::<SpawnFieldLabel>()
        .init_resource::<ButtonMaterials>()
        .insert_resource(Maps::default())
        .insert_resource(ClearColor(Color::hex("6e7f80").unwrap()))
        .insert_resource(Globals {
            var1: 66.666f32,
            var2: 77u16,
            var3: MyEnum::C,
        })
        .insert_resource(Cursor::default())
        .insert_resource(OtherGlobals {
            var1: MyEnum::A,
            var2: 22f64,
            var3: 44u8,
        })
        .add_startup_system(setup.label("setup"))
        .add_startup_system(dashboard_variables.after("setup"))
        .add_system(spawn_text_label)
        .add_system(spawn_knob)
        .add_system(check_mouse)
        .add_system(record_mouse_events_system)
        .add_system(knob_action)
        .add_system(move_knob)
        .add_system(button_system)
        .run();
}

#[derive(Component)]
struct ColorText;

#[derive(Component)]
struct UiBoard;
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut maps: ResMut<Maps>,
) {
    let vert = asset_server.load::<Shader, _>("shaders/vert.vert");
    let ends = asset_server.load::<Shader, _>("shaders/bounding_box.frag");

    use std::{thread, time};
    let hundred_millis = time::Duration::from_millis(100);
    thread::sleep(hundred_millis);

    // commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(OrthographicCameraBundle {
        transform: Transform::from_translation(Vec3::new(00.0, 0.0, 10.0))
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        orthographic_projection: OrthographicProjection {
            scale: 1.0,
            far: 100000.0,
            near: -100000.0,
            ..Default::default()
        },
        ..OrthographicCameraBundle::new_2d()
    });

    commands.spawn_bundle(UiCameraBundle::default());

    let fields_button_pipeline_handle =
        pipelines.add(PipelineDescriptor::default_config(ShaderStages {
            vertex: vert.clone(),
            fragment: Some(ends.clone()),
        }));

    maps.pipeline_handles
        .insert("fields_button", fields_button_pipeline_handle.clone());

    let fields_button_mesh = meshes.add(Mesh::from(shape::Quad {
        size: Vec2::new(100.0, 50.0),
        flip: false,
    }));

    maps.mesh_handles
        .insert("fields_button", fields_button_mesh);

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(31.0), Val::Px(500.0)),
                // border: Rect::all(Val::Px(5.0)),
                position: Rect {
                    // top: Val::Px(15.0),
                    left: Val::Px(15.0),
                    bottom: Val::Px(15.0),
                    ..Default::default()
                },
                align_self: bevy::ui::AlignSelf::FlexStart,
                ..Default::default()
            },
            material: materials.add(Color::rgba(0.65, 0.65, 0.65, 0.5).into()),
            ..Default::default()
        })
        .insert(UiBoard);
}

struct SpawnFieldLabel(Vec<(String, String)>);

struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgba(0.8, 0.65, 0.5, 0.5).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
        }
    }
}

fn button_system(
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut material, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                // text.sections[0].value = "".to_string();
                *material = button_materials.pressed.clone();
            }
            Interaction::Hovered => {
                // text.sections[0].value = "".to_string();
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                // text.sections[0].value = "".to_string();
                *material = button_materials.normal.clone();
            }
        }
    }
}

fn spawn_text_label(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut text_event_reader: EventReader<SpawnFieldLabel>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ui_query: Query<Entity, With<UiBoard>>,
) {
    for event in text_event_reader.iter() {
        for (k, (key, value)) in event.0.iter().enumerate() {
            let ui_entity = ui_query.single();

            // let text_content = event.0.clone();
            // let key = text_content[0].clone();
            // let value = text_content[1].clone();
            let height = k as f32 * 60.0;

            let offset = 5.0;

            let button_entity = commands
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        align_self: AlignSelf::FlexEnd,
                        position_type: PositionType::Absolute,
                        size: Size::new(Val::Percent(90.0), Val::Px(50.0)),
                        position: Rect {
                            bottom: Val::Px(height + offset),
                            right: Val::Px(15.0),
                            left: Val::Px(15.0),
                            ..Default::default()
                        },

                        ..Default::default()
                    },

                    material: materials.add(Color::rgba(0.8, 0.65, 0.5, 0.5).into()),
                    ..Default::default()
                })
                .id();

            commands.entity(ui_entity).push_children(&[button_entity]);

            let fields_entity = commands
                .spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::FlexEnd,
                        position_type: PositionType::Absolute,
                        position: Rect {
                            bottom: Val::Percent(2.0),
                            left: Val::Percent(2.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    // Use the `Text::with_section` constructor
                    text: Text::with_section(
                        // Accepts a `String` or any type that converts into a `String`, such as `&str`
                        key,
                        TextStyle {
                            font: asset_server.load("fonts/Lekton-Regular.ttf"),
                            font_size: 50.0,
                            color: Color::NAVY,
                        },
                        // Note: You can use `Default::default()` in place of the `TextAlignment`
                        TextAlignment {
                            horizontal: HorizontalAlign::Left,
                            ..Default::default()
                        },
                    ),
                    ..Default::default()
                })
                .insert(ColorText)
                .id();

            let values_entity = commands
                .spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::FlexEnd,
                        position_type: PositionType::Absolute,
                        position: Rect {
                            bottom: Val::Percent(0.0),
                            right: Val::Percent(2.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    // Use the `Text::with_section` constructor
                    text: Text::with_section(
                        // Accepts a `String` or any type that converts into a `String`, such as `&str`
                        value,
                        TextStyle {
                            font: asset_server.load("fonts/Lekton-Regular.ttf"),
                            font_size: 50.0,
                            color: Color::NAVY,
                        },
                        // Note: You can use `Default::default()` in place of the `TextAlignment`
                        TextAlignment {
                            horizontal: HorizontalAlign::Right,
                            ..Default::default()
                        },
                    ),
                    ..Default::default()
                })
                .insert(ColorText)
                .id();

            commands
                .entity(button_entity)
                .push_children(&[fields_entity, values_entity]);
        }
    }
}

// fn query_text(mut commands: Commands, mut query: Query<&mut Text, With<ColorText>>) {
//     for mut text in query.iter_mut() {
//         text.sections[0].value = "yas".to_string();
//     }
// }

// use std::any::{Any, TypeId};
fn dashboard_variables(
    mut commands: Commands,
    mut globals: ResMut<Globals>,
    mut other_globals: ResMut<OtherGlobals>,
    mut text_event: EventWriter<SpawnFieldLabel>,
) {
    #[macro_export]
    macro_rules! attemp_downcasting {
        ($eq:expr, $($t:ty),+) => {{
            let mut maybe_value_f64: Option<f64> = None;
            let value = $eq;
            $(
                if let Some(val) = value.downcast_ref::<$t>() {
                    let val_f64: f64 = (*val).into();
                    // println!("success f64: {:?}", val_f64);
                    maybe_value_f64 = Some(val_f64);

                }
            )+
            maybe_value_f64
        }};
    }

    #[macro_export]
    macro_rules! add_to_dashboard_variables {
        ($($y:expr),*) => {{
            let mut field_map: HashMap<String, f64> = HashMap::new();
            let mut field_vec: Vec<(String, f64)> = Vec::new();
            let mut maybe_value_f64: Option<f64> = None;
            $(
                for (i, value) in $y.iter_fields().enumerate() {
                    let field_name = globals.name_at(i).unwrap();
                    let mut struct_name = stringify!($y).to_string();
                    struct_name.push('.');
                    struct_name.push_str(field_name);

                    maybe_value_f64 = attemp_downcasting![value, u8, u16, u32, i8, i16, i32, f32, f64, MyEnum];
                    if let Some(f64_value) = maybe_value_f64 {
                        field_map.insert(struct_name.clone(), f64_value);
                        field_vec.push((struct_name, f64_value));
                    }
                }
            )+
            (field_map, field_vec)

        }};
    }

    let (field_map, field_vec): (HashMap<String, f64>, Vec<(String, f64)>) =
        add_to_dashboard_variables!(globals, other_globals);
    let mut v: Vec<(String, String)> = Vec::new();
    for (key, value) in field_vec.iter() {
        let mut temp_key = key.clone();
        let value_string = format!("{:.4}", (*value).to_string());
        // temp_key.push_str(": ");
        // temp_key.push_str(&value_string);
        v.push((temp_key, value_string));
    }

    text_event.send(SpawnFieldLabel(v));

    // println!("field_map: {:?}", field_map);

    // for (i, value) in globals.iter_fields().enumerate() {
    //     let field_name = globals.name_at(i).unwrap();
    //     let num_types = vecco![value, usize, u8, u16, u32, u64, i8, i16, i32, f32, f64, MyEnum];
    // }

    let mut all_dash_vars = AllVars {
        globals: globals.as_mut(),
        other_globals: other_globals.as_mut(),
    };

    all_dash_vars.globals.var1 = 4321.0;
}

type KnobId = u32;
struct ClickedOnKnob(KnobId);

fn check_mouse(
    mouse_button_input: Res<Input<MouseButton>>,
    mut spawn_knob_event_writer: EventWriter<SpawnKnobEvent>,
    cursor: Res<Cursor>,
    keyboard_input: Res<Input<KeyCode>>,
    mut clicked_on_knob_event_writer: EventWriter<ClickedOnKnob>,
    mut query: Query<(&Transform, &mut LinearKnob<f32>)>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let pressed_g = keyboard_input.pressed(KeyCode::LControl);
        if pressed_g {
            spawn_knob_event_writer.send(SpawnKnobEvent(cursor.position));
        }

        for (transform, lin_knob) in query.iter_mut() {
            if cursor.position.distance(transform.translation.truncate()) < lin_knob.radius {
                clicked_on_knob_event_writer.send(ClickedOnKnob(lin_knob.id));
            }
        }
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        for (transform, mut lin_knob) in query.iter_mut() {
            lin_knob.state = KnobState::Idle;
            lin_knob.previous_canvas_position = transform.translation.truncate();
            lin_knob.previous_position = lin_knob.position;
        }
    }
}

fn spawn_knob(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut globals: ResMut<Globals>,
    mut other_globals: ResMut<OtherGlobals>,
    mut spawn_knob_event: EventReader<SpawnKnobEvent>,
    mut maps: ResMut<Maps>,
) {
    for event in spawn_knob_event.iter() {
        let mouse_position = event.0;
        println!("{:?}", mouse_position);
        let texture_handle = asset_server.load("textures/knob.png");

        let scale = 1.0;
        let scale_transform = Transform::from_scale(Vec3::new(scale, scale, 1.0));

        use rand::Rng;
        let mut rng = rand::thread_rng();
        let id = rng.gen();

        let sprite_size = 75.0;
        let knob: LinearKnob<f32> = LinearKnob {
            position: 0.0,
            bounds: (0.0, 1.0),
            previous_canvas_position: mouse_position,
            previous_position: 0.0,
            id,
            radius: sprite_size * scale * 0.8,
            state: KnobState::Idle,
        };

        // // attach knob to a variable here using an id for the knob and a reflection?
        // let mut knobs: HashMap<u32, Knob> = HashMap::new();
        // knobs.insert(0, knob_a);
        // knobs.insert(1, knob_b);

        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(texture_handle.into()),
                sprite: Sprite::new(Vec2::new(sprite_size, sprite_size)),
                transform: Transform::from_translation(mouse_position.extend(0.0))
                    .mul_transform(scale_transform),

                ..Default::default()
            })
            .insert(knob);

        // let field_button_pipeline_handle = maps.pipeline_handles["fields_button"].clone();

        // let render_piplines_fields_button =
        //     RenderPipelines::from_pipelines(vec![RenderPipeline::new(
        //         field_button_pipeline_handle,
        //     )]);

        // let fields_button_mesh_handle = maps.mesh_handles["fields_button"].clone();
        // // a mesh that acts like a button
        // commands
        //     .spawn_bundle(MeshBundle {
        //         mesh: fields_button_mesh_handle.clone(),
        //         // visible: visible_anchors.clone(),
        //         render_pipelines: render_piplines_fields_button.clone(),
        //         transform: Transform::from_translation(Vec3::new(100.0, height, -10.0)),
        //         ..Default::default()
        //     })
        //     // .insert(shader_params_handle_bb.clone())
        //     // .insert(field_id)
        //     .id();
    }
}

fn knob_action(
    mut query: Query<&mut LinearKnob<f32>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut clicked_on_knob_event_reader: EventReader<ClickedOnKnob>,
) {
    for event in clicked_on_knob_event_reader.iter() {
        let knob_id = event.0;
        println!("HEERE: {:?}", knob_id);
        for mut lin_knob in query.iter_mut() {
            if knob_id == lin_knob.id && keyboard_input.pressed(KeyCode::LShift) {
                lin_knob.state = KnobState::Moving;
            } else if knob_id == lin_knob.id {
                lin_knob.state = KnobState::Rotating;
            }
        }
    }
}

fn move_knob(mut query: Query<(&mut Transform, &mut LinearKnob<f32>)>, cursor: Res<Cursor>) {
    for (mut transform, mut lin_knob) in query.iter_mut() {
        match lin_knob.state {
            KnobState::Moving => {
                transform.translation =
                    (lin_knob.previous_canvas_position + cursor.pos_relative_to_click).extend(0.0);
            }
            KnobState::Rotating => {
                let new_angle = lin_knob.previous_position
                    - cursor.pos_relative_to_click.y
                        / (100.0 + cursor.pos_relative_to_click.x.abs());
                transform.rotation = Quat::from_rotation_z(new_angle);
                lin_knob.position = new_angle;
                println!("{:?}", new_angle);
            }
            _ => {}
        }
    }
}

pub struct Cursor {
    pub position: Vec2,
    pub pos_relative_to_click: Vec2,
    pub last_click_position: Vec2,
}

impl Default for Cursor {
    fn default() -> Self {
        Cursor {
            position: Vec2::ZERO,
            pos_relative_to_click: Vec2::ZERO,
            last_click_position: Vec2::ZERO,
        }
    }
}

pub fn record_mouse_events_system(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut cursor_res: ResMut<Cursor>,
    mut windows: ResMut<Windows>,
    cam_transform_query: Query<&Transform, With<OrthographicProjection>>,
    cam_ortho_query: Query<&OrthographicProjection>,
) {
    for event in cursor_moved_events.iter() {
        let cursor_in_pixels = event.position; // lower left is origin
        let window_size = Vec2::new(
            windows.get_primary_mut().unwrap().width(),
            windows.get_primary_mut().unwrap().height(),
        );

        let screen_position = cursor_in_pixels - window_size / 2.0;

        let cam_transform = cam_transform_query.iter().next().unwrap();

        // this variable currently has no effect
        let mut scale = 1.0;

        for ortho in cam_ortho_query.iter() {
            scale = ortho.scale;
        }

        let cursor_vec4: Vec4 = cam_transform.compute_matrix()
            * screen_position.extend(0.0).extend(1.0 / (scale))
            * scale;

        let cursor_pos = Vec2::new(cursor_vec4.x, cursor_vec4.y);
        cursor_res.position = cursor_pos;
        cursor_res.pos_relative_to_click = cursor_res.position - cursor_res.last_click_position;
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        cursor_res.last_click_position = cursor_res.position;
        cursor_res.pos_relative_to_click = Vec2::ZERO;
    }
}
