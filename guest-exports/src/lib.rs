wit_bindgen::generate!({
    path: "../exports.wit",
    world: "example",
    exports: {
        // world: Component,
        "component:guest-reactor/guest-add": ComponentAdd,
        "guest-sub": ComponentSub,
    }
});

// struct Component;

// impl Guest for Component {}

struct ComponentAdd;

impl exports::component::guest_reactor::guest_add::Guest for ComponentAdd {
    fn add_three(num: i32) -> i32 {
        let num = component::guest_reactor::host_add::add_one(num);
        let num = component::guest_reactor::host_add::add_one(num);
        let num = component::guest_reactor::host_add::add_one(num);
        num
    }
}

struct ComponentSub;

impl exports::guest_sub::Guest for ComponentSub {
    fn sub_three(num: i32) -> i32 {
        let num = host_sub::sub_one(num);
        let num = host_sub::sub_one(num);
        let num = host_sub::sub_one(num);
        num
    }
}
