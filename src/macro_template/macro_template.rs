use crate::util::*;
extern crate dashboard_derive;
use bevy::prelude::*;
use strum::IntoEnumIterator;
// use bimap::BiMap;
// use std::collections::HashMap;
// use num::traits::Zero;
// use std::any::Any;
// use strum_macros::EnumIter;

impl Default for MyEnum {
    fn default() -> Self {
        Self::A
    }

    // fn sort_values(&self)
    // sorting: sort_by(|x,y| x.partial_cmp(y))
}

impl From<MyEnum> for i64 {
    fn from(my: MyEnum) -> i64 {
        // let length = MyEnum::iter().count();
        let mut k = 0;
        let mut my_enum_iter = MyEnum::iter();

        // assumes non-empty enum
        let mut e = my_enum_iter.next().unwrap();

        while e != my {
            e = my_enum_iter.next().unwrap();
            k = k + 1;
        }
        k as i64
    }
}

// TODO: the case where num < -0.5
impl From<i64> for MyEnum {
    fn from(num: i64) -> Self {
        let mut my_enum_iter = MyEnum::iter();
        if num < 0 {
            return my_enum_iter.next().unwrap();
        }

        if num as usize > my_enum_iter.len() {
            return my_enum_iter.last().unwrap().into();
        }

        let mut e = my_enum_iter.next().unwrap();
        let mut e_i64: i64 = e.into();

        while !(e_i64 >= num) {
            if let Some(e_temp) = my_enum_iter.next() {
                e_i64 = e_temp.into();
                e = e_temp;
            } else {
                return e;
            }
        }
        e
    }
}

impl MyComponent {
    fn myenum_from_nbr(&mut self, nbr: Nbr) {
        if let Nbr::Int64(inner) = nbr {
            self.v3 = MyEnum::from(inner);
        }
    }
}

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
            if let Some(e_temp) = my_enum_iter.next() {
                e_f64 = e_temp.into();
                e = e_temp;
            } else {
                return e;
            }
        }
        e
    }
}

#[derive(Reflect, Debug, Clone)]
pub struct GlobalsNbr {
    pub var1: Nbr,
    pub var2: Nbr,
    pub var3: Nbr,
}

#[derive(Reflect, Debug, Clone)]
pub struct OtherGlobalsNbr {
    pub var1: Nbr,
    pub var2: Nbr,
    pub var3: Nbr,
}

#[derive(Reflect)]
pub struct AllDashRes {
    pub globals: GlobalsNbr,
    pub other_globals: OtherGlobalsNbr,
}

impl AllDashRes {
    pub fn new() -> Self {
        Self {
            globals: GlobalsNbr {
                var1: Nbr::Float32(0.0),
                var2: Nbr::UInt16(0),
                var3: Nbr::Int64(0),
            },
            other_globals: OtherGlobalsNbr {
                var1: Nbr::Float64(0.0),
                var2: Nbr::UInt64(0),
                var3: Nbr::UInt8(0),
            },
        }
    }

    pub fn set_values_from_resources(&self, globals: Globals, other_globals: OtherGlobals) -> Self {
        Self {
            globals: GlobalsNbr {
                var1: Nbr::Float32(globals.var1),
                var2: Nbr::UInt16(globals.var2),
                var3: Nbr::Int64(globals.var3),
            },
            other_globals: OtherGlobalsNbr {
                var1: Nbr::Float64(other_globals.var1),
                var2: Nbr::UInt64(other_globals.var2),
                var3: Nbr::UInt8(other_globals.var3),
            },
        }
    }

