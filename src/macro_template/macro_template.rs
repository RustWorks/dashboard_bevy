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
pub struct AllVars {
    pub globals: GlobalsNbr,
    pub other_globals: OtherGlobalsNbr,
}

impl AllVars {
    // pub fn new(globals: Globals, other_globals: OtherGlobals) -> Self {
    // Self {
    //     globals: GlobalsNbr {
    //         var1: Nbr::Float32(globals.var1),
    //         var2: Nbr::UInt16(globals.var2),
    //         var3: Nbr::Int64(globals.var3),
    //     },
    //     other_globals: OtherGlobalsNbr {
    //         var1: Nbr::Float64(other_globals.var1),
    //         var3: Nbr::UInt64(other_globals.var2),
    //         var2: Nbr::UInt8(other_globals.var3),
    //     },

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

impl Globals {
    pub fn modify_field(&mut self, field_name: &str, value_f64: f64) {
        match field_name {
            "var1" => self.var1 = value_f64 as f32,
            "var2" => self.var2 = value_f64 as u16,
            "var3" => self.var3 = value_f64 as i64,
            _ => {}
        }
    }
}

impl OtherGlobals {
    pub fn modify_field(&mut self, field_name: &str, value_f64: f64) {
        match field_name {
            "var1" => self.var1 = value_f64 as f64,
            "var2" => self.var2 = value_f64 as u64,
            "var3" => self.var3 = value_f64 as u8,
            _ => {}
        }
    }
}

// struct dyn DashboardResource;

// #[macro_export]
// macro_rules! create_dashboard_resource {
// ($($y:expr),*) => {{
//     let mut dashboard_resource = DashboardResource::new();
//     $(
//         dashboard_resource.add_field($y);
//     )*
//     dashboard_resource
// }};
// }

// pub fn pre_setup(mut commands: Commands, globals: Res<Globals>, other_globals: Res<OtherGlobals>) {
//     let all_vars = AllVars::new(globals.clone(), other_globals.clone());
//     commands.insert_resource(all_vars);
// }

// HEEEERE

// if let Some(dash_resource) = value.downcast_ref::<yyy>() {
//     for (j, inner_value) in dash_resource.iter_fields().enumerate() {
//         let field_name = dash_resource.name_at(j).unwrap();

//         // let mut field_name = stringify!(all_vars).to_string();
//         println!("{}", field_name);
//         // println!("{}", struct_name);
//     }
// }

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
macro_rules! downcast_struct {
($value:expr, $($struct_type:ty),+)  => {{

        let mut field_vec: Vec<(FieldName, FieldValue)> = Vec::new();
        $(
            if let Some(dash_resource) = $value.downcast_ref::<$struct_type>() {
                for (j, inner_value) in dash_resource.iter_fields().enumerate() {
                    let field_name = dash_resource.name_at(j).unwrap();
                    // struct_name.push('.');
                    // struct_name.push_str(field_name);
                    if let Some(f64_value) =
                        // procedural macro to generate the list of Dashboard implemented types
                        attemp_downcasting![inner_value, MyEnum, Nbr] {
                        field_vec.push((field_name.to_string(), f64_value));
                    }
                }
            }
        )*
        println!("{:?}", field_vec);
        field_vec
    }
}}

pub fn add_2_dashboard(all_vars: AllVars, mut spawn_fields_event: EventWriter<SpawnFieldLabel>) {
    let mut struct_field_vec: Vec<(String, FieldValue)> = Vec::new();
    // let mut v: Vec<(FieldName, FieldValue)> = Vec::new();
    //
    for (i, value) in all_vars.iter_fields().enumerate() {
        //
        let mut struct_name = all_vars.name_at(i).unwrap().to_string();
        //
        // procedural macro to generate the list of Dashboard implemented types
        let mut v: Vec<(FieldName, FieldValue)> =
            downcast_struct![value, GlobalsNbr, OtherGlobalsNbr];
        v = v
            .iter_mut()
            .map(|(field_name, val_f64)| {
                let mut struct_name_temp = struct_name.clone();
                struct_name_temp.push('.');
                struct_name_temp.push_str(field_name);
                (struct_name_temp.to_owned(), val_f64.clone())
            })
            .collect::<Vec<(FieldName, FieldValue)>>();
        struct_field_vec.append(&mut v);
    }
    spawn_fields_event.send(SpawnFieldLabel(struct_field_vec));
}

// #[macro_export]
// macro_rules! add_to_dashboard_variables {
// ($($y:expr),*) => {{
//     let mut field_map: HashMap<String, f64> = HashMap::new();
//     let mut field_vec: Vec<(String, f64)> = Vec::new();
//     $(
//         for (i, value) in $y.iter_fields().enumerate() {
//             let field_name = $y.name_at(i).unwrap();
//             let mut struct_name = stringify!($y).to_string();
//             struct_name.push('.');
//             struct_name.push_str(field_name);

//             if let Some(f64_value) =
//                 attemp_downcasting![value, u8, u16, u32, i8, i16, i32, f32, f64, MyEnum, Nbr]
//             {
//                 field_map.insert(struct_name.clone(), f64_value);
//                 field_vec.push((struct_name, f64_value));
//             }
//         }
//     )*
//     (field_map, field_vec)

// }}
// }

pub fn update_dashboard_variables(
    mut globals: ResMut<Globals>,
    mut other_globals: ResMut<OtherGlobals>,
    mut events: EventReader<KnobRotated>,
    mut changed_dash_var_event: EventWriter<ChangedDashVar>,
) {
    for KnobRotated(full_name, knob_position) in events.iter() {
        // println!("field_name : {:?}", struct_info.0);

        let vec_name = full_name
            .split(".")
            .map(|x| x.to_owned())
            .collect::<Vec<String>>();

        // the following code will need to change when implementing nested fields compatibility
        let struct_name = vec_name.get(0).unwrap().clone();
        let field_name = vec_name.get(1).unwrap().clone();
        let ref_struct_name = struct_name.as_str();

        let value_f64 = *knob_position as f64;

        match ref_struct_name {
            "globals" => {
                globals.modify_field(&field_name, value_f64);
                changed_dash_var_event.send(ChangedDashVar(full_name.clone(), value_f64))
            }

            "other_globals" => {
                other_globals.modify_field(&field_name, value_f64);
                changed_dash_var_event.send(ChangedDashVar(full_name.clone(), value_f64))
            }
            _ => {}
        }
    }
}

pub fn attach_knob_to_field(
    mut commands: Commands,
    globals: Res<Globals>,
    other_globals: Res<OtherGlobals>,
    knob_sprite_query: Query<(Entity, &KnobSprite), With<LinkingFieldToKnob>>,
    // mut knob_query: Query<(Entity, &mut Transform, &mut LinearKnob<f32>), With<LinkingFieldToKnob>>,
    mut button_query: Query<(Entity, &ButtonId), (With<Button>, With<LinkingFieldToKnob>)>,
    // mut released_on_knob_event_writer: EventReader<ReleasedOnKnob>,
) {
    // for knob_id in released_on_knob_event_writer.iter() {
    // for (knob_entity, mut knob_transform, mut knob) in knob_query.iter_mut() {
    for (button_entity, button_id) in button_query.iter_mut() {
        for (knob_sprite_entity, knob_sprite) in knob_sprite_query.iter() {
            let full_name = button_id.0.as_str();

            #[macro_export]
            macro_rules! replace_knob {
                    ($($yy:expr, $xx:ty),*) => {{
                        match full_name {
                            $(
                                stringify!($yy) => {
                                    let mut new_knob: LinearKnob<$xx> = LinearKnob::new($yy as $xx);
                                    new_knob.bound_field = Some(button_id.0.to_owned());
                                    new_knob.id = knob_sprite.id.clone();
                                    println!("attached to {:?}", new_knob.bound_field);
                                    commands.entity(knob_sprite_entity).insert(new_knob);

                                }
                            )*
                            _ => {}
                        };
                    }}
                }

            replace_knob![
                globals.var1,
                f64,
                globals.var2,
                i64,
                globals.var3,
                i64,
                other_globals.var1,
                i64,
                other_globals.var2,
                f64,
                other_globals.var3,
                i64
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

// pub fn attach_knob_to_field(
//     mut commands: Commands,
//     globals: Res<Globals>,
//     other_globals: Res<OtherGlobals>,
//     mut knob_query: Query<(Entity, &mut Transform, &mut LinearKnob<f32>), With<LinkingFieldToKnob>>,
//     mut button_query: Query<(Entity, &ButtonId), (With<Button>, With<LinkingFieldToKnob>)>,
//     // mut released_on_knob_event_writer: EventReader<ReleasedOnKnob>,
// ) {
//     // for knob_id in released_on_knob_event_writer.iter() {
//     for (knob_entity, mut knob_transform, mut knob) in knob_query.iter_mut() {
//         for (button_entity, button_id) in button_query.iter_mut() {
//             // get field here
//             let full_name = button_id.0.as_str();
//             let value: f64 = match full_name {
//                 "globals.var1" => globals.var1.into(),
//                 "globals.var2" => globals.var2.into(),
//                 "globals.var3" => globals.var3.into(),
//                 "other_globals.var1" => other_globals.var1.into(),
//                 "other_globals.var2" => other_globals.var2.into(),
//                 "other_globals.var3" => other_globals.var3.into(),
//                 _ => 0.0,
//             };

//             knob.bound_field = Some(button_id.0.to_owned());
//             knob.position = value as f32;
//             knob_transform.rotation = Quat::from_rotation_z(knob.position);
//             knob.previous_position = value as f32;

//             // let new_knob = LinearKnob<>

//             println!("attached to {:?}", knob.bound_field);
//             // cleaning up
//             commands
//                 .entity(button_entity)
//                 .remove::<LinkingFieldToKnob>();
//             commands.entity(knob_entity).remove::<LinkingFieldToKnob>();
//         }
//     }
//     // }
// }
