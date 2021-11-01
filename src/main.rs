mod macro_template;
mod util;

use macro_template::*;
use util::*;

use std::fmt::Debug;
extern crate dashboard_derive;
use bevy::{
    prelude::*,
    reflect::DynamicStruct,
    render::{
        camera::OrthographicProjection,
        // pipeline::{PipelineDescriptor, RenderPipeline, RenderPipelines},
        pipeline::PipelineDescriptor,
        shader::ShaderStages,
    },
};
// use bimap::BiMap;
// use std::collections::HashMap;

// use num::traits::Zero;
// use std::any::Any;
// use strum::IntoEnumIterator;
// use strum_macros::EnumIter;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<SpawnKnobEvent>()
        .add_event::<ClickedOnKnob>()
        .add_event::<SpawnLabels>()
        // .add_event::<SpawnComponentLabels>()
        .add_event::<KnobRotated>()
        .add_event::<ReleasedOnKnob>()
        .add_event::<ChangedDashVar>()
        .init_resource::<ButtonMaterials>()
        .insert_resource(DynamicStruct::default())
        .insert_resource(Maps::default())
        .insert_resource(ClearColor(Color::hex("6e7f80").unwrap()))
        // .insert_resource(FieldKnobMap(BiMap::new()))
        .insert_resource(Globals {
            var1: 66.666f32,
            var2: 77u16,
            var3: 55i64,
        })
        .insert_resource(Cursor::default())
        .insert_resource(OtherGlobals {
            var1: 8f64,
            var2: 22u64,
            var3: 44u8,
        })
        .insert_resource(AllDashRes::new())
        .insert_resource(AllDashComp::new())
        // .add_startup_system(dashboard_variables_setup.after("setup"))
        .add_startup_system(setup.label("setup"))
        .add_startup_system(dashboard_variables_setup.exclusive_system().at_end())
        .add_system(update_dashboard_variables)
        .add_system(spawn_text_label)
        .add_system(spawn_knob)
        .add_system(record_mouse_events_system)
        .add_system(knob_action)
        .add_system(move_knob)
        .add_system(button_system)
        .add_system(modify_field_upon_knob_change)
        .add_system(print_global)
        .add_system(attach_knob_to_field)
        .add_system(update_dashboard_labels)
        .add_system(move_octo)
        .add_system(check_mouse.exclusive_system().at_end())
        // .add_system(cleanup_system::<KnobSprite>)
        .run();
}

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

    let texture_handle = asset_server.load("textures/octopus.png");
    let sprite_size = 100.0;
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(texture_handle.clone().into()),
            sprite: Sprite::new(Vec2::new(sprite_size, sprite_size)),
            transform: Transform::from_translation(Vec3::new(0.0, -90.0, 0.0)),

            ..Default::default()
        })
        // .insert(knob)
        .insert(MyComponent {
            y_position: -90.0,
            v2: 25,
            v3: MyEnum::B,
        })
        .insert(DashComponent);

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(texture_handle.into()),
            sprite: Sprite::new(Vec2::new(sprite_size, sprite_size)),
            transform: Transform::from_translation(Vec3::new(0.0, 90.0, 0.0)),

            ..Default::default()
        })
        // .insert(knob)
        .insert(MyComponent {
            y_position: 90.0,
            v2: 25,
            v3: MyEnum::C,
        })
        .insert(DashComponent);
}

fn move_octo(
    // mut comp_query_set: QuerySet<(
    //     QueryState<(Entity, &MyComponent), With<DashComponent>>,
    //     QueryState<(Entity, &mut Transform), (With<DashComponent>, Changed<MyComponent>)>,
    // )>,
    mut query: Query<
        (Entity, &mut Transform, &MyComponent),
        (With<DashComponent>, Changed<MyComponent>),
    >,
) {
    // let my_component_query = comp_query_set.q0();
    // let y_pos = my_component_query.iter().next().unwrap().1.y_position;
    // let mut transform_query = comp_query_set.q1();
    for (_entity, mut transform, my_component) in query.iter_mut() {
        transform.translation.y = my_component.y_position;
        // let mut transform = transform_query.iter_mut().next();
        // if let Some(pair) = transform.as_mut() {
        //     pair.1.translation.y = y_pos;
        // }
    }
}

fn button_system(
    mut commands: Commands,
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (Entity, &Interaction, &mut Handle<ColorMaterial>, &Children),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (entity, interaction, mut material, _children) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();
                // *button_phantom_state.as_mut() = ButtonPhantomState::Moving;
                commands.entity(entity).insert(MovingButton);
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}

