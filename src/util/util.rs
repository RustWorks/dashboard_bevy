#![feature(specialization)]

use std::fmt::Debug;
extern crate dashboard_derive;
use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{pipeline::PipelineDescriptor, renderer::RenderResources},
};

// use bimap::BiMap;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

// use bevy::{
//     prelude::*,
//     reflect::DynamicStruct,
//     render::{
//         camera::OrthographicProjection,
//         pipeline::{PipelineDescriptor, RenderPipeline, RenderPipelines},
//         shader::ShaderStages,
//     },
// };

// use num::traits::Zero;
// use std::any::Any;
// use strum::IntoEnumIterator;
use num::{integer::Integer, Float, Num, NumCast};

//////////////////// LINEAR KNOB ///////////////////
/* A linear knob can either have a continuous domain, in which case
    T is a floating point type, or it can have a discrete domain, in which case
    T is an integer type.
*/

#[derive(Debug, Clone, PartialEq, Component)]
pub struct LinearKnob<T: Num + Copy> {
    pub position: T,
    pub previous_position: T,
    pub bounds: Option<(T, T)>,
    // pub previous_canvas_position: Vec2,
    pub linked_field: Option<String>,

    pub id: KnobId,
    pub speed: f32,
    // pub radius: f32,
    // pub state: KnobState,
}

impl<T: Num + Copy> LinearKnob<T> {
    pub fn new(position: T) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let new_id: u32 = rng.gen();

        LinearKnob {
            position,
            previous_position: position,
            linked_field: None,
            id: new_id,
            bounds: None,
            speed: 1.0,
            // radius: 75.0 * 0.8,
        }
    }
}

impl KnobControl<i64> for LinearKnob<i64> {
    fn set_position(&mut self, value: i64) {
        if let Some(bounds) = self.bounds {
            self.position = value.clamp(bounds.0, bounds.1);
        } else {
            self.position = value;
        }
    }

    fn get_value(&self) -> i64 {
        self.position
    }

    fn remove_bounds(&mut self, maybe_speed: Option<f32>) {
        self.bounds = None;
        if let Some(speed) = maybe_speed {
            self.speed = speed;
        }
    }

    fn set_bounds_and_speed(&mut self, input_bounds: Option<(i64, i64)>) {
        if let Some(bounds) = input_bounds {
            self.bounds = Some(bounds);
        } else {
            let mut bounds = (0, 0);

            bounds.1 = 10_i64.pow((self.position.abs() as f32).log10().ceil() as u32);
            println!("{:?}", bounds.1);

            if self.position < 0 {
                bounds.0 = -bounds.1;
            }
            self.speed = (bounds.1 - bounds.0).abs() as f32 / 10.0;
            self.bounds = Some(bounds);
        }
    }

    // maps the position of the knob to the the angle given the bounds
    fn get_angle(&self) -> f32 {
        let offset = 0.0; //0.92 + std::f32::consts::PI;
        let zero = 0.0; //2.0 * std::f32::consts::PI * 0.1 + offset;
        let one = 1.0; //2.0 * std::f32::consts::PI * 0.9 + offset;
        let range = self.bounds.unwrap().1 - self.bounds.unwrap().0;
        return zero + (one - zero) * (self.position as f32 / range as f32);
    }
}

impl KnobControl<f64> for LinearKnob<f64> {
    fn set_position(&mut self, value: f64) {
        if let Some(bounds) = self.bounds {
            self.position = value.clamp(bounds.0, bounds.1);
        } else {
            self.position = value;
        }
    }

    fn get_value(&self) -> f64 {
        self.position
    }

    fn remove_bounds(&mut self, maybe_speed: Option<f32>) {
        self.bounds = None;
        if let Some(speed) = maybe_speed {
            self.speed = speed;
        }
    }

    fn set_bounds_and_speed(&mut self, input_bounds: Option<(f64, f64)>) {
        if let Some(bounds) = input_bounds {
            self.bounds = Some(bounds);
        } else {
            let mut bounds = (0.0, 0.0);
            if self.position == 0.0 {
                bounds.1 = 100.0;
                self.bounds = Some(bounds);
                return;
            }

            bounds.1 = 10.0_f64.powf(self.position.abs().log10().ceil());
            if self.position < 0.0 {
                bounds.0 = -bounds.1;
            }

            self.speed = (bounds.1 - bounds.0).abs() as f32 / 10.0;
            self.bounds = Some(bounds);
        }
    }

