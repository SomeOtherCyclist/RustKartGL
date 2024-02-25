pub fn load_shader(shader_name: &str) -> String {
    let environment = std::env::current_dir().unwrap();
    let filepath = environment.join("srcshaders").join(shader_name);

    let expect = "Failed to load shader".to_owned() + &shader_name;

    return std::fs::read_to_string(&filepath).expect(&expect);
}