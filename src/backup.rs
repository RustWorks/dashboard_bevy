use std::fmt::Debug;
extern crate dashboard_derive;
use bevy::{
    prelude::*,
    reflect::{
        serde::{ReflectDeserializer, ReflectSerializer},
        DynamicStruct, Reflect, TypeRegistry,
    },
};
use dashboard_derive::EnumVariantCount;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub trait Dashable {
    // fn add_to_dashboard(self: Self, x: T);
    fn to_f64s(self) -> Vec<f64>;
    fn enums_to_str(self) -> Vec<Vec<String>>;
}

#[derive(Reflect, Copy, Clone, PartialEq, Serialize, Deserialize, EnumIter, Debug)]
#[reflect_value(PartialEq, Serialize, Deserialize)]
pub enum MyEnum {
    A,
    B,
    C,
}

impl Into<f64> for MyEnum {
    fn into(self) -> f64 {
        // let length = MyEnum::iter().count();
        let mut k = 0;
        let mut my_enum = Self::iter();

        // assumes non-empty enum
        let mut e = my_enum.next().unwrap();

        while e != self {
            e = my_enum.next().unwrap();
            k = k + 1;
        }
        k as f64
    }
}

// // Only useful if we consider empty Enums
// use std::convert::TryInto;
// impl TryInto<f64> for MyEnum {
//     type Error = ();

//     fn try_into(self) -> Result<f64, ()> {
//         // let length = MyEnum::iter().count();
//         let mut k = 0;
//         let mut my_enum = Self::iter();

//         // assumes non-empty enum
//         if let Some(mut e) = my_enum.next() {
//             while e != self {
//                 e = my_enum.next().unwrap();
//                 k = k + 1;
//             }
//             return Ok(k as f64);
//         } else {
//             return Err(());
//         }
//     }
// }

use std::fmt;
impl fmt::Display for MyEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let letter = match self {
            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
        };
        write!(f, "{}", letter)
    }
}

#[derive(Debug, Clone, Reflect)]
pub struct Globals {
    pub var1: f32,
    pub var2: u16,
    pub var3: MyEnum,
}

impl Dashable for Globals {
    fn to_f64s(self) -> Vec<f64> {
        let mut v = Vec::new();
        // v.push(self.var1.into());
        // v.push(self.var2.into());
        // v.push(self.var3.into());

        for e in self.iter_fields() {
            v.push(e.into());
        }

        return v;
    }
}

//     fn enums_to_str(self) -> Vec<Vec<String>> {
//         let mut v: Vec<Vec<String>> = vec![];
//         let mut w: Vec<String> = vec![];
//         for e in MyEnum::iter() {
//             // println!("{:?}", e);
//             w.push(e.to_string());
//         }
//         v.push(w);
//         return v;
//     }
// }

#[derive(Debug)]
enum DashboardItem {
    Float32(f32),
    Int32(i32),
}

#[derive(Debug)]
struct Dashboard {
    items: Vec<DashboardItem>,
}

// could use dynstack for dynamic stack (collection of values with different types)
fn main() {
    let mut d = Dashboard {
        items: vec![DashboardItem::Float32(12f32), DashboardItem::Int32(12i32)],
    };

    let value = Globals {
        var1: 66f32,
        var2: 77u16,
        var3: MyEnum::C,
    };

    let letters = value.clone().enums_to_str();
    println!("{:?}", letters);

    println!("{:?}", value.to_f64s());
}

// this will automatically implement the Reflect trait and the Struct trait (because the type is a struct)
#[derive(Reflect)]
struct Foo {
    a: u32,
    b: Bar,
    c: i32,
    d: Vec<Baz>,
}

// this will automatically implement the Reflect trait and the TupleStruct trait (because the type is a tuple struct)
#[derive(Reflect)]
struct Bar(String);

#[derive(Reflect)]
struct Baz {
    value: f32,
}

fn fr(value: f64) {
    // We will use this value to illustrate `bevy_reflect` features

    // let field_name = foo.name_at(i).unwrap();

    println!(" is a u32 with the value: {}", value);
}

