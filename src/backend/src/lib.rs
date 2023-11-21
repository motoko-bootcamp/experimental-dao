use ic_cdk_macros::{query, update, export_candid};


#[query]
fn hello(name: String) -> String {
    format!("Hello, {}!", name)
}

#[update]
fn world(name: String) -> String {
    format!("World, {}!", name)
}

export_candid!();
