fn main() {
    let sdl = api_schema::schema_sdl();
    cynic_codegen::register_schema("app")
        .from_sdl(&sdl)
        .unwrap()
        .as_default()
        .unwrap();
}