    pub fn modify_field(
        &mut self,
        field_name: &str,
        new_value: Box<dyn Reflect>,
        globals: &mut Globals,
        other_globals: &mut OtherGlobals,
    ) {
        match field_name {
            "globals.var1" => {
                let new_val_downcasted = *new_value.downcast_ref::<f32>().unwrap();
                globals.var1 = new_val_downcasted;
                self.globals.var1 = Nbr::Float32(new_val_downcasted);
            }
            "globals.var2" => {
                let new_val_downcasted = *new_value.downcast_ref::<u16>().unwrap();
                globals.var2 = new_val_downcasted;
                self.globals.var2 = Nbr::UInt16(new_val_downcasted);
            }
            "globals.var3" => {
                let new_val_downcasted = *new_value.downcast_ref::<i64>().unwrap();
                globals.var3 = new_val_downcasted;
                self.globals.var3 = Nbr::Int64(new_val_downcasted);
            }
            "other_globals.var1" => {
                let new_val_downcasted = *new_value.downcast_ref::<f64>().unwrap();
                other_globals.var1 = new_val_downcasted;
                self.other_globals.var1 = Nbr::Float64(new_val_downcasted);
            }
            "other_globals.var2" => {
                let new_val_downcasted = *new_value.downcast_ref::<u64>().unwrap();
                other_globals.var2 = new_val_downcasted;
                self.other_globals.var2 = Nbr::UInt64(new_val_downcasted);
            }
            "other_globals.var3" => {
                let new_val_downcasted = *new_value.downcast_ref::<u8>().unwrap();
                other_globals.var3 = new_val_downcasted;
                self.other_globals.var3 = Nbr::UInt8(new_val_downcasted);
            }

            _ => {}
        }
    }
}

#[derive(Reflect)]
pub struct AllDashComp {
    pub my_component1: MyComponentNbr,
    pub my_component2: MyComponentNbr,
}

#[derive(Reflect, Debug, Clone)]
pub struct MyComponentNbr {
    pub y_position: Nbr,
    pub v2: Nbr,
    pub v3: MyEnum,
}

impl AllDashComp {
    pub fn new() -> Self {
        Self {
            my_component1: MyComponentNbr {
                y_position: Nbr::Float32(0.0),
                v2: Nbr::UInt16(0),
                v3: MyEnum::A,
            },
            my_component2: MyComponentNbr {
                y_position: Nbr::Float32(0.0),
                v2: Nbr::UInt16(0),
                v3: MyEnum::A,
            },
        }
    }

    pub fn set_values_from_components(
        &self,
        my_component1: MyComponent,
        my_component2: MyComponent,
    ) -> Self {
        Self {
            my_component1: MyComponentNbr {
                y_position: Nbr::Float32(my_component1.y_position),
                v2: Nbr::UInt16(my_component1.v2),
                v3: my_component1.v3,
            },
            my_component2: MyComponentNbr {
                y_position: Nbr::Float32(my_component2.y_position),
                v2: Nbr::UInt16(my_component2.v2),
                v3: my_component2.v3,
            },
        }
    }

    pub fn modify_field(
        &mut self,
        field_name: &str,
        new_value: Box<dyn Reflect>,
        my_component1: &mut MyComponent,
        my_component2: &mut MyComponent,
    ) {
        match field_name {
            "my_component1.y_position" => {
                let new_val_downcasted = *new_value.downcast_ref::<f32>().unwrap();
                my_component1.y_position = new_val_downcasted;
                self.my_component1.y_position = Nbr::Float32(new_val_downcasted.into());
            }
            "my_component1.v2" => {
                let new_val_downcasted = *new_value.downcast_ref::<u16>().unwrap();
                my_component1.v2 = new_val_downcasted;
                self.my_component1.v2 = Nbr::UInt16(new_val_downcasted.into());
            }
            "my_component1.v3" => {
                let new_val_downcasted = *new_value.downcast_ref::<MyEnum>().unwrap();
                my_component1.v3 = new_val_downcasted;
                self.my_component1.v3 = new_val_downcasted.into();
            }
            "my_component2.y_position" => {
                let new_val_downcasted = *new_value.downcast_ref::<f32>().unwrap();
                my_component2.y_position = new_val_downcasted;
                self.my_component1.y_position = Nbr::Float32(new_val_downcasted.into());
            }
            "my_component2.v2" => {
                let new_val_downcasted = *new_value.downcast_ref::<u16>().unwrap();
                my_component2.v2 = new_val_downcasted;
                self.my_component1.v2 = Nbr::UInt16(new_val_downcasted.into());
            }
            "my_component2.v3" => {
                let new_val_downcasted = *new_value.downcast_ref::<MyEnum>().unwrap();
                my_component2.v3 = new_val_downcasted;
                self.my_component1.v3 = new_val_downcasted.into();
            }

            _ => {}
        }
    }
}