fn qw() {
    let mut foo = Foo {
        a: 1,
        b: Bar("hello".to_string()),
        c: 16,
        d: vec![Baz { value: 3.14 }],
    };

    for (i, value) in foo.iter_fields().enumerate() {
        let mut value2: f64 = -11911.11911;
        if let Some(value) = value.downcast_ref::<f64>() {
            value2 = *value;
        } else if let Some(value) = value.downcast_ref::<u16>() {
            value2 = (*value).into();
        } else if let Some(value) = value.downcast_ref::<i32>() {
            value2 = (*value).into();
        } else if let Some(value) = value.downcast_ref::<u32>() {
            value2 = (*value).into();
        } else if let Some(value) = value.downcast_ref::<i16>() {
            value2 = (*value).into();
        }
        if value2 != -11911.11911 {
            println!(" is a u32 with the value: {}", value2);
        }
    }
}

fn main() {
    qw();
}

fn downcast_any_type_into_f64(value: &dyn Reflect) -> f64 {
    if let Some(value) = value.downcast_ref::<f64>() {
        *value
    } else if let Some(value) = value.downcast_ref::<f32>() {
        (*value).into()
    } else if let Some(value) = value.downcast_ref::<i8>() {
        (*value).into()
    } else if let Some(value) = value.downcast_ref::<i16>() {
        (*value).into()
    } else if let Some(value) = value.downcast_ref::<i32>() {
        (*value).into()
    } else if let Some(value) = value.downcast_ref::<u8>() {
        (*value).into()
    } else if let Some(value) = value.downcast_ref::<u16>() {
        (*value).into()
    } else if let Some(value) = value.downcast_ref::<u32>() {
        (*value).into()
    } else if let Some(value) = value.downcast_ref::<MyEnum>() {
        (*value).into()
    } else {
        // dummy default value
        -11911.11911
    }
}

use std::fmt::Debug;
extern crate dashboard_derive;
use bevy::{
    prelude::*,
    // reflect::{
    //     // serde::{ReflectDeserializer, ReflectSerializer},
    //     DynamicStruct, Reflect, TypeRegistry,
    // },
    reflect::{GetPath, Reflect},
};
use bimap::BiMap;
// use dashboard_derive::EnumVariantCount;
use std::collections::HashMap;

use enum_dispatch::enum_dispatch;

use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, PartialEq)]
struct LinearKnob {
    position: f64,
}
#[derive(Debug, Clone, PartialEq)]
struct LogarithmicKnob {
    position: f64,
}

impl Eq for LogarithmicKnob {}
impl Eq for LinearKnob {}

impl LinearKnob {
    fn new(val: f64) -> Self {
        Self { position: val }
    }
}

impl LogarithmicKnob {
    fn new(val: f64) -> Self {
        Self { position: val }
    }
}

impl KnobControl for LinearKnob {
    fn set_position(&mut self, value: f64) {
        self.position = value;
    }

    fn get_value(&self) -> f64 {
        self.position
    }
}
impl KnobControl for LogarithmicKnob {
    fn set_position(&mut self, value: f64) {
        self.position = value;
    }

    fn get_value(&self) -> f64 {
        (self.position + 1.).log2()
    }
}

#[enum_dispatch(Knob)]
trait KnobControl {
    fn set_position(&mut self, value: f64);
    fn get_value(&self) -> f64;
}

#[enum_dispatch]
#[derive(Debug, PartialEq, Eq)]
enum Knob {
    LinearKnob,
    LogarithmicKnob,
}

#[derive(Reflect, Copy, Clone, PartialEq, Serialize, Deserialize, EnumIter, Debug)]

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

#[derive(Debug, Clone, Reflect)]
pub struct Globals {
    pub var1: f32,
    pub var2: u16,
    pub var3: MyEnum,
}

#[derive(Debug, Clone, Reflect)]
pub struct OtherGlobals {
    pub var1: MyEnum,
    pub var2: f64,
    pub var3: u8,
}

// fn downcast_any_type_that_can_be_converted_into_f64(value: &dyn Reflect) -> f64 {
//     if let Some(value) = value.downcast_ref::<f32>() {
//         (*value).into()
//     }
//     /* ...
//         same conditions but for u8, u16, i8, i16, i32, ...
//     ... */
//     // Even an enum can be converted to f32 using the strum crate
//     else if let Some(value) = value.downcast_ref::<MyEnum>() {
//         (*value).into()
//     } else {
//         0.0 // default value
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

