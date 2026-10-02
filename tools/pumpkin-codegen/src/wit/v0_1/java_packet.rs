pub fn build() -> String {
    std::fs::read_to_string(std::path::Path::new(super::WIT_OUT_DIR).join("java-packets.wit"))
        .expect("Failed to read v0.1 Java packet schema")
}
