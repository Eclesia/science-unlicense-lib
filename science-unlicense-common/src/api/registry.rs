use std::mem::transmute;
use std::ptr;

// singleton instance of the registry
static mut _data:*const Registry = 0 as *const Registry;


pub fn getModules() -> Vec<String> {

    let mut v: Vec<String> = vec![];
    unsafe {
        let registry = get();
        v = registry.modules();
    }
    return v;
}

pub unsafe fn getRegistry<'a>() -> &'a mut Registry {
    return get();
}


pub struct Registry {
    modules: Vec<String>
}

impl Registry {

    pub fn register(&mut self, name:String) {
        println!("Write to static");
        self.modules.push(name);
    }

    pub fn modules(&self) -> Vec<String>{
        return self.modules.clone();
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