// function that attemps to downcast any type that implements Into<64>
fn downcast_any_type_into_f64<U>(reflect_value: &dyn Reflect) -> Option<f64>
where
    U: Reflect + Into<f64> + Copy,
{
    if let Some(q) = reflect_value.downcast_ref::<U>() {
        let value: f64 = (*q).into();
        Some(value)
    } else {
        None
    }
}

// macro
#[derive(Reflect, Debug)]
struct AllVars {
    globals: Globals,
    other_globals: OtherGlobals,
}

use num_traits::bounds::Bounded;
trait Bounded2<T: Reflect + Copy + Into<f64> + Bounded> {}

fn main() {
    ////////////////// code block in the example code
    let mut globals = Globals {
        var1: 66.66f32,
        var2: 77u16,
        var3: MyEnum::C,
    };
    let mut other_globals = OtherGlobals {
        var1: MyEnum::A,
        var2: 22f64,
        var3: 44u8,
    };
    ////////////////// code block in the example code

    // begin macro loop over dashboard structs
    // macro and construct dash_struct_names in macro as well
    let mut all_dash_vars = AllVars {
        globals: globals,
        other_globals: other_globals,
    };
    let all_dash_vars_len = all_dash_vars.iter_fields().len();

    ////// the following will change completely in the macro
    let mut dash_struct_names = Vec::new();
    for (k, _field) in all_dash_vars.iter_fields().enumerate() {
        let field_name = all_dash_vars.name_at(k).unwrap();
        dash_struct_names.push(field_name);
    }
    ////// end : the following will change completely in the macro

    let mut field_name_map: HashMap<&str, Vec<&str>> = HashMap::new();

    // macro loop over dashboard structs
    let sub_struct = all_dash_vars
        .field("globals")
        .unwrap()
        .downcast_ref::<Globals>()
        .unwrap();

    let mut field_name_vec: Vec<&str> = Vec::new();
    for (k, _sub_fields) in sub_struct.iter_fields().enumerate() {
        let field_name = sub_struct.name_at(k).unwrap();
        field_name_vec.push(field_name);
    }
    field_name_map.insert("globals", field_name_vec);
    //////////////////

    let sub_struct = all_dash_vars
        .field("other_globals")
        .unwrap()
        .downcast_ref::<OtherGlobals>()
        .unwrap();

    // this here is not traversing the whole nested structure (see just below)
    // vec of field names (in macro: and their type)
    let mut field_name_vec: Vec<&str> = Vec::new();
    for (k, _sub_fields) in sub_struct.iter_fields().enumerate() {
        let field_name = sub_struct.name_at(k).unwrap();
        field_name_vec.push(field_name);
    }
    field_name_map.insert("other_globals", field_name_vec);

    println!("field_name_map: {:?}", field_name_map);
    // end macro loop over dashboard structs

    // begin nested structure
    // while we find structs, go deeper in the nested structure
    let mut field_paths: Vec<Vec<&str>> = vec![];
    for struct_name in dash_struct_names {
        let field_names = field_name_map.get(&struct_name).unwrap();
        for field_name in field_names {
            let mut ex_field_path: Vec<&str> = vec![struct_name.clone()];
            ex_field_path.push(field_name);
            field_paths.push(ex_field_path);
        }
    }
    println!("field_paths: {:?}", field_paths);
    let value = *all_dash_vars.get_path::<f32>("globals.var1").unwrap();

    println!("get path: {:?}", value);

    // how to get data
    let struct_data = all_dash_vars
        .field(&field_paths[0][0])
        .unwrap()
        .downcast_ref::<Globals>()
        .unwrap();

    let field_data1 = struct_data
        .field(&field_paths[0][1])
        .unwrap()
        .downcast_ref::<f32>()
        .unwrap();

    println!("fiiiieeelld {:?}", field_data1);
    // end nested structure

    // in example code
    let knob0: Knob = LinearKnob::new(-4.1).into();
    let knob1: Knob = LinearKnob::new(0.1).into();
    let knob2: Knob = LogarithmicKnob::new(0.69).into();
    let knob3: Knob = LogarithmicKnob::new(0.87).into();

    // not sure where this belongs
    let mut knobs: HashMap<u32, Knob> = HashMap::new();
    knobs.insert(0, knob0);
    knobs.insert(1, knob1);
    knobs.insert(2, knob2);
    knobs.insert(3, knob3);
    // end example code

    // inside macro
    // bidirectional map between field names and knob ids
    let mut v = BiMap::new();
    for idx in 0..all_dash_vars_len {
        let struct_name = all_dash_vars.name_at(idx).unwrap().to_string();
        v.insert(struct_name, idx as u32);
    }

    // in the macro, this for loop doesn't exist, each global variable will have it own downcast
    for (i, value) in all_dash_vars.iter_fields().enumerate() {
        //
        let mut maybe_value = downcast_any_type_into_f64::<MyEnum>(value);
        maybe_value = maybe_value.map_or(downcast_any_type_into_f64::<f32>(value), |val| Some(val));
        maybe_value = maybe_value.map_or(downcast_any_type_into_f64::<u32>(value), |val| Some(val));
        maybe_value = maybe_value.map_or(downcast_any_type_into_f64::<u16>(value), |val| Some(val));
        maybe_value = maybe_value.map_or(downcast_any_type_into_f64::<u8>(value), |val| Some(val));
        maybe_value = maybe_value.map_or(downcast_any_type_into_f64::<i32>(value), |val| Some(val));
        maybe_value = maybe_value.map_or(downcast_any_type_into_f64::<i16>(value), |val| Some(val));
        maybe_value = maybe_value.map_or(downcast_any_type_into_f64::<i8>(value), |val| Some(val));

        if let Some(value4) = maybe_value {
            let key = all_dash_vars.name_at(i).unwrap().to_string();
            if let Some(knob_id) = v.get_by_left(&key) {
                let knob = knobs.get_mut(&knob_id).unwrap();
                println!("got a match");
                knob.set_position(value4);
            }
        }
    }

    println!("before {:?}", knobs);

    /////////// emulate user changing the knob position
    let knob_id = 2;
    let knob_value = 1.3333f64;
    knobs
        .get_mut(&knob_id)
        .unwrap() // This unwrap should be in a "let if"
        .set_position(knob_value.into());

    // set global value corresponding to the knob that just moved
    let new_pos = knobs.get_mut(&knob_id).unwrap().get_value();
    let field_to_change = v.get_by_right(&knob_id).unwrap();
    println!("field_to_change : {:?}", field_to_change);

    let aaa = all_dash_vars
        .field_mut(&field_to_change)
        .unwrap()
        .downcast_mut::<MyEnum>() // Macro needs a variable here
        .unwrap();
    *aaa = (new_pos as f64).into();
    /////////// emulate user changing the knob position

    /////////// emulate user changing the knob position
    let knob_id = 0;
    let knob_value = 0.6666f64;
    knobs
        .get_mut(&knob_id)
        .unwrap()
        .set_position(knob_value.into());

    // set global value corresponding to the knob that just moved
    let new_pos = knobs.get_mut(&knob_id).unwrap().get_value();
    let field_to_change = v.get_by_right(&knob_id).unwrap();
    println!("field_to_change : {:?}", field_to_change);

    let aaa = all_dash_vars
        .field_mut(&field_to_change)
        .unwrap()
        .downcast_mut::<f32>() // Macro needs a variable here
        .unwrap();
    *aaa = (new_pos as f32).into(); // this as f32 is not consistent with the above as f64

    /////////// emulate user changing the knob position

    println!("after {:?}", knobs);

    println!("globals : {:?}", all_dash_vars);
}