impl Globals {
    pub fn modify_and_get_field(&mut self, field_name: &str, value_f64: f64) -> String {
        let var_string = match field_name {
            "var1" => {
                self.var1 = value_f64 as f32;
                self.var1.to_string()
            }
            "var2" => {
                self.var2 = value_f64 as u16;
                self.var2.to_string()
            }
            "var3" => {
                self.var3 = value_f64 as i64;
                self.var3.to_string()
            }
            _ => "".to_string(),
        };

        return format!("{:.5}", var_string.to_string());
    }
}

impl OtherGlobals {
    pub fn modify_and_get_field(&mut self, field_name: &str, value_f64: f64) -> String {
        let var_string = match field_name {
            "var1" => {
                self.var1 = value_f64 as f64;
                self.var1.to_string()
            }
            "var2" => {
                self.var2 = value_f64 as u64;
                self.var2.to_string()
            }
            "var3" => {
                self.var3 = value_f64 as u8;
                self.var3.to_string()
            }
            _ => "".to_string(),
        };

        return format!("{:.5}", var_string);
    }
}

impl MyComponent {
    pub fn modify_and_get_field(&mut self, field_name: &str, value_f64: f64) -> String {
        let var_string = match field_name {
            "y_position" => {
                self.y_position = value_f64 as f32;
                self.y_position.to_string()
            }
            "v2" => {
                self.v2 = value_f64 as u16;
                self.v2.to_string()
            }
            "v3" => {
                self.v3 = MyEnum::from(value_f64);
                format!("{:?}", self.v3)
            }
            _ => "".to_string(),
        };

        return format!("{:.5}", var_string.to_string());
    }
}

#[macro_export]
macro_rules! attemp_downcasting {
($eq:expr, $($inner_value_type:ty),+) => {{
    let mut maybe_string: Option<String> = None;
    let value = $eq;
    $(
        if let Some(val) = value.downcast_ref::<$inner_value_type>() {
            let value_string = format!("{:?}", val);
            println!("{:?}", value_string );
            maybe_string = Some(value_string);

        }
    )+
    maybe_string
}};
}

#[macro_export]
macro_rules! downcast_struct {
($value:expr, $($struct_type:ty),+)  => {{

        let mut field_vec: Vec<(FieldName, FieldString)> = Vec::new();
        $(
            if let Some(dash_resource) = $value.downcast_ref::<$struct_type>() {
                //
                for (j, inner_value) in dash_resource.iter_fields().enumerate() {
                    //
                    let field_name = dash_resource.name_at(j).unwrap();
                    if let Some(string_value) =
                        // procedural macro to generate the list of Dashboard implemented types
                        attemp_downcasting![inner_value, MyEnum, Nbr] {
                        field_vec.push((field_name.to_string(), string_value));
                    }
                }
            }
        )*
        println!("{:?}", field_vec);
        field_vec
    }
}}

