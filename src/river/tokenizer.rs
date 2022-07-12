pub fn tokenize(file_data: String) -> Vec<String> {
    #[cfg(target_os = "linux")]
    let lines = file_data.split("\n");
    #[cfg(target_os = "windows")]
    let lines = file_data.split("\r\n");

    let mut tokens: Vec<String> = Vec::new();

    for line in lines {
        //TODO Find a regex string for spliting
        let to_push = line.split("");
        for thing in to_push { tokens.push(thing.to_string()); }
    }

    return tokens;
}