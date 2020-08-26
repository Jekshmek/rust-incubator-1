extern crate step_3_2;
use step_3_2::btreemap_proc;

macro_rules! btreemap_decl {
    ( $(( $key:expr, $val:expr )),* ) => {{
        let mut map = std::collections::BTreeMap::new();
        $(
            map.insert($key, $val);
        )*
        map
    }};
}

fn main() {
    let map_decl = btreemap_decl![(1, "one"), (2, "two")];
    let map_proc = btreemap_proc![(1, "one"), (2, "two")];

    dbg!(&map_proc);

    assert_eq!(map_decl, map_proc);
}