fn spawn_text_label(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut text_res_event_reader: EventReader<SpawnLabels>,
    // mut text_comp_event_reader: EventReader<SpawnComponentLabels>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    // ui_res_query: Query<Entity, With<UiBoardResources>>,
    // ui_comp_query: Query<Entity, With<UiBoardComponents>>,
) {
    for event in text_res_event_reader.iter() {
        let ui_entity = event.1;
        let scale = 0.75;
        for (k, (key, value)) in event.0.iter().enumerate() {
            // let mut ui_entity = ui_res_query.single();

            // let text_content = event.0.clone();
            // let key = text_content[0].clone();
            // let value = text_content[1].clone();
            let height = k as f32 * 60.0 * scale;

            let offset = 10.0;

            let button_entity = commands
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        align_self: AlignSelf::FlexEnd,
                        position_type: PositionType::Absolute,
                        size: Size::new(Val::Percent(90.0), Val::Px(50.0 * scale)),
                        position: Rect {
                            top: Val::Px(height + offset),
                            right: Val::Px(15.0),
                            left: Val::Px(15.0),
                            ..Default::default()
                        },

                        ..Default::default()
                    },

                    material: materials.add(Color::rgba(0.8, 0.65, 0.5, 0.5).into()),
                    ..Default::default()
                })
                .insert(ButtonId(key.to_string()))
                .id();

            commands.entity(ui_entity).push_children(&[button_entity]);

            let fields_entity = commands
                .spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::FlexEnd,
                        position_type: PositionType::Absolute,
                        position: Rect {
                            top: Val::Percent(2.0),
                            left: Val::Percent(2.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },

                    text: Text::with_section(
                        key,
                        TextStyle {
                            font: asset_server.load("fonts/Lekton-Regular.ttf"),
                            font_size: 50.0 * scale,
                            color: Color::NAVY,
                        },
                        TextAlignment {
                            horizontal: HorizontalAlign::Left,
                            ..Default::default()
                        },
                    ),
                    ..Default::default()
                })
                .insert(ColorText)
                .id();

            let value_string = format!("{:.5}", value.to_string());
            let values_entity = commands
                .spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::FlexEnd,
                        position_type: PositionType::Absolute,
                        position: Rect {
                            top: Val::Percent(0.0),
                            right: Val::Percent(2.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },

                    text: Text::with_section(
                        value_string,
                        TextStyle {
                            font: asset_server.load("fonts/Lekton-Regular.ttf"),
                            font_size: 50.0 * scale,
                            color: Color::NAVY,
                        },
                        TextAlignment {
                            horizontal: HorizontalAlign::Right,
                            ..Default::default()
                        },
                    ),
                    ..Default::default()
                })
                .insert(FieldValueText(key.to_string()))
                .id();

            commands
                .entity(button_entity)
                .push_children(&[fields_entity, values_entity]);
        }
    }
}

// fn query_text(mut commands: Commands, mut query: Query<&mut Text, With<ColorText>>) {

// }

// fn cleanup_system<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
//     for e in q.iter() {
//         commands.entity(e).despawn_recursive();
//     }
// }

fn print_global(
    keyboard_input: Res<Input<KeyCode>>,
    globals: Res<Globals>,
    other_globals: Res<OtherGlobals>,
    query: Query<(&LinearKnob<i64>)>,
    dynstruct: Res<DynamicStruct>,
) {
    if keyboard_input.just_pressed(KeyCode::V) {
        println!("{:?}", globals);
        println!("{:?}", other_globals);
        for knob in query.iter() {
            println!("knob: {:?}", knob)
        }
        // println!("dynstruct: {:?}", dynstruct.name());
    }
}

// view
fn update_dashboard_labels(
    mut query: Query<(&mut Text, &FieldValueText)>,
    mut changed_dash_var_event: EventReader<ChangedDashVar>,
) {
    // iterates over all text fields, optimization required
    for ChangedDashVar(field_name, value_f64) in changed_dash_var_event.iter() {
        for (mut text, struct_key) in query.iter_mut() {
            // println!("struct_key : {:?}", &struct_key.0);

            if &struct_key.0 == field_name {
                //
                let value_string = format!("{:.5}", value_f64.to_string());
                text.sections[0].value = value_string;
            }
        }
    }
}

