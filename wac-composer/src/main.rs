use wac_graph::{types::Package, CompositionGraph, EncodeOptions};


const RUN_FUNC_PREFIX: &str = "wasi:cli/run@";
// const APP_COMPONENT: &str = "../component.manually-fixed.wasm";
const APP_COMPONENT: &str = "../output/tests/core-components/output.wasm";
const CORE_LIB_NAME: &str = "whamm-core";
// const CORE_LIB_COMPONENT: &str = "../whamm_core-component/target/wasm32-wasip1/release/whamm_core_component.wasm";
const CORE_LIB_COMPONENT: &str = "../whamm_core-component/target/wasm32-wasip2/release/whamm_core.wasm";

fn main() {
    let mut graph = CompositionGraph::new();

    // Register the package dependencies into the graph
    let package = Package::from_file(
        "app",
        None,
        APP_COMPONENT,
        graph.types_mut(),
    ).unwrap();
    let app = graph.register_package(package).unwrap();

    let package = Package::from_file(
        CORE_LIB_NAME,
        None,
        CORE_LIB_COMPONENT,
        graph.types_mut(),
    ).unwrap();
    let whamm_core = graph.register_package(package).unwrap();

    // print out some helpful information about what the imports/exports are from the packages.
    println!("LIB EXPORTS:");
    for (name, ty) in &graph.types()[graph[whamm_core].ty()].exports {
        println!("- {name}: {:?}", ty);
    }
    println!("APP IMPORTS");
    for (name, ty) in &graph.types()[graph[app].ty()].imports {
        println!("- {name}: {:?}", ty);
    }
    println!("APP EXPORTS");
    let mut run_func_name = None;
    for (name, ty) in &graph.types()[graph[app].ty()].exports {
        if name.starts_with(RUN_FUNC_PREFIX) {
            run_func_name = Some(name.clone());
        }
        println!("- {name}: {:?}", ty);
    }

    // Instantiate the whamm_core instance which does not have any arguments
    let whamm_core_instance = graph.instantiate(whamm_core);

    // Instantiate the app instance which has a single argument "whamm-core"
    // which is an instance of `whamm_core`
    let app_instance = graph.instantiate(app);

    // plug in the instance of `whamm_core` into the `app` import.
    graph
        .set_instantiation_argument(app_instance, CORE_LIB_NAME, whamm_core_instance)
        .unwrap();

    // Export the "run" function from the app
    if let Some(run_name) = run_func_name {
        let run_export = graph
            .alias_instance_export(app_instance, &run_name)
            .unwrap();
        graph.export(run_export, &run_name).unwrap();
    } else {
        panic!("Could not find an exported main function from the component, should start with: {RUN_FUNC_PREFIX}")
    }

    // Encode the graph into a WASM binary
    let encoding = graph.encode(EncodeOptions::default()).unwrap();
    std::fs::write("composition.wasm", encoding).unwrap();
}