use std::fmt::Debug;
extern crate dashboard_derive;
use bevy::{
    prelude::*,
    // reflect::{
    //     // serde::{ReflectDeserializer, ReflectSerializer},
    //     DynamicStruct, Reflect, TypeRegistry,
    // },
    reflect::{GetPath, Reflect},
};
use bimap::BiMap;
// use dashboard_derive::EnumVariantCount;
use num::*;
use std::collections::HashMap;

use enum_dispatch::enum_dispatch;

use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[enum_dispatch]
trait KnobControl<T: Num + Clone> {
    fn set_position(&mut self, value: T);
    fn get_value(&self) -> T;
}

//////////////////// LINEAR KNOB ///////////////////
#[derive(Debug, Clone, Copy, PartialEq)]
struct LinearKnob<T: Num> {
    position: T,
}
impl<T: Num> Eq for LinearKnob<T> {}

impl<T: Num + Clone> LinearKnob<T> {
    fn new(val: T) -> Self {
        Self { position: val }
    }
}
impl<T: Num + Copy + Clone> KnobControl<T> for LinearKnob<T> {
    fn set_position(&mut self, value: T) {
        self.position = value;
    }

    fn get_value(&self) -> T {
        self.position
    }
}
//////////////////// LINEAR KNOB ///////////////////

