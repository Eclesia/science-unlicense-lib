use std::mem::transmute;
use std::ptr;
use crate::api::Module;

// singleton instance of the registry
static mut _DATA:*const Registry = 0 as *const Registry;


pub fn get_modules<'t>() -> Vec<&'t Module> {

    let mut v: Vec<&Module> = vec![];
    unsafe {
        let registry = get();
        let ite = registry.modules.iter();
        for x in ite {
            v.push(x);
        }
    }
    return v;
}

pub fn get_module_names() -> Vec<String> {

    let mut v: Vec<String> = vec![];
    unsafe {
        let registry = get();
        let ite = registry.modules.iter();
        for x in ite {
            v.push(x.name.clone());
        }
    }
    return v;
}

pub fn register_module(module: Module) {
    unsafe {
        let registry = get();
        registry.register(module);
    }
}

pub unsafe fn get_registry<'a>() -> &'a mut Registry {
    return get();
}

pub struct Registry {
    modules: Vec<Module>
}

impl Registry {

    pub fn register(&mut self, module:Module) {
        println!("Registering module {}", module.name);
        self.modules.push(module);
    }

}

unsafe fn get<'a>() -> &'a mut Registry {
    if _DATA == ptr::null::<Registry>() {
        _DATA = transmute(Box::new(Registry { modules: vec![] }));
    }
    return transmute(_DATA);
}

// `#[allow(dead_code)]` is an attribute that disables the `dead_code` lint
// this is normal
#[allow(dead_code)]
unsafe fn release() {
    ptr::read::<Registry>(_DATA);
}

impl Drop for Registry {
    fn drop(&mut self) {
        println!("Dropped static");
    }
}