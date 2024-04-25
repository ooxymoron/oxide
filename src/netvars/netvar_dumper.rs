use core::slice;
use std::{collections::HashMap, ffi::CStr, intrinsics::breakpoint};

use crate::{
    sdk::{
        base_client::BaseClient,
        networkable::{ClassId, ClientClass, RecvProp, RecvTable},
    },
    vmt_call,
};

use super::{Netvar, NetvarType};

pub fn load_netvars(client: &BaseClient) -> HashMap<String, HashMap<String, Netvar>> {
    let mut client_class: ClientClass = (*vmt_call!(client, get_all_classes)).clone().into();

    let mut classes = HashMap::new();
    loop {
        let mut netvars = HashMap::new();

        let table = unsafe { client_class.recv_table.read() };
        parse_table(table, 0, &mut netvars);
        classes.insert(client_class.network_name, netvars);

        if client_class.next.is_null() {
            break;
        } else {
            client_class = unsafe { client_class.next.read().into() };
        }
    }
    classes
}
pub fn parse_table(table: RecvTable, super_offset: usize, netvars: &mut HashMap<String, Netvar>) {
    unsafe {
        let mut props = slice::from_raw_parts_mut(table.props, table.props_count as usize).to_vec();
        props.sort_by_key(|prop| prop.offset);
        for prop in props.into_iter() {
            let name = CStr::from_ptr(prop.var_name).to_str().unwrap().to_string();
            let mut netvar = Netvar {
                netvar_type: NetvarType::from_prop(&prop),
                offset: prop.offset as usize + super_offset,
                name: name.clone(),
            };
            if let NetvarType::OBJECT((table_name, child_netvars)) = &mut netvar.netvar_type {
                let data_table = prop.data_table.read();
                *table_name = CStr::from_ptr(data_table.table_name)
                    .to_str()
                    .unwrap()
                    .to_string().replace("DT_", "C");
                parse_table(
                    data_table,
                    prop.offset as usize + super_offset,
                    child_netvars,
                );
            }
            netvars.insert(name.clone(), netvar);
        }
    }
}
