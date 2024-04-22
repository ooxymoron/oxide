use std::{collections::HashMap, ffi::CStr, mem::transmute};

use crate::{
    error::{OxideError, OxideResult},
    o,
    sdk::networkable::{ClassId, PropType, RecvProp},
};

pub mod netvar_dumper;

#[derive(Debug, Clone)]
pub enum NetvarType {
    INT,
    FLOAT,
    VECTOR3,
    VECTOR2,
    STRING,
    BOOL,
    ARRAY(Box<(NetvarType, usize)>),
    INT64,
    OBJECT(HashMap<String, Netvar>),
}
impl NetvarType {
    pub fn from_prop(prop: &RecvProp) -> NetvarType {
        unsafe {
            let name = CStr::from_ptr(prop.var_name).to_str().unwrap().to_string();
            if name.starts_with("m_b") {
                NetvarType::BOOL
            } else {
                match prop.recv_type {
                    PropType::INT => NetvarType::INT,
                    PropType::FLOAT => NetvarType::FLOAT,
                    PropType::VECTOR => NetvarType::VECTOR3,
                    PropType::VECTOR2D => NetvarType::VECTOR2,
                    PropType::STRING => NetvarType::STRING,
                    PropType::ARRAY => {
                        NetvarType::ARRAY(Box::new((NetvarType::INT, prop.elements as usize)))
                    }
                    PropType::DATATABLE => NetvarType::OBJECT(HashMap::new()),
                    PropType::INT64 => NetvarType::INT64,
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Netvar {
    pub netvar_type: NetvarType,
    pub offset: usize,
    pub name: String,
}

pub trait HasNetvars {
    fn get_class_name() -> String;
    fn get_netvar<T, const L: usize>(&self, path: [&str; L]) -> OxideResult<&mut T> {
        macro_rules! err {
            () => {
                OxideError::new("netvar not found")
            };
        }
        let netvars = o!().netvars.get(&Self::get_class_name()).unwrap();
        let mut netvar = netvars.get(path[0]).ok_or(err!())?;
        let mut path = path.into_iter();
        path.next();
        if path.len() > 1 {
            for name in path {
                let NetvarType::OBJECT(netvars) = &netvar.netvar_type else {
                    return Err(err!())
                };
                netvar = netvars.get(name).ok_or(err!())?;
            }
        }
        Ok(unsafe { transmute((self as *const _ as *const u8).byte_add(netvar.offset)) })
    }
}

//PERF: SLOW SLOW SLOW get the offset on init and resuse it
#[macro_export]
macro_rules! define_netvar {
    ($name: ident, $path: expr, $type: ty) => {
        pub fn $name(&self) -> &mut $type {
            self.get_netvar($path).unwrap()
        }
        
    };
}