    // fn get_angle(&self) -> f32 {
    //     let offset = 0.92 + std::f32::consts::PI;
    //     let zero = 2.0 * std::f32::consts::PI * 0.1 + offset;
    //     let one = 2.0 * std::f32::consts::PI * 0.9 + offset;
    //     let range = self.bounds.unwrap().1 - self.bounds.unwrap().0;
    //     return zero + (one - zero) * (self.position as f32 / range as f32);
    // }

    // maps the position of the knob to the the angle given the bounds
    fn get_angle(&self) -> f32 {
        // let offset = 0.0; //0.92 + std::f32::consts::PI;
        // let zero = 0.; //2.0 * std::f32::consts::PI * 0.1 + offset;
        // let one = 1.0; //2.0 * std::f32::consts::PI * 0.9 + offset;
        let range = self.bounds.unwrap().1 - self.bounds.unwrap().0;
        return (self.position as f32 - self.bounds.unwrap().0 as f32) / range as f32;
    }
}

pub trait KnobControl<T: Num> {
    fn set_position(&mut self, value: T);
    fn get_value(&self) -> T;
    fn set_bounds_and_speed(&mut self, bounds: Option<(T, T)>);
    fn get_angle(&self) -> f32;
    fn remove_bounds(&mut self, maybe_speed: Option<f32>);
}

// impl KnobControl<i32> for LinearKnob<i32> {
//     fn set_position(&mut self, value: i32) {
//         self.position = value.into();
//     }

//     fn get_value(&self) -> i32 {
//         self.position
//     }
// }

// impl KnobControl<f32> for LinearKnob<f32> {
//     fn set_position(&mut self, value: f32) {
//         self.position = value.into();
//     }

//     fn get_value(&self) -> f32 {
//         self.position
//     }
// }

// impl KnobControl<f64> for LinearKnob<f64> {
//     fn set_position(&mut self, value: f64) {
//         self.position = value.into();
//     }

//     fn get_value(&self) -> f64 {
//         self.position
//     }
// }

//////////////////// LINEAR KNOB ///////////////////

use serde::Serialize;
#[derive(Copy, Clone, PartialEq, EnumIter, Debug, Reflect, Hash, Serialize, PartialOrd)]
#[reflect(Hash, Serialize, PartialEq)]
pub enum MyEnum {
    A,
    B,
    C,
    D,
    F,
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

#[derive(Reflect, Clone, Copy)]
pub enum Nbr {
    Float32(f32),
    Float64(f64),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
}

impl Into<f64> for Nbr {
    fn into(self) -> f64 {
        match self {
            Self::Float32(v) => v as f64, // precision loss
            Self::Float64(v) => v,
            Self::Int8(v) => v.into(),
            Self::Int16(v) => v.into(),
            Self::Int32(v) => v.into(),
            Self::Int64(v) => v as f64, // precision loss
            Self::UInt8(v) => v.into(),
            Self::UInt16(v) => v.into(),
            Self::UInt32(v) => v.into(),
            Self::UInt64(v) => v as f64, // precision loss
        }
    }
}

impl Debug for Nbr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Self::Float32(v) => write!(f, "{}", v),
            // Self::Float64(v) => write!(f, "{}", v),
            // Self::Int8(v) => write!(f, "{}", v),
            // _ => write!(f, "{}", 0),
            Self::Float32(v) => write!(f, "{}", v),
            Self::Float64(v) => write!(f, "{}", v),
            Self::Int8(v) => write!(f, "{}", v),
            Self::Int16(v) => write!(f, "{}", v),
            Self::Int32(v) => write!(f, "{}", v),
            Self::Int64(v) => write!(f, "{}", v),
            Self::UInt8(v) => write!(f, "{}", v),
            Self::UInt16(v) => write!(f, "{}", v),
            Self::UInt32(v) => write!(f, "{}", v),
            Self::UInt64(v) => write!(f, "{}", v),
        }
    }
}
///// Shader parameters
#[derive(TypeUuid, Debug, Clone, RenderResources, Component)]
#[uuid = "1e08866c-0b8a-437e-8bae-38844b21137e"]
#[allow(non_snake_case)]
pub struct KnobShader {
    pub color: Color,
    pub clearcolor: Color,
    pub bounds: Vec2,
    pub hovered: f32,
    pub zoom: f32,
    pub angle: f32,
}