fn modify_field_upon_knob_change(
    mut knob_rotated_event: EventWriter<KnobRotated>,
    // query: Query<&LinearKnob<f64>, With<RotatingKnob>>,
    mut query_set: QuerySet<(
        QueryState<&LinearKnob<f64>, With<RotatingKnob>>,
        QueryState<&LinearKnob<i64>, With<RotatingKnob>>,
    )>,
) {
    for knob in query_set.q0().iter() {
        if let Some(field_name) = knob.linked_field.clone() {
            knob_rotated_event.send(KnobRotated(field_name, knob.get_value() as f32))
        }
    }
    for knob in query_set.q1().iter() {
        if let Some(field_name) = knob.linked_field.clone() {
            knob_rotated_event.send(KnobRotated(field_name, knob.get_value() as f32))
        }
    }
}

fn check_mouse(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    mut spawn_knob_event_writer: EventWriter<SpawnKnobEvent>,
    cursor: Res<Cursor>,
    keyboard_input: Res<Input<KeyCode>>,
    mut clicked_on_knob_event_writer: EventWriter<ClickedOnKnob>,
    // released_on_knob_event_writer: EventWriter<ReleasedOnKnob>,
    button_query: Query<Entity, (With<Button>, With<MovingButton>)>,
    mut query_set: QuerySet<(
        QueryState<(Entity, &Transform, &mut KnobSprite)>,
        QueryState<(Entity, &mut LinearKnob<f64>), With<RotatingKnob>>,
        QueryState<(Entity, &mut LinearKnob<i64>), With<RotatingKnob>>,
        QueryState<(Entity, &Transform, &mut KnobSprite), With<TranslatingKnob>>,
    )>,
    // knob_query: Query<(Entity, &Transform, &mut LinearKnob<f32>)>,
    // mut moving_knob_query: Query<(Entity, &Transform, &mut LinearKnob<f32>), With<RotatingKnob>>,
) {
    let mouse_just_pressed = mouse_button_input.just_pressed(MouseButton::Left);
    let mouse_just_released = mouse_button_input.just_released(MouseButton::Left);

    let mut clicked_on_knob: Option<(Entity, u32)> = None;
    let mut released_on_knob: Option<(Entity, u32)> = None;

    // shared computations
    if mouse_just_pressed || mouse_just_released {
        let mut knob_sorite_query = query_set.q0();
        for (entity, transform, knob_sprite) in knob_sorite_query.iter_mut() {
            if cursor.position.distance(transform.translation.truncate()) < knob_sprite.radius {
                if mouse_just_pressed {
                    clicked_on_knob = Some((entity, knob_sprite.id));
                }
                if mouse_just_released {
                    released_on_knob = Some((entity, knob_sprite.id));
                }
            }
        }
    }

    if mouse_just_pressed {
        let pressed_ctrl = keyboard_input.pressed(KeyCode::LControl);
        //
        // case of spawning a knob
        if pressed_ctrl {
            spawn_knob_event_writer.send(SpawnKnobEvent(cursor.position));

        // case of clicking on a knob (multiple possible actions)
        } else if let Some((_knob_entity, id)) = clicked_on_knob {
            clicked_on_knob_event_writer.send(ClickedOnKnob(id));
        }
    }

    if mouse_just_released {
        for button_entity in button_query.iter() {
            if let Some((knob_entity, _id)) = released_on_knob {
                // released_on_knob_event_writer.send(ReleasedOnKnob(id));
                commands.entity(button_entity).insert(LinkingFieldToKnob);
                commands.entity(knob_entity).insert(LinkingFieldToKnob);
            }
            commands.entity(button_entity).remove::<MovingButton>();
        }

        // remove "RotatingKnob" tag on currently rotating knob (f64)
        for (entity, mut lin_knob) in query_set.q1().iter_mut() {
            // lin_knob.state = KnobState::Idle;
            commands.entity(entity).remove::<RotatingKnob>();
            lin_knob.previous_position = lin_knob.position;
        }

        // remove "RotatingKnob" tag on currently rotating knob (i64)
        for (entity, mut lin_knob) in query_set.q2().iter_mut() {
            // lin_knob.state = KnobState::Idle;
            commands.entity(entity).remove::<RotatingKnob>();
            lin_knob.previous_position = lin_knob.position;
        }

        // remove "TranslatingKnob" tag on currently moving knob
        for (entity, transform, mut knob_sprite) in query_set.q3().iter_mut() {
            // lin_knob.state = KnobState::Idle;
            commands.entity(entity).remove::<TranslatingKnob>();
            knob_sprite.previous_position = transform.translation.truncate();
        }
    }
}