//////////////////// LOGARITHMIC KNOB ///////////////////
#[derive(Debug, Clone, Copy, PartialEq)]
struct LogarithmicKnob<T: Float> {
    position: T,
}
impl<T: Float> Eq for LogarithmicKnob<T> {}

impl<T: Float + Clone> LogarithmicKnob<T> {
    fn new(val: T) -> Self {
        Self { position: val }
    }
}
impl<T: Float + Copy + Clone> KnobControl<T> for LogarithmicKnob<T> {
    fn set_position(&mut self, value: T) {
        self.position = value;
    }

    fn get_value(&self) -> T {
        (self.position + num::one()).log2()
    }
}
//////////////////// LOGARITHMIC KNOB ///////////////////

#[enum_dispatch(KnobControl)]
#[derive(Debug, PartialEq, Eq)]
enum Knob<T: Num = f32, U: Float = f32> /* defaults to f32 */ {
    LinKnob(LinearKnob<T>),
    LogKnob(LogarithmicKnob<U>),
}

#[derive(Reflect, Copy, Clone, PartialEq, Serialize, Deserialize, EnumIter, Debug)]

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

#[derive(Debug, Clone, Reflect)]
pub struct Globals {
    pub var1: f32,
    pub var2: u16,
    pub var3: MyEnum,
}

#[derive(Debug, Clone, Reflect)]
pub struct OtherGlobals {
    pub var1: MyEnum,
    pub var2: f64,
    pub var3: u8,
}

// fn downcast_any_type_that_can_be_converted_into_f64(value: &dyn Reflect) -> f64 {
//     if let Some(value) = value.downcast_ref::<f32>() {
//         (*value).into()
//     }
//     /* ...
//         same conditions but for u8, u16, i8, i16, i32, ...
//     ... */
//     // Even an enum can be converted to f32 using the strum crate
//     else if let Some(value) = value.downcast_ref::<MyEnum>() {
//         (*value).into()
//     } else {
//         0.0 // default value
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

// macro
#[derive(Debug)]
struct AllVars<'a> {
    globals: &'a mut Globals,
    other_globals: &'a mut OtherGlobals,
}

use num_traits::bounds::Bounded;
trait Bounded2<T: Reflect + Copy + Into<f64> + Bounded> {}

fn main() {
    ////////////////// code block in the example code
    let mut globals = Globals {
        var1: 66f32,
        var2: 77u16,
        var3: MyEnum::C,
    };
    let mut other_globals = OtherGlobals {
        var1: MyEnum::A,
        var2: 22f64,
        var3: 44u8,
    };
    ////////////////// code block in the example code

    // the second type parameter does not matter here (only for logarithmic knob)
    // Knob takes two type parameters, but the default on both is f32
    let lin_knob: Knob<u32> = (LinearKnob { position: 30_u32 }).into();
    let log_knob: Knob = Knob::LogKnob(LogarithmicKnob {
        position: 1500.0_f32,
    })
    .into();

    println!("lin_knob {:?}", lin_knob);
    if let Knob::LogKnob(knob) = log_knob {
        println!("log_knob {:?}", knob.get_value());
    }

    // begin macro loop over dashboard structs
    // macro and construct dash_struct_names in macro as well
    let mut all_dash_vars = AllVars {
        globals: &mut globals,
        other_globals: &mut other_globals,
    };

    all_dash_vars.globals.var1 = 4321.0;

    // let all_dash_vars_len = all_dash_vars.iter_fields().len();

    // println!("all_dash_vars {:?}", globals);

    // in example code
    // let knob0: Knob = LinearKnob::new(&mut all_dash_vars.globals.var1).into();
    let mut knob0: LinearKnob<f32> = LinearKnob::new(all_dash_vars.globals.var1);
    knob0.set_position(0.2f32);

    // let knob1: Knob = Knob::LinearKnob::new(0.1f32);
    // let knob2: Knob = LogarithmicKnob::new(0.69f32).into();
    // let knob3: Knob = LogarithmicKnob::new(0.87f32).into();

    // let mut knobs: HashMap<u32, Knob> = HashMap::new();
    // knobs.insert(0, knob0);
    // knobs.insert(1, knob1);
    // knobs.insert(2, knob2);
    // knobs.insert(3, knob3);
}

