pub fn parse_script(input: String) {
    let mut real_tokens: Vec<&str> = Vec::new();    
    let mut inside_pes: bool = false;

    input.split(" ").into_iter().map(|token| {
        match token {
            "pes" => {
                inside_pes = true;
            },
            _ => {
                if inside_pes {
                    real_tokens.push(token);
                }
            }
        }
    })
    .collect()
}
