use std::{ffi::CStr, mem::MaybeUninit};

use rrplug::{
    bindings::{
        squirreldatatypes::{HSquirrelVM, SQObject, SQObjectType_OT_STRING},
        unwraped::SquirrelFunctionsUnwraped,
    },
    call_sq_object_function as call_sq_object_function_macro,
    wrappers::squirrel::call_sq_object_function,
};

use crate::{plugin_state::PluginState, sqvm_functions::SQVM_FUNCTIONS, utils::find_var_on_stack};

pub fn try_new_order(
    sqvm: *mut HSquirrelVM,
    sq_functions: &SquirrelFunctionsUnwraped,
    state: &mut PluginState,
) {
    let mut sqvm_functions = SQVM_FUNCTIONS.write();

    call_sq_object_function(sqvm, sq_functions, sqvm_functions.get_time).unwrap();

    // unsafe {
    //     let sqvm_ref = &*sqvm;

    //     for (e, obj) in (0..10)
    //         .map(|i| sqvm_ref._stack.add(i))
    //         // .filter(|ptr| ptr.is_null())
    //         .map(|ptr| &*ptr)
    //         .enumerate()
    //     {
    //         log::info!("Type {e} {:?}", obj._Type);

    //         let time = (sq_functions.sq_getfloat)(sqvm, e as i32);

    //         log::info!("time {time}");
    //     }
    // }

    // might be a good idea to make runtime stack search lol

    let time = unsafe { (sq_functions.sq_getfloat)(sqvm, 2) } as u32;

    log::info!("time {time}");

    // let mut random = rand::thread_rng();

    // if state.nexttime.get() > time && random.gen_range(0..=100) > state.chance.get() {
    //     return;
    // }

    // if time < 100 {
    //     return;
    // }

    // if state.active {
    //     return;
    // }

    state.active = true;

    log::info!("new order");

    call_sq_object_function(sqvm, sq_functions, sqvm_functions.get_all_players).unwrap();

    unsafe {
        let sqvm_ref = &*sqvm;

        // for (e, obj) in (0..10)
        //     .map(|i| sqvm_ref._stack.add(i))
        //     // .filter(|ptr| ptr.is_null())
        //     .map(|ptr| &*ptr)
        //     .enumerate()
        // {
        //     log::info!("Type {e} {:?}", obj._Type);

        //     let time = (sq_functions.sq_getfloat)(sqvm, e as i32);

        //     log::info!("time {time}");
        // }

        let player_array_object: &rrplug::bindings::squirreldatatypes::SQObject =
            &*sqvm_ref._stack.add(7);
        // log::info!("type array {}", player_array_object._Type);
        let player_array = &*player_array_object._VAL.asArray;

        // log::info!("player_array._usedSlots {}", player_array._usedSlots);

        (0..player_array._usedSlots)
            .filter_map(|i| i.try_into().ok())
            .map(|i| player_array._values.add(i))
            .map(|ptr| MaybeUninit::new(*ptr))
            // .inspect(|_| log::info!("player found"))
            .collect::<Vec<MaybeUninit<SQObject>>>()
            .into_iter()
            .map(|ent| {
                call_sq_object_function_macro!(sqvm, sq_functions, sqvm_functions.get_name, ent)
                    .unwrap();
                // find_var_on_stack(sqvm, SQObjectType_OT_STRING);
                // Some(
                //     (*(sqvm.as_ref()?._stack.add(9)))._VAL.asString.as_ref()? as *const _
                //         as *const i8,
                // )
                (sq_functions.sq_getstring)(sqvm, crate::DBG_NUM_CONVAR.wait().get_value_i32())
            })
            .map(|ptr| CStr::from_ptr(ptr).to_string_lossy().to_string())
            .for_each(|name| log::info!("name {name:?}"));
    }
}