use std::fmt::Debug;
extern crate dashboard_derive;
use bevy::{
    prelude::*,
    // reflect::{
    //     // serde::{ReflectDeserializer, ReflectSerializer},
    //     DynamicStruct, Reflect, TypeRegistry,
    // },
    reflect::{GetPath, Reflect},
};
use bimap::BiMap;
// use dashboard_derive::EnumVariantCount;
use num::*;
use std::collections::HashMap;

use enum_dispatch::enum_dispatch;

use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[enum_dispatch]
trait KnobControl<T: Num> {
    fn set_position(&mut self, value: T);
    fn get_value(&self) -> T;
}

use std::marker::PhantomData;
//////////////////// LINEAR KNOB ///////////////////
#[derive(Debug, Clone, Copy, PartialEq)]
struct LinearKnob<T: Num, U: Float> {
    position: T,
    phantom: PhantomData<U>,
}
impl<T: Num + Copy, U: Float> Eq for LinearKnob<T, U> {}

impl<T: Num + Copy, U: Float> LinearKnob<T, U> {
    fn new(val: T) -> Self {
        Self {
            position: val,
            phantom: PhantomData,
        }
    }
}
impl<T: Num + Copy, U: Float> KnobControl<T> for LinearKnob<T, U> {
    fn set_position(&mut self, value: T) {
        self.position = value;
    }

    fn get_value(&self) -> T {
        self.position
    }
}
//////////////////// LINEAR KNOB ///////////////////

//////////////////// LOGARITHMIC KNOB ///////////////////
#[derive(Debug, Clone, Copy, PartialEq)]
struct LogarithmicKnob<T, U: Float> {
    position: U,
    phantom: PhantomData<T>,
}
impl<T: Num, U: Float> Eq for LogarithmicKnob<T, U> {}

impl<T: Num, U: Float> LogarithmicKnob<T, U> {
    fn new(val: U) -> Self {
        Self {
            position: val,
            phantom: PhantomData,
        }
    }
}
impl<T: Num, U: Float> KnobControl<U> for LogarithmicKnob<T, U> {
    fn set_position(&mut self, value: U) {
        self.position = value;
    }

    fn get_value(&self) -> U {
        (self.position + num::one()).log2()
    }
}
//////////////////// LOGARITHMIC KNOB ///////////////////

// #[enum_dispatch(KnobControl<T,U>)]
// #[derive(Debug, PartialEq, Eq)]
// enum Knob<T: Num = f32, U: Float = f32> /* defaults to f32 */ {
//     Linear(LinearKnob<T>),
//     Logarithmic(LogarithmicKnob<U>),
// }

#[enum_dispatch(KnobControl<T, U>)]
enum Knob<T: Num, U: Float> /* defaults to f32 */ {
    LinearKnob(LinearKnob<T, U>),
    LogarithmicKnob(LinearKnob<T, U>),
}

fn some_existing_knobs() -> (LinearKnob<f32, f32>, LogarithmicKnob<f32, f32>) {
    (
        LinearKnob {
            position: 0.5,
            phantom: PhantomData,
        },
        LogarithmicKnob {
            position: 0.5,
            phantom: PhantomData,
        },
    )
}

#[derive(Reflect, Copy, Clone, PartialEq, Serialize, Deserialize, EnumIter, Debug)]

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

#[derive(Debug, Clone, Reflect)]
pub struct Globals {
    pub var1: f32,
    pub var2: u16,
    pub var3: MyEnum,
}

#[derive(Debug, Clone, Reflect)]
pub struct OtherGlobals {
    pub var1: MyEnum,
    pub var2: f64,
    pub var3: u8,
}

