use rrplug::{
    bindings::squirreldatatypes::{HSquirrelVM, SQObject},
    sq_return_null,
    wrappers::squirrel::{
        call_sq_object_function, compile_string, CSquirrelVMHandle, Save, SQFUNCTIONS,
    },
};

use crate::{get_func_object, new_order::try_new_order, sqvm_functions::SQVM_FUNCTIONS, PLUGIN};

pub fn init_stuff_for_sqvm(handle: &CSquirrelVMHandle<Save>) {
    let sqvm = unsafe { handle.get_sqvm() };
    let sqfunctions = SQFUNCTIONS.server.wait();

    compile_string(
        sqvm,
        sqfunctions,
        true,
        r#"
    thread void function() {
        wait 0
        WaitForFullStartup()
    }()
    "#,
    )
    .unwrap()
}

#[rrplug::sqfunction(VM=Server,ExportName=WaitForFullStartup)]
pub fn wait_for_sqvm() {
    // {
    //     let mut sqvm_functions = SQVM_FUNCTIONS.write();
    //     sqvm_functions.get_all_players = get_sq_function(sqvm, sq_functions, "GetPlayerArray");
    //     sqvm_functions.get_time = get_sq_function(sqvm, sq_functions, "Time");
    // }

    compile_string(
        sqvm,
        sq_functions,
        true,
        r#"
    thread void function() {
        string functionref( entity ) get_name = string function( entity player ) { return player.GetPlayerName() }
        SaveFunctions( Time, GetPlayerArray, get_name )

        wait 0

        for(;;) {
            RunFrame()
            wait 0
        }
    }()
    "#,
    )
    .unwrap();

    sq_return_null!()
}

pub fn save_function_objects(sqvm: *mut HSquirrelVM) -> i32 {
    let sq_functions = SQFUNCTIONS.server.wait();
    let get_time = get_func_object!(1, sqvm, sq_functions);
    let get_all_players = get_func_object!(2, sqvm, sq_functions);
    let get_name = get_func_object!(3, sqvm, sq_functions);

    call_sq_object_function(sqvm, sq_functions, *get_time).unwrap();

    let time = unsafe { (sq_functions.sq_getfloat)(sqvm, 3) } as u32;

    log::info!("time {time}");

    let mut sqvm_functions = SQVM_FUNCTIONS.write();
    sqvm_functions.get_all_players = *get_all_players;
    sqvm_functions.get_time = *get_time;
    sqvm_functions.get_name = *get_name;

    sq_return_null!()
}

#[rrplug::sqfunction(VM=Server,ExportName=RunFrame)]
pub fn runframe() {
    // unsafe {
    //     let sqvm_ref = &*sqvm;

    //     for (e, obj) in (0..20)
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

    let mut state = PLUGIN.wait().state.lock();

    try_new_order(sqvm, sq_functions, &mut state);

    sq_return_null!()
}
