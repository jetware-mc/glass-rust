#![allow(dead_code)]

use bindings::gtraits::GlassPrint;

pub mod bindings;



on_enable!({
    plugin_name!("sample");
    
    "Hello, World!".gprint();
    register_plugin()
});

on_disable!({
    "Goodbye, World!".gprint();
});