// procedural macro here for resources (globals and other_globals)
pub fn dashboard_variables_setup(
    mut commands: Commands,
    all_vars_res: ResMut<AllDashRes>,
    all_vars_comp: ResMut<AllDashComp>,

    mut comp_query_set: QuerySet<(QueryState<(Entity, &MyComponent), With<DashComponent>>,)>,
    globals: Res<Globals>,
    other_globals: Res<OtherGlobals>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut spawn_res_fields_event: EventWriter<SpawnLabels>,
    // mut spawn_comp_fields_event: EventWriter<SpawnComponentLabels>,
    // ui_res_query: Query<Entity, With<UiBoardResources>>,
    // ui_comp_query: Query<Entity, With<UiBoardComponents>>,
) {
    let ui_board = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(31.0), Val::Percent(95.0)),
                // border: Rect::all(Val::Px(5.0)),
                position: Rect {
                    // top: Val::Px(15.0),
                    left: Val::Percent(2.0),
                    bottom: Val::Percent(2.5),
                    ..Default::default()
                },
                align_self: bevy::ui::AlignSelf::FlexStart,
                ..Default::default()
            },
            material: materials.add(Color::rgba(0.65, 0.25, 0.65, 0.45).into()),
            ..Default::default()
        })
        .id();

    let ui_res_entity = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(95.0), Val::Percent(45.0)),
                // border: Rect::all(Val::Px(5.0)),
                position: Rect {
                    // top: Val::Px(30.0),
                    left: Val::Percent(2.5),
                    bottom: Val::Percent(2.5),
                    // bottom: Val::Px(30.0),
                    ..Default::default()
                },
                align_self: bevy::ui::AlignSelf::FlexStart,
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            material: materials.add(Color::rgba(0.65, 0.1, 0.65, 0.5).into()),
            ..Default::default()
        })
        .insert(UiBoardResources)
        .id();

    commands.entity(ui_board).push_children(&[ui_res_entity]);

    let ui_comp_entity = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(95.0), Val::Percent(45.0)),
                // border: Rect::all(Val::Px(5.0)),
                position: Rect {
                    // top: Val::Px(15.0),
                    // right: Val::Px(15.0),
                    left: Val::Percent(2.5),
                    top: Val::Px(15.0),
                    ..Default::default()
                },
                align_self: bevy::ui::AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            material: materials.add(Color::rgba(0.65, 0.8, 0.75, 0.5).into()),
            ..Default::default()
        })
        .insert(UiBoardComponents)
        .id();

    commands.entity(ui_board).push_children(&[ui_comp_entity]);
    // commands
    //     .entity(ui_board)
    //     .push_children(&[ui_res_entity, ui_comp_entity]);

    //////////////////////////////// resources /////////////////////////
    let all_vars_resources = all_vars_res
        .as_ref()
        .set_values_from_resources(globals.clone(), other_globals.clone());

    let mut struct_field_vec: Vec<(String, FieldString)> = Vec::new();
    //
    for (i, value) in all_vars_resources.iter_fields().enumerate() {
        //
        let mut struct_name = all_vars_resources.name_at(i).unwrap().to_string();
        //
        // procedural macro to generate the list of Dashboard implemented types
        let mut v: Vec<(FieldName, FieldString)> =
            downcast_struct![value, GlobalsNbr, OtherGlobalsNbr];
        v = v
            .iter_mut()
            .map(|(field_name, val_string)| {
                //
                let mut struct_name_temp = struct_name.clone();
                //
                struct_name_temp.push('.');
                struct_name_temp.push_str(field_name);
                //
                (struct_name_temp.to_owned(), val_string.clone())
            })
            .collect::<Vec<(FieldName, FieldString)>>();
        // format!("{:.5}", value.to_string());
        struct_field_vec.append(&mut v);
    }
    spawn_res_fields_event.send(SpawnLabels(struct_field_vec.clone(), ui_res_entity));
    //////////////////////////////// resources /////////////////////////

    //////////////////////////////// components ////////////////////////
    let comp_query = comp_query_set.q0();
    let mut comp_iter = comp_query.iter();
    let (_comp_entity, my_component1) = comp_iter.next().unwrap();
    let (_comp_entity, my_component2) = comp_iter.next().unwrap();
    // let my_component_vec = comp_query_set.q0().iter().map(|(_, comp)| comp).collect::<Vec<_>>();
    let all_vars_components = all_vars_comp
        .as_ref()
        .set_values_from_components(my_component1.clone(), my_component2.clone());

    let mut struct_field_vec: Vec<(String, FieldString)> = Vec::new();
    //
    for (i, value) in all_vars_components.iter_fields().enumerate() {
        //
        let mut struct_name = all_vars_components.name_at(i).unwrap().to_string();
        //
        // procedural macro to generate the list of Dashboard implemented types
        let mut v: Vec<(FieldName, FieldString)> = downcast_struct![value, MyComponentNbr];
        v = v
            .iter_mut()
            .map(|(field_name, val_f64)| {
                //
                let mut struct_name_temp = struct_name.clone();
                //
                struct_name_temp.push('.');
                struct_name_temp.push_str(field_name);
                //
                (struct_name_temp.to_owned(), val_f64.clone())
            })
            .collect::<Vec<(FieldName, FieldString)>>();
        struct_field_vec.append(&mut v);
    }
    spawn_res_fields_event.send(SpawnLabels(struct_field_vec, ui_comp_entity));
    //////////////////////////////// components /////////////////////////
}

