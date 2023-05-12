use honggfuzz::fuzz;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            if let Ok(input_str) = std::str::from_utf8(data) {
                let jd = &mut serde_json::Deserializer::from_str(input_str);
                let _: Result<&[u8], _> = serde_path_to_error::deserialize(jd);
            }
        });
    }
}