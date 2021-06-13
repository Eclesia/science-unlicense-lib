use std::mem::transmute;
use std::ptr;
use crate::api::Module;

// singleton instance of the registry
static mut _data:*const Registry = 0 as *const Registry;


pub fn getModules<'t>() -> Vec<&'t Module> {

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

pub fn getModuleNames() -> Vec<String> {

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

pub fn registerModule(module: Module) {
    unsafe {
        let registry = get();
        registry.register(module);
    }
}

pub unsafe fn getRegistry<'a>() -> &'a mut Registry {
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
    if _data == ptr::null::<Registry>() {
        _data = transmute(Box::new(Registry { modules: vec![] }));
    }
    return transmute(_data);
}

unsafe fn release() {
    ptr::read::<Registry>(_data);
}

impl Drop for Registry {
    fn drop(&mut self) {
        println!("Dropped static");
    }
}