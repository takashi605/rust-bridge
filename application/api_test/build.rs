fn main() {
    let sdl = api_schema::schema_sdl();
    cynic_codegen::register_schema("app")
        .from_sdl(&sdl)
        .expect("Failed to register GraphQL schema from SDL")
        .as_default()
        .expect("Failed to set schema as default");
}