//////////// dummy structs that we want to track with the dashboard /////////////
//
// resource
#[derive(Reflect, Debug, Clone)]
pub struct Globals {
    pub var1: f32,
    pub var2: u16,
    pub var3: i64,
}

// resource
#[derive(Reflect, Debug, Clone)]
pub struct OtherGlobals {
    pub var1: f64,
    pub var2: u64,
    pub var3: u8,
}

// component
#[derive(Reflect, Debug, Clone, Component)]
pub struct MyComponent {
    pub y_position: f32,
    pub v2: u16,
    pub v3: MyEnum,
}

//////////// dummy structs that we want to track with the dashboard /////////////
// pub struct FieldKnobMap(pub BiMap<String, KnobId>);

#[derive(Component)]
pub struct KnobSprite {
    pub id: KnobId,
    pub position: Vec2,
    pub previous_position: Vec2,
    pub radius: f32,
}

#[derive(Component)]
pub struct LinkedWithKnob(pub KnobId);

#[derive(Component)]
pub struct LinkingFieldToKnob;

#[derive(Component)]
pub struct MovingButton;

#[derive(Component)]
pub struct FieldValueText(pub String);

#[derive(Component, Debug, PartialEq, Eq, Hash)]
pub struct ButtonId(pub String);

#[derive(Component)]
pub struct ColorText;

#[derive(Component)]
pub struct UiBoardResources;

#[derive(Component)]
pub struct UiBoardComponents;

#[derive(Component)]
pub struct TranslatingKnob;

#[derive(Component)]
pub struct RotatingKnob;

#[derive(Component)]
pub struct DashComponent;

#[derive(Component)]
pub struct SettingKnobAngleOnce;

// #[derive(Component)]
// pub struct SettingKnobAngleOnce2<T: Num + Copy>(pub LinearKnob<T>);

pub type KnobId = u32;

pub struct ClickedOnKnob(pub KnobId);
pub struct ReleasedOnKnob(pub KnobId);
pub struct SpawnKnobEvent(pub Vec2);

pub struct KnobRotated(pub String, pub f32);
pub struct SpawnLabels(pub Vec<(FieldName, FieldString)>, pub Entity); // entity is either the ui_res_board or ui_comp_board
                                                                       // pub struct SpawnComponentLabels(pub Vec<(FieldName, FieldValue)>, pub Entity);

pub struct ChangedDashVar(pub FieldName, pub FieldString);

pub type FieldName = String;
pub type FieldValue = f64;
pub type FieldString = String;

pub struct ButtonMaterials {
    pub normal: Handle<ColorMaterial>,
    pub hovered: Handle<ColorMaterial>,
    pub pressed: Handle<ColorMaterial>,
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

// #[derive(Debug, Clone, Component)]
// enum Knob<T: PartialOrd + Copy = f32, U: Float = f32> /* defaults to f32 */ {
//     Linear(LinearKnob<T>),
//     Logarithmic(LogarithmicKnob<U>),
// }

// #[derive(Debug)]
// struct AllDashRes<'a> {
//     globals: &'a mut Globals,
//     other_globals: &'a mut OtherGlobals,
// }

// pub trait ReflectOrd<T: PartialOrd + Clone + Default>: Reflect {}

// pub trait DowncastIntoPartialOrd<T: PartialOrd + Reflect + Clone + Default> {
//     fn downcast_into_partial_ord(&self) -> T;
// }

// impl<T: PartialOrd + Reflect + Clone + Default> DowncastIntoPartialOrd<T> for &dyn Reflect {
//     fn downcast_into_partial_ord(&self) -> T {
//         if let Some(val) = self.downcast_ref::<T>() {
//             return val.clone();
//         } else {
//             return T::default();
//         }
//     }
// }

//////////////////// LOGARITHMIC KNOB ///////////////////
// use num::Float;
// #[derive(Debug, Clone, Copy, PartialEq)]
// struct LogarithmicKnob<U: Float> {
//     position: U,
// }

// impl<U: Float> LogarithmicKnob<U> {
//     fn new(val: U) -> Self {
//         Self { position: val }
//     }
// }
// impl<U: Float> KnobControl<U> for LogarithmicKnob<U> {
//     fn set_position(&mut self, value: U) {
//         self.position = value;
//     }

//     fn get_value(&self) -> U {
//         (self.position + num::one()).log2()
//     }
// }
//////////////////// LOGARITHMIC KNOB ///////////////////