fn spawn_knob(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut spawn_knob_event: EventReader<SpawnKnobEvent>,
) {
    for event in spawn_knob_event.iter() {
        let mouse_position = event.0;
        // println!("{:?}", mouse_position);
        let texture_handle = asset_server.load("textures/knob.png");

        let scale = 1.0;
        let scale_transform = Transform::from_scale(Vec3::new(scale, scale, 1.0));

        use rand::Rng;
        let mut rng = rand::thread_rng();
        let id: KnobId = rng.gen();

        let sprite_size = 75.0;

        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(texture_handle.into()),
                sprite: Sprite::new(Vec2::new(sprite_size, sprite_size)),
                transform: Transform::from_translation(mouse_position.extend(0.0))
                    .mul_transform(scale_transform),

                ..Default::default()
            })
            // .insert(knob)
            .insert(KnobSprite {
                id,
                position: mouse_position,
                previous_position: mouse_position,
                radius: 75.0 * 0.8,
            });

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
    mut commands: Commands,
    mut knob_query: Query<(
        Entity,
        Option<&mut LinearKnob<f64>>,
        Option<&mut LinearKnob<i64>>,
    )>,
    // mut knob_query: Query<(Entity, &mut LinearKnob<f64>)>,
    // mut knob_query2: Query<(Entity, &mut LinearKnob<i64>)>,
    sprite_knob_query: Query<(Entity, &KnobSprite)>,
    keyboard_input: Res<Input<KeyCode>>,
    mut clicked_on_knob_event_reader: EventReader<ClickedOnKnob>,
) {
    for event in clicked_on_knob_event_reader.iter() {
        let knob_id = event.0;
        // println!("HEERE: {:?}", knob_id);
        for (entity, lin_knob_opt, lin_knob_opt2) in knob_query.iter_mut() {
            let no_s = !keyboard_input.pressed(KeyCode::LShift);
            if let Some(lin_knob) = lin_knob_opt {
                if knob_id == lin_knob.id && no_s {
                    commands.entity(entity).insert(RotatingKnob);
                }
            }
            if let Some(lin_knob) = lin_knob_opt2 {
                if knob_id == lin_knob.id && no_s {
                    commands.entity(entity).insert(RotatingKnob);
                }
            }
        }

        // for (entity, mut lin_knob) in knob_query2.iter_mut() {
        //     if knob_id == lin_knob.id && !keyboard_input.pressed(KeyCode::LShift) {
        //         commands.entity(entity).insert(RotatingKnob);
        //     }
        // }

        for (entity, knob_sprite) in sprite_knob_query.iter() {
            if knob_id == knob_sprite.id && keyboard_input.pressed(KeyCode::LShift) {
                commands.entity(entity).insert(TranslatingKnob);
            }
        }
    }
}

fn move_knob(
    // mut query: Query<(&mut Transform, &mut LinearKnob<f32>)>,
    mut query_set: QuerySet<(
        QueryState<(&mut Transform, &mut LinearKnob<i64>), With<RotatingKnob>>,
        QueryState<(&mut Transform, &mut LinearKnob<f64>), With<RotatingKnob>>,
        QueryState<(&mut Transform, &mut KnobSprite), With<TranslatingKnob>>,
    )>,
    cursor: Res<Cursor>,
) {
    for (mut transform, knob_sprite) in query_set.q2().iter_mut() {
        transform.translation =
            (knob_sprite.previous_position + cursor.pos_relative_to_click).extend(0.0);
    }

    for (mut transform, mut lin_knob) in query_set.q0().iter_mut() {
        let mut new_angle = lin_knob.previous_position as f32
            + lin_knob.speed * cursor.pos_relative_to_click.y / 100.0;
        new_angle = new_angle.round();
        lin_knob.set_position(new_angle as i64);
        transform.rotation = Quat::from_rotation_z(-lin_knob.get_angle());

        // println!("{:?}", new_angle);
    }

    for (mut transform, mut lin_knob) in query_set.q1().iter_mut() {
        let new_angle = lin_knob.previous_position as f32
            + lin_knob.speed * cursor.pos_relative_to_click.y / 100.0;

        lin_knob.set_position(new_angle as f64);
        transform.rotation = Quat::from_rotation_z(-lin_knob.get_angle());

        // println!("{:?}", new_angle);
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