// model
pub fn update_dashboard_variables(
    mut globals: ResMut<Globals>,
    mut other_globals: ResMut<OtherGlobals>,
    mut my_component_query: Query<&mut MyComponent, With<DashComponent>>,
    mut events: EventReader<KnobRotated>,
    mut changed_dash_var_event: EventWriter<ChangedDashVar>,
) {
    for KnobRotated(full_name, knob_position) in events.iter() {
        let vec_name = full_name
            .split(".")
            .map(|x| x.to_owned())
            .collect::<Vec<String>>();

        // the following code will need to change when implementing nested fields compatibility
        let struct_name = vec_name.get(0).unwrap().clone();
        let field_name = vec_name.get(1).unwrap().clone();
        let ref_struct_name = struct_name.as_str();

        let value_f64 = *knob_position as f64;

        // let mut my_component = my_component_query.iter_mut().next().unwrap();
        let mut my_comp_iter = my_component_query.iter_mut();
        let mut my_component1 = my_comp_iter.next().unwrap();
        let mut my_component2 = my_comp_iter.next().unwrap();

        #[macro_export]
        macro_rules! route_to_correct_variable {
        ($($dash_struct:expr),+) => {{
            match ref_struct_name {
                $(
                    stringify!($dash_struct) => {
                        let string_value = $dash_struct.modify_and_get_field(&field_name, value_f64);
                        changed_dash_var_event.send(ChangedDashVar(full_name.clone(), string_value))
                    }
                )+
                _ => {}
            }

        }};
        }

        // procedural macro
        route_to_correct_variable![globals, other_globals, my_component1, my_component2];
    }
}

