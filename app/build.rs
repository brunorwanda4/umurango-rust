fn main() {
    println!("cargo:rerun-if-changed=../assets/brand/umuryango-app.ico");

    #[cfg(windows)]
    {
        let mut resource = winres::WindowsResource::new();
        resource.set_icon("../assets/brand/umuryango-app.ico");
        resource
            .compile()
            .expect("Windows application icon must compile");
    }
}
