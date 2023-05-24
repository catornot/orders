#[macro_export]
macro_rules! sqfunction_raw {
    ($name: literal, $sqtypes: literal, $sqreturn: literal, $vm: path, $function: path ) => {{
        unsafe extern "C" fn trampoline(
            sqvm: *mut rrplug::bindings::squirreldatatypes::HSquirrelVM,
        ) -> i32 {
            $function(sqvm)
        }

        || ($name, $name, $sqtypes, $sqreturn, $vm, trampoline)
    }};
}

#[macro_export]
macro_rules! get_func_object {
    ($stack_pos: literal, $sqvm:expr, $sq_functions: expr) => {
        unsafe {
            let mut obj = Box::new(std::mem::MaybeUninit::<SQObject>::zeroed());
            ($sq_functions.sq_getobject)($sqvm, $stack_pos, obj.as_mut_ptr());
            obj
        }
    };
}