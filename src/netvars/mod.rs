use std::{collections::HashMap, ffi::CStr};

use crate::{
    error::{OxideError, OxideResult},
    o,
    sdk::networkable::{PropType, RecvProp},
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
    OBJECT((String, HashMap<String, Netvar>)),
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
                    PropType::DATATABLE => NetvarType::OBJECT(("".to_string(), HashMap::new())),
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
    fn get_class_name() -> &'static str;
    fn get_netvar<const L: usize>(&self, path: [&str; L]) -> OxideResult<Netvar> {
        macro_rules! err {
            () => {
                OxideError::new("netvar not found")
            };
        }
        let netvars = o!().netvars.get(Self::get_class_name()).unwrap();
        let mut path = path.into_iter();
        let mut netvar = netvars.get(path.next().unwrap()).ok_or(err!())?;
        for name in path {
            let NetvarType::OBJECT((_,netvars)) = &netvar.netvar_type else {
                return Err(err!())
            };
            netvar = netvars.get(name).ok_or(err!())?;
        }
        Ok(netvar.clone())
    }
}

//PERF: SLOW SLOW SLOW get the offset on init and resuse it
#[macro_export]
macro_rules! define_netvar {
    ($name: ident, $path: expr, $type: ty) => {
        pub fn $name(&self) -> &mut $type {
            use std::mem::transmute;
            let netvar = self.get_netvar($path).unwrap();
            unsafe { transmute((self as *const _ as *const u8).byte_add(netvar.offset)) }
        }
    };
}
