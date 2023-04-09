use std::io;

pub(crate) fn wait_input_sync() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().to_string(),
        Err(e) => panic!("Can not get input value: {:?}", e),
    }
}

pub(crate) fn split_string(input: String, sep: char) -> Option<(String, String)> {
    let found: Vec<&str> = input.splitn(2, |c| c == sep).collect();
    if let 2 = found.len() {
        let f = found.first().unwrap().trim();
        let s = found.get(1).unwrap().trim();
        if !f.is_empty() && !s.is_empty() {
            return Some((f.to_string(), s.to_string()));
        }
    }
    None
}