// fn downcast_any_type_that_can_be_converted_into_f64(value: &dyn Reflect) -> f64 {
//     if let Some(value) = value.downcast_ref::<f32>() {
//         (*value).into()
//     }
//     /* ...
//         same conditions but for u8, u16, i8, i16, i32, ...
//     ... */
//     // Even an enum can be converted to f32 using the strum crate
//     else if let Some(value) = value.downcast_ref::<MyEnum>() {
//         (*value).into()
//     } else {
//         0.0 // default value
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

// macro
#[derive(Debug)]
struct AllVars<'a> {
    globals: &'a mut Globals,
    other_globals: &'a mut OtherGlobals,
}

use num_traits::bounds::Bounded;
trait Bounded2<T: Reflect + Copy + Into<f64> + Bounded> {}

fn main() {
    ////////////////// code block in the example code
    let mut globals = Globals {
        var1: 66f32,
        var2: 77u16,
        var3: MyEnum::C,
    };
    let mut other_globals = OtherGlobals {
        var1: MyEnum::A,
        var2: 22f64,
        var3: 44u8,
    };
    ////////////////// code block in the example code

    // the second type parameter does not matter here (only for logarithmic knob)
    // Knob takes two type parameters, but the default on both is f32
    // let lin_knob: Knob = (LinearKnob { position: 30_f32 }).into();

    // let lin_knob: LinearKnob<f32> = LinearKnob { position: 30_f32 };
    // let log_knob: Knob = Knob::Logarithmic(LogarithmicKnob {
    //     position: 1500.0_f32,
    // });

    // let log_knob: Knob<f32, f32> = (Knob::LogarithmicKnob(LogarithmicKnob {
    //     position: 1500.0_f32,
    // }))
    // .into();

    // .into();

    // println!("lin_knob {:?}", lin_knob);
    // if let Knob::LogarithmicKnob(knob) = log_knob {
    //     println!("log_knob {:?}", knob.get_value());
    // }

    // begin macro loop over dashboard structs
    // macro and construct dash_struct_names in macro as well
    let mut all_dash_vars = AllVars {
        globals: &mut globals,
        other_globals: &mut other_globals,
    };

    all_dash_vars.globals.var1 = 4321.0;

    // let all_dash_vars_len = all_dash_vars.iter_fields().len();

    // println!("all_dash_vars {:?}", globals);

    // in example code
    // let knob0: Knob = LinearKnob::new(&mut all_dash_vars.globals.var1).into();
    // let mut knob0: LinearKnob<f32> = LinearKnob::new(all_dash_vars.globals.var1);
    // knob0.set_position(0.2f32);

    // let knob1: Knob = Knob::LinearKnob::new(0.1f32);
    // let knob2: Knob = LogarithmicKnob::new(0.69f32).into();
    // let knob3: Knob = LogarithmicKnob::new(0.87f32).into();

    // let mut knobs: HashMap<u32, Knob> = HashMap::new();
    // knobs.insert(0, knob0);
    // knobs.insert(1, knob1);
    // knobs.insert(2, knob2);
    // knobs.insert(3, knob3);
}

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
                size: Size::new(Val::Percent(31.0), Val::Percent(50.0)),
                border: Rect::all(Val::Px(5.0)),
                align_self: bevy::ui::AlignSelf::FlexEnd,
                ..Default::default()
            },
            material: materials.add(Color::rgba(0.65, 0.65, 0.65, 0.5).into()),
            ..Default::default()
        })
        .insert(UiBoard);
}

struct SpawnFieldLabel([String; 2]);

