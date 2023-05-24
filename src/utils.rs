use rrplug::{
    bindings::{
        squirreldatatypes::{HSquirrelVM, SQObject, SQObjectType},
        unwraped::SquirrelFunctionsUnwraped,
    },
    to_sq_string,
};
use std::mem::MaybeUninit;

#[allow(unused)]
pub fn get_sq_function(
    sqvm: *mut HSquirrelVM,
    sqfunctions: &SquirrelFunctionsUnwraped,
    function_name: impl Into<String>,
) -> MaybeUninit<SQObject> {
    let mut obj = Box::new(std::mem::MaybeUninit::<SQObject>::zeroed());
    let ptr = obj.as_mut_ptr();

    let function_name = function_name.into();
    let sq_function_name = to_sq_string!(function_name);

    let result = unsafe {
        (sqfunctions.sq_getfunction)(sqvm, sq_function_name.as_ptr(), ptr, std::ptr::null())
    };

    if result != 0 {
        panic!("no function found {}", function_name)
    } else {
        *obj
    }
}

#[allow(unused)]
pub unsafe fn find_var_on_stack(sqvm: *mut HSquirrelVM, var_type: SQObjectType) {
    let sqvm_ref = &*sqvm;

    for (obj, e) in (0..20)
        .map(|i| (sqvm_ref._stack.add(i), i))
        .map(|(ptr, i)| (&*ptr, i))
        // .filter(|(object, _)| object._Type == var_type)
    {
        log::info!("found var at {e} {:?}", obj._Type);
    }
}
