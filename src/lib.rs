use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use rrplug::prelude::*;
use rrplug::{
    bindings::convar::FCVAR_GAMEDLL,
    wrappers::convars::{ConVarRegister, ConVarStruct},
    wrappers::northstar::{EngineLoadType, PluginData, ScriptVmType},
};
use std::num::NonZeroU32;

const NON_ZERO_DEFAULT: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(1) };

static DELAY_CONVAR: OnceCell<ConVarStruct> = OnceCell::new();
pub static DBG_NUM_CONVAR: OnceCell<ConVarStruct> = OnceCell::new();

use plugin_state::PluginState;

mod init_sqvm_stuff;
mod macros;
mod new_order;
mod orders;
mod plugin_state;
mod sqvm_functions;
mod utils;

#[derive(Debug)]
pub struct OrdersPlugin {
    pub state: Mutex<PluginState>,
}

impl Plugin for OrdersPlugin {
    type SaveType = squirrel::Save;

    fn new() -> Self {
        Self {
            state: Mutex::new(PluginState::default()),
        }
    }

    fn initialize(&mut self, plugin_data: &PluginData) {
        plugin_data.register_sq_functions(init_sqvm_stuff::info_wait_for_sqvm);
        plugin_data.register_sq_functions(init_sqvm_stuff::info_runframe);
        plugin_data.register_sq_functions(sqfunction_raw!(
            "SaveFunctions",
            "float functionref( ) get_time, array<entity> functionref( ) get_all_players, string functionref( entity ) get_name",
            "void",
            ScriptVmType::Server,
            init_sqvm_stuff::save_function_objects
        ));
    }

    fn main(&self) {}

    fn on_sqvm_created(&self, sqvm_handle: &squirrel::CSquirrelVMHandle<Self::SaveType>) {
        if ScriptVmType::Server != sqvm_handle.get_context() {
            return;
        }

        init_sqvm_stuff::init_stuff_for_sqvm(sqvm_handle)
    }

    fn on_sqvm_destroyed(&self, context: ScriptVmType) {
        if context != ScriptVmType::Server {
            return;
        }

        let mut lock = self.state.lock();
        lock.active = false;
        lock.endtime = NON_ZERO_DEFAULT;
        lock.nexttime = lock.delay;
        lock.player_message_handles.clear();
        lock.player_score.clear();
    }

    fn on_engine_load(&self, engine: &EngineLoadType) {
        match engine {
            EngineLoadType::Engine(_) => {}
            EngineLoadType::EngineFailed => return,
            EngineLoadType::Server => return,
            EngineLoadType::Client => return,
        };

        let convar = ConVarStruct::try_new().unwrap();
        let register_info = ConVarRegister {
            callback: Some(delay_changed_callback),
            ..ConVarRegister::mandatory(
                "order_repeat",
                "300",
                FCVAR_GAMEDLL.try_into().unwrap(),
                "the time between a chance for a new order",
            )
        };

        convar.register(register_info).unwrap();

        _ = DELAY_CONVAR.set(convar);

        let convar = ConVarStruct::try_new().unwrap();
        let register_info =
            ConVarRegister::new("order_dbg_num", "1", FCVAR_GAMEDLL.try_into().unwrap(), "");

        convar.register(register_info).unwrap();

        _ = DBG_NUM_CONVAR.set(convar);
    }
}

#[rrplug::convar]
fn delay_changed_callback(convar: Option<ConVarStruct>, old_value: String, float_old_value: f32) {
    match NonZeroU32::new(
        DELAY_CONVAR
            .wait()
            .get_value_i32()
            .try_into()
            .unwrap_or_default(),
    ) {
        Some(delay) => PLUGIN.wait().state.lock().delay = delay,
        None => log::error!("this isn't a valid delay"),
    }
}

entry!(OrdersPlugin);