pub fn attach_knob_to_field(
    mut commands: Commands,
    globals: Res<Globals>,
    other_globals: Res<OtherGlobals>,
    mut my_component_query: Query<&mut MyComponent, With<DashComponent>>,
    knob_sprite_query: Query<
        (
            Entity,
            &KnobSprite,
            Option<&LinearKnob<f64>>,
            Option<&LinearKnob<i64>>,
        ),
        With<LinkingFieldToKnob>,
    >,
    // mut knob_query: Query<(Entity, &mut Transform, &mut LinearKnob<f32>), With<LinkingFieldToKnob>>,
    mut new_button_query: Query<
        (Entity, &ButtonId),
        (
            With<Button>,
            With<LinkingFieldToKnob>,
            Without<LinkedWithKnob>,
        ),
    >,
    other_button_query: Query<
        (Entity, &ButtonId),
        (With<LinkedWithKnob>, Without<LinkingFieldToKnob>),
    >,
    // mut button_query: QuerySet<
    //     (QueryState<(Entity, &ButtonId),
    //     (
    //         With<Button>,
    //         With<LinkingFieldToKnob>,
    //         Without<LinkedWithKnob>,
    //     )>,
    //     QueryState<(Entity, &ButtonId), With<LinkedWithKnob>>)
    // >,
) {
    for (button_entity, button_id) in new_button_query.iter_mut() {
        //button_query.q0().iter_mut() {
        for (knob_sprite_entity, knob_sprite, maybe_knob_f64, maybe_knob_i64) in
            knob_sprite_query.iter()
        {
            let full_name = button_id.0.as_str();

            #[macro_export]
            macro_rules! replace_knob {

                ($($value:expr, $ty_of_val:ty, $bounds:expr),*) => {{
                    match full_name {
                        $(
                            stringify!($value) => {
                                let mut new_knob: LinearKnob<$ty_of_val> = LinearKnob::new($value as $ty_of_val);

                                new_knob.set_bounds_and_speed($bounds);
                                new_knob.set_position($value as $ty_of_val);
                                // new_knob.linked_field
                                new_knob.linked_field = Some(button_id.0.to_owned());
                                new_knob.id = knob_sprite.id.clone();
                                println!("attached to {:?}", new_knob.linked_field);

                                // TODO: add assets in bevy_pen_tool_plugin
                                // TODO: make knob go in the right direction on linking
                                // remove the LinkedWithKnob on the button that was previously linked to the knob
                                if let Some(knob) = maybe_knob_f64 {
                                    if let Some(linked_field)  = &knob.linked_field {

                                        for (other_entity, other_button_id) in other_button_query.iter() { //} button_query.q1().iter() {
                                        // if other_button_id.0 == button_id.0 {


                                            // println!("succes f64: {} ----- {}", linked_field,  other_button_id.0.as_str());
                                            if linked_field == other_button_id.0.as_str() {
                                                commands.entity(other_entity).remove::<LinkedWithKnob>();

                                            }
                                        }
                                    }
                                } else if let Some(knob) = maybe_knob_i64 {
                                    if let Some(linked_field)  = &knob.linked_field {
                                        for (other_entity, other_button_id) in other_button_query.iter() { //} button_query.q1().iter() {
                                            if linked_field == other_button_id.0.as_str() {
                                                commands.entity(other_entity).remove::<LinkedWithKnob>();

                                            }
                                        }
                                    }

                                }
                                commands.entity(button_entity).insert(LinkedWithKnob(new_knob.id.clone()));
                                commands.entity(button_entity).remove::<LinkingFieldToKnob>();

                                commands.entity(knob_sprite_entity).remove::<LinearKnob<f64>>();
                                commands.entity(knob_sprite_entity).remove::<LinearKnob<i64>>();

                                commands.entity(knob_sprite_entity).insert(SettingKnobAngleOnce);
                                commands.entity(knob_sprite_entity).insert(new_knob);

                            }
                        )*
                        _ => {}
                    };

                }}
            }

            let mut my_comp_iter = my_component_query.iter_mut();
            let my_component1 = my_comp_iter.next().unwrap();
            let my_component2 = my_comp_iter.next().unwrap();

            // procedural macro
            replace_knob![
                globals.var1,
                f64,
                None,
                globals.var2,
                i64,
                None,
                globals.var3,
                i64,
                None,
                other_globals.var1,
                f64,
                None,
                other_globals.var2,
                f64,
                None,
                other_globals.var3,
                i64,
                None,
                my_component1.y_position,
                f64,
                None,
                my_component1.v2,
                i64,
                None,
                my_component1.v3,
                i64,
                Some((0, (MyEnum::iter().len() - 1) as i64)),
                my_component2.y_position,
                f64,
                None,
                my_component2.v2,
                i64,
                None,
                my_component2.v3,
                i64,
                Some((0, (MyEnum::iter().len() - 1) as i64))
            ];

            // cleaning up
            commands
                .entity(button_entity)
                .remove::<LinkingFieldToKnob>();
            commands
                .entity(knob_sprite_entity)
                .remove::<LinkingFieldToKnob>();
        }
    }
}
