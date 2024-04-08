use std::{
    alloc::{alloc, Layout},
    error::Error,
    ffi::CString,
    mem::transmute,
    usize,
};

use libc::{c_void, dlsym};

use crate::{
    cfn,
    sdk::{
        base_client::{BaseClient, VMTBaseClient}, base_engine::{BaseEngine, VMTBaseEngine}, client_mode::{ClientMode, VMTClientMode}, cvar::{CVar, VMTCVar}, engine_trace::{EngineTrace, VMTEngineTrace}, engine_vgui::{EngineVgui, VMTEngineVgui}, entity_list::{EntityList, VMTEntityList}, game_movement::{GameMovement, VMTGameMovement}, input::{Input, VMTInput}, mat_surface::{Surface, VMTMatSurface}, material_system::{MaterialSystem, VMTMaterialSystem}, model_info::{ModelInfo, VMTModelInfo}, model_render::{ModelRender, VMTModelRender}, panel::{Panel, VMTPanel}, predictions::{Prediction, VMTPrediction}, render_view::{RenderView, VMTRenderView}, HasVmt
    },
    util::{get_handle, sigscanner::find_sig, vmt_size},
};

#[derive(Debug, Clone)]
pub struct Interface<T: HasVmt<V> + 'static, V: 'static> {
    pub interface_ref: *mut T,
    pub old_vmt: *mut V,
}
impl<T: HasVmt<V>, V> Interface<T, V> {
    pub unsafe fn new(interface_ref: &'static mut T) -> Interface<T, V> {
        let old = (*interface_ref).get_vmt();
        let size = vmt_size(transmute(old));

        let layout = Layout::from_size_align(size, 8).unwrap();
        let new: &'static mut V = transmute(alloc(layout));

        libc::memcpy(transmute(&mut *new), transmute(old), size);
        (*interface_ref).set_vmt(new);

        Interface {
            interface_ref,
            old_vmt: (old as *const _ as *mut V),
        }
    }
    unsafe fn create(
        handle: *mut c_void,
        name: &str,
    ) -> Result<Interface<T, V>, std::boxed::Box<dyn Error>> {
        let create_interface_fn: cfn!(*const c_void, *const i8, *const isize) =
            std::mem::transmute(dlsym(handle, CString::new("CreateInterface")?.as_ptr()));

        let name = CString::new(name).unwrap();
        let interface_ref: &'static mut T =
            transmute(create_interface_fn(name.as_ptr(), std::ptr::null()));

        Ok(Interface::new(interface_ref))
    }
}

impl<T: HasVmt<V>, V> Interface<T, V> {
    pub fn get_vmt(&self) -> &'static V {
        unsafe { (*self.interface_ref).get_vmt() }
    }
    fn restore(&mut self) {
        unsafe {
            (*self.interface_ref).set_vmt(self.old_vmt);
        }
    }
    pub fn interface_ref(&self) -> &'static mut T {
        unsafe { &mut *self.interface_ref }
    }
}

#[derive(Debug, Clone)]
pub struct Interfaces {
    pub base_client: Interface<BaseClient, VMTBaseClient>,
    pub base_engine: Interface<BaseEngine, VMTBaseEngine>,
    pub entity_list: Interface<EntityList, VMTEntityList>,
    pub engine_vgui: Interface<EngineVgui, VMTEngineVgui>,
    pub cvar: Interface<CVar, VMTCVar>,
    pub surface: Interface<Surface, VMTMatSurface>,
    pub panel: Interface<Panel, VMTPanel>,
    pub model_info: Interface<ModelInfo, VMTModelInfo>,
    pub render_view: Interface<RenderView, VMTRenderView>,
    pub engine_trace: Interface<EngineTrace, VMTEngineTrace>,
    pub material_system: Interface<MaterialSystem, VMTMaterialSystem>,
    pub model_render: Interface<ModelRender, VMTModelRender>,
    pub game_movement: Interface<GameMovement, VMTGameMovement>,
    pub prediction: Interface<Prediction, VMTPrediction>,
    pub client_mode: Interface<ClientMode, VMTClientMode>,
    pub input: Interface<Input, VMTInput>,
}
impl Interfaces {
    pub unsafe fn init() -> Result<Interfaces, std::boxed::Box<dyn Error>> {
        let client_handle = get_handle("./tf/bin/client.so")?;
        let engine_handle = get_handle("./bin/engine.so")?;
        let matsurface_handle = get_handle("./bin/vguimatsurface.so")?;
        let vgui_handle = get_handle("./bin/vgui2.so")?;
        let materialsystem_handle = get_handle("./bin/materialsystem.so")?;
        let vstdlib_handle = get_handle("./bin/libvstdlib.so")?;
        let base_client: Interface<BaseClient, VMTBaseClient> =
            Interface::create(client_handle, "VClient017")?;

        let client_mode = Interfaces::get_client_mode(base_client.interface_ref());

        let input = find_sig(
            "./tf/bin/client.so",
            "A1 ? ? ? ? C6 05 ? ? ? ? ? 8B 10 89 04 24 FF 92 ? ? ? ? A1",
        ) as usize
            + 1;
        let input = transmute(**(input as *const *const *const Input)); 
        Ok(Interfaces {
            base_client,
            base_engine: Interface::create(engine_handle, "VEngineClient014")?,
            entity_list: Interface::create(client_handle, "VClientEntityList003")?,
            engine_vgui: Interface::create(engine_handle, "VEngineVGui002")?,
            cvar: Interface::create(vstdlib_handle, "VEngineCvar004")?,
            surface: Interface::create(matsurface_handle, "VGUI_Surface030")?,
            panel: Interface::create(vgui_handle, "VGUI_Panel009")?,
            model_info: Interface::create(engine_handle, "VModelInfoClient006")?,
            render_view: Interface::create(engine_handle, "VEngineRenderView014")?,
            engine_trace: Interface::create(engine_handle, "EngineTraceClient003")?,
            material_system: Interface::create(materialsystem_handle, "VMaterialSystem081")?,
            model_render: Interface::create(engine_handle, "VEngineModel016")?,
            game_movement: Interface::create(client_handle, "GameMovement001")?,
            prediction: Interface::create(client_handle, "VClientPrediction001")?,
            client_mode: Interface::new(client_mode),
            input: Interface::new(input)
        })
    }

    unsafe fn get_client_mode(base_client: &BaseClient) -> &'static mut ClientMode {
        **transmute::<usize, &'static mut &'static mut &'static mut ClientMode>(
            (*base_client.vmt).hud_process_input as usize + 1,
        )
    }
    pub fn restore(&mut self) {
        self.base_client.restore();
        self.base_engine.restore();
        self.entity_list.restore();
        self.engine_vgui.restore();
        self.cvar.restore();
        self.surface.restore();
        self.panel.restore();
        self.model_info.restore();
        self.render_view.restore();
        self.engine_trace.restore();
        self.material_system.restore();
        self.model_render.restore();
        self.game_movement.restore();
        self.prediction.restore();
        self.client_mode.restore();
    }
}
