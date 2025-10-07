report var const: u32;
report var misc: u32;
report var control: u32;
report var local: u32;
report var global: u32;
report var table: u32;
report var load: u32;
report var store: u32;
report var mem: u32;
report var arith: u32;
report var compare: u32;
report var convert: u32;
report var exn: u32;
report var simd: u32;
report var ref: u32;
report var gc: u32;
report var atomic: u32;

wasm:opcode:*:before {
    if (category_name == "const") {
        const++;
    } elif (category_name == "misc") {
        misc++;
    } elif (category_name == "control") {
        control++;
    } elif (category_name == "local") {
        local++;
    } elif (category_name == "global") {
        global++;
    } elif (category_name == "table") {
        table++;
    } elif (category_name == "load") {
        load++;
    } elif (category_name == "store") {
        store++;
    } elif (category_name == "mem") {
        mem++;
    } elif (category_name == "arith") {
        arith++;
    } elif (category_name == "compare") {
        compare++;
    } elif (category_name == "convert") {
        convert++;
    } elif (category_name == "exn") {
        exn++;
    } elif (category_name == "simd") {
        simd++;
    } elif (category_name == "ref") {
        ref++;
    } elif (category_name == "gc") {
        gc++;
    } elif (category_name == "atomic") {
        atomic++;
    }
}