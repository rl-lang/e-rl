use crate::{stdlib::macros::vs, values::VmValue, vm_logic::Vm};
use alloc::string::ToString;

pub fn func(eval: &mut Vm) -> VmValue {
    match eval.source_file() {
        Some(f) => vs!(f.name.to_string()),
        None => VmValue::Null,
    }
}
