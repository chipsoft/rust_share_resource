use core::mem::swap;
use core::mem::replace;

pub struct Resource {
    id: u32,
}

impl Resource {
    pub fn new(id: u32) -> Self {
        Resource { id }
    }

    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }
}

struct Use1 {
    resource: Option<Resource>,
}

impl Use1 {

    fn new() -> Self {
        Use1 { resource: None }
    }

    fn do_something(&mut self, id: u32) {
        if let Some(tmp) = &mut self.resource {
            tmp.set_id(id);
            println!("Use1 command");
        }
        else {
            panic!();
        }
    }

    fn acquire_resource(&mut self, resource: Option<Resource>) {
        self.resource = resource;
    }

    fn release_resource(&mut self) -> Option<Resource> {
        replace(&mut self.resource, None)
    }

}

struct Use2 {
    resource: Option<Resource>,
}

impl Use2 {

    fn new() -> Self {
        Use2 { resource: None }
    }

    fn do_something(&mut self, id: u32) {
        if let Some(tmp) = &mut self.resource {
            tmp.set_id(id);
            println!("Use2 command");
        }
        else {
            panic!();
        }
    }

    fn acquire_resource(&mut self, resource: Option<Resource>) {
        self.resource = resource;
    }

    fn release_resource(&mut self) -> Option<Resource> {
        replace(&mut self.resource, None)
    }
    
}

struct Manager {
    resource: Option<Resource>,
    use1: Use1,
    use2: Use2,
}

impl Manager {
    fn new(resource: Resource) -> Self {
        Manager { resource: Some(resource), use1: Use1::new(), use2: Use2::new() }
    }

    fn release_resource(&mut self) -> Option<Resource> {
        replace(&mut self.resource, None)
    }

    fn applyUse1(&mut self, id: u32) {
        let temp_res = self.release_resource();
        self.use1.acquire_resource(temp_res);
        // self.use1.set_resource(replace(&mut self.resource, None));
        // swap(&mut self.resource, &mut self.use1.resource);
        self.use1.do_something(id);
        // swap(&mut self.use1.resource, &mut self.resource);
        // self.resource = replace(&mut self.use1.resource, None)
        self.resource = self.use1.release_resource();
    }

    fn applyUse2(&mut self, id: u32) {
        let temp_res = self.release_resource();
        self.use2.acquire_resource(temp_res);        
        // self.use2.set_resource(replace(&mut self.resource, None));
        // swap(&mut self.resource, &mut self.use2.resource);
        self.use2.do_something(id);
        // swap(&mut self.use2.resource, &mut self.resource);
        // self.resource = replace(&mut self.use2.resource, None)
        self.resource = self.use2.release_resource();
    }
}

fn main() {
    let resource = Resource::new(1);
    let mut manager = Manager::new(resource);
    manager.applyUse1(1);
    manager.applyUse2(2);
    manager.applyUse1(1);
    manager.applyUse2(2);
    manager.applyUse1(1);
    manager.applyUse2(2);
    manager.applyUse1(1);
    manager.applyUse1(1);
    manager.applyUse2(2);
    manager.applyUse2(2);
}

// TODO: All of component must have release_resource
// TODO: All chipd component must have acquire_resource
