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

impl Globals {
    pub fn modify_field(&mut self, field_name: &str, value_f64: f64) {
        match field_name {
            "var1" => self.var1 = value_f64 as f32,
            "var2" => self.var2 = value_f64 as u16,
            "var3" => self.var3 = value_f64 as i32,
            _ => {}
        }
    }
}

impl OtherGlobals {
    pub fn modify_field(&mut self, field_name: &str, value_f64: f64) {
        match field_name {
            "var1" => self.var1 = value_f64 as i64,
            "var2" => self.var2 = value_f64 as f64,
            "var3" => self.var3 = value_f64 as u8,
            _ => {}
        }
    }
}

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
    $(
        for (i, value) in $y.iter_fields().enumerate() {
            let field_name = $y.name_at(i).unwrap();
            let mut struct_name = stringify!($y).to_string();
            struct_name.push('.');
            struct_name.push_str(field_name);


            if let Some(f64_value) =
                attemp_downcasting![value, u8, u16, u32, i8, i16, i32, f32, f64, MyEnum]
            {
                field_map.insert(struct_name.clone(), f64_value);
                field_vec.push((struct_name, f64_value));
            }
        }
    )*
    (field_map, field_vec)

}}
}

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
