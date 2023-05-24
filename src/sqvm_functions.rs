use std::mem::MaybeUninit;

use parking_lot::RwLock;
use rrplug::bindings::squirreldatatypes::SQObject;


pub static SQVM_FUNCTIONS: RwLock<SqvmFunctions> = RwLock::new(SqvmFunctions::new());

pub struct SqvmFunctions {
    pub get_all_players: MaybeUninit<SQObject>,
    pub get_time: MaybeUninit<SQObject>,
    pub get_name: MaybeUninit<SQObject>,
}

impl SqvmFunctions {
    const fn new() -> Self {
        Self {
            get_all_players: MaybeUninit::uninit(),
            get_time: MaybeUninit::uninit(),
            get_name: MaybeUninit::uninit(),
        }
    }
}

unsafe impl Sync for SqvmFunctions {}
unsafe impl Send for SqvmFunctions {}
