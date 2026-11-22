fn main() {
    println!("cargo:rerun-if-changed=../assets/brand/umuryango-app.ico");

    #[cfg(windows)]
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let mut resource = winres::WindowsResource::new();
        resource.set_icon("../assets/brand/umuryango-app.ico");
        resource
            .compile()
            .expect("Windows application icon must compile");
    }
}
