use crate::util::{Globals, MyEnum, OtherGlobals};
extern crate dashboard_derive;

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
            "var3" => self.var3 = value_f64.into(),
            _ => {}
        }
    }
}

impl OtherGlobals {
    pub fn modify_field(&mut self, field_name: &str, value_f64: f64) {
        match field_name {
            "var1" => self.var1 = value_f64.into(),
            "var2" => self.var2 = value_f64.into(),
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
    )+
    (field_map, field_vec)

}}
}