fn spawn_text_label(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut text_event_reader: EventReader<SpawnFieldLabel>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ui_query: Query<Entity, With<UiBoard>>,
) {
    let num_fields = text_event_reader.iter().count();
    println!("{:?}", num_fields);
    for (k, event) in text_event_reader.iter().enumerate() {
        let ui_entity = ui_query.single();

        let text_content = event.0.clone();
        let key = text_content[0].clone();
        let value = text_content[1].clone();
        let height = k as f32 * 30.0;
        // let height = k as f32 * 100.0 / num_fields as f32;

        let offset = 5.0;

        let button_entity = commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(30.0), Val::Percent(15.0)),
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Relative,
                    position: Rect {
                        bottom: Val::Percent(10.0),
                        right: Val::Percent(10.0),
                        ..Default::default()
                    },

                    ..Default::default()
                },
                visible: bevy::render::draw::Visible {
                    is_visible: true,
                    is_transparent: false,
                },

                //  Style {
                //     size: Size::new(Val::Percent(31.0), Val::Percent(50.0)),
                //     border: Rect::all(Val::Px(5.0)),
                //     align_self: bevy::ui::AlignSelf::FlexEnd,
                //     ..Default::default()
                // },
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
                        bottom: Val::Percent(50.0),
                        right: Val::Percent(50.0),
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
                        bottom: Val::Px(height + offset),
                        right: Val::Px(offset),
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
            let mut maybe_value_f64: Option<f64> = None;
            $(
                for (i, value) in $y.iter_fields().enumerate() {
                    let field_name = globals.name_at(i).unwrap();
                    let mut struct_name = stringify!($y).to_string();
                    struct_name.push('.');
                    struct_name.push_str(field_name);

                    maybe_value_f64 = attemp_downcasting![value, u8, u16, u32, i8, i16, i32, f32, f64, MyEnum];
                    if let Some(f64_value) = maybe_value_f64 {
                        field_map.insert(struct_name, f64_value);
                    }
                }
            )+
            field_map

        }};
    }

    let field_map: HashMap<String, f64> = add_to_dashboard_variables!(globals, other_globals);
    for (key, value) in field_map.iter() {
        let mut temp_key = key.clone();
        let value_string = format!("{:.4}", (*value).to_string());
        // temp_key.push_str(": ");
        // temp_key.push_str(&value_string);
        text_event.send(SpawnFieldLabel([temp_key, value_string]));
    }
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

fn attach_knob_to_field(
    mut commands: Commands,
    globals: Res<Globals>,
    other_globals: Res<OtherGlobals>,
    mut knob_query: Query<&mut LinearKnob<f32>>,
    mut button_query: Query<(Entity, &ButtonId), (With<Button>, With<LinkingFieldToKnob>)>,
    // mut released_on_knob_event_writer: EventReader<ReleasedOnKnob>,
) {
    // for knob_id in released_on_knob_event_writer.iter() {
    for mut knob in knob_query.iter_mut() {
        for (entity, button_id) in button_query.iter_mut() {
            knob.bound_field = Some(button_id.0.to_owned());
            println!("attached to {:?}", knob.bound_field);
            // get field here
            let comparator = button_id.0.as_str();
            let value: f64 = match comparator {
                "globals.var1" => globals.var1.into(),
                "globals.var2" => globals.var2.into(),
                "globals.var3" => globals.var3.into(),
                "other_globals.var1" => other_globals.var1.into(),
                "other_globals.var2" => other_globals.var2.into(),
                "other_globals.var3" => other_globals.var3.into(),
                _ => 0.0,
            };
            knob.position = value as f32;
            knob.previous_position = value as f32;
            commands.entity(entity).remove::<LinkingFieldToKnob>();
        }
    }
    // }
}

fn attach_knob_to_field(
    globals: Res<Globals>,
    other_globals: Res<OtherGlobals>,
    mut knob_query: Query<&mut LinearKnob<f32>>,
    mut button_query: Query<&ButtonId, With<Button>>,
    mut released_on_knob_event_writer: EventReader<ReleasedOnKnob>,
) {
    for knob_id in released_on_knob_event_writer.iter() {
        for mut knob in knob_query.iter_mut() {
            if knob_id.0 == knob.id {
                for button_id in button_query.iter_mut() {
                    // if let &ButtonPhantomState::Moving = button_state.as_ref() {
                    knob.bound_field = Some(button_id.0.to_owned());
                    println!("attached to {:?}", knob.bound_field);
                    // get field here
                    let comparator = button_id.0.as_str();
                    let value: f64 = match comparator {
                        "globals.var1" => globals.var1.into(),
                        "globals.var2" => globals.var2.into(),
                        "globals.var3" => globals.var3.into(),
                        "other_globals.var1" => other_globals.var1.into(),
                        "other_globals.var2" => other_globals.var2.into(),
                        "other_globals.var3" => other_globals.var3.into(),
                        _ => 0.0,
                    };
                    knob.position = value as f32;
                    knob.previous_position = value as f32;
                }
            }
        }
    }
}
