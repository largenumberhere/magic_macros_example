//! A working example of macro_magic from the example given on the crates.io page at  https://crates.io/crates/macro_magic
//! The subproject project_proc_macros is where it all happens
//!

use Default;
use std::fmt::format;

fn main() {
    //Use the struct we made from S and S2 in the code bellow
    let s = M::S2::default();
    let s2_debug = format!("{:?}", s);

    assert_eq!("S2 { item3: \"\", item4: 0, item5: false, item: \"\", item2: \"\" }", s2_debug);
    println!("{:#?}", s);
}



mod M {
    //Use the attributes we created
    use project_proc_macros::combine_structs;
    use project_proc_macros::export_struct;

    //This attribute is exported from macro magic in project_proc_macros/src/lib.rs
    #[export_struct]
    pub struct S {
        item: String,
        item2: String,
    }

    //This is the attribute defined in project_proc_macros/src/lib.rs
    #[combine_structs(S)]
    /*These attributes must go underneath to apply to the output that gets created. They don't work if above combine_structs*/
    #[derive(Debug)]
    #[derive(Default)]
    pub struct S2 {
        item3: String,
        item4: usize,
        item5: bool
    }
}


