#![feature(specialization)]

use std::fmt::Debug;
extern crate dashboard_derive;
use bevy::{prelude::*, render::pipeline::PipelineDescriptor};

use std::collections::HashMap;

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
// use bimap::BiMap;
// use num::traits::Zero;
// use std::any::Any;
// use strum::IntoEnumIterator;
use num::{integer::Integer, Float, Num, NumCast};

//////////////////// LINEAR KNOB ///////////////////
#[derive(Debug, Clone, PartialEq, Component)]
pub struct LinearKnob<T: Num + Copy> {
    pub position: T,
    pub previous_position: T,
    // pub bounds: (T, T),
    // pub previous_canvas_position: Vec2,
    pub bound_field: Option<String>,

    pub id: KnobId,
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
            bound_field: None,
            id: new_id,
            // radius: 75.0 * 0.8,
        }
    }
}

impl<T: Num + Copy> KnobControl<T> for LinearKnob<T> {
    fn set_position(&mut self, value: T) {
        self.position = value;
    }

    fn get_value(&self) -> T {
        self.position
    }
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

trait KnobControl<T: Num> {
    fn set_position(&mut self, value: T);
    fn get_value(&self) -> T;
}

use serde::Serialize;
#[derive(Copy, Clone, PartialEq, EnumIter, Debug, Reflect, Hash, Serialize, PartialOrd)]
#[reflect(Hash, Serialize, PartialEq)]
pub enum MyEnum {
    A,
    B,
    C,
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

//////////// dummy structs that we want to track with the dashboard /////////////
#[derive(Reflect, Debug, Clone)]
pub struct Globals {
    pub var1: f32,
    pub var2: u16,
    pub var3: i32,
}

#[derive(Reflect, Debug)]
pub struct OtherGlobals {
    pub var1: i64,
    pub var2: f64,
    pub var3: u8,
}
//////////// dummy structs that we want to track with the dashboard /////////////
#[derive(Component)]
pub struct KnobSprite {
    pub id: KnobId,
    pub position: Vec2,
    pub previous_position: Vec2,
    pub radius: f32,
}

#[derive(Component)]
pub struct LinkingFieldToKnob;

#[derive(Component)]
pub struct MovingButton;

#[derive(Component)]
pub struct FieldValueText(pub String);

#[derive(Component, Debug)]
pub struct ButtonId(pub String);

#[derive(Component)]
pub struct ColorText;

#[derive(Component)]
pub struct UiBoard;

#[derive(Component)]
pub struct TranslatingKnob;

#[derive(Component)]
pub struct RotatingKnob;

pub type KnobId = u32;

pub struct ClickedOnKnob(pub KnobId);
pub struct ReleasedOnKnob(pub KnobId);
pub struct SpawnKnobEvent(pub Vec2);
pub struct KnobRotated(pub String, pub f32);
pub struct SpawnFieldLabel(pub Vec<(FieldName, FieldValue)>);
pub struct ChangedDashVar(pub FieldName, pub FieldValue);

pub type FieldName = String;
pub type FieldValue = f64;

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
// struct AllVars<'a> {
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
