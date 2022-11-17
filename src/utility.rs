pub fn is_hidden(name: std::ffi::OsString) -> bool {
    let mut is_hidden = false;
    let file_name_owned = name.to_str().to_owned();
    match file_name_owned {
        Some(f) => {
            if f.starts_with(".") {
                is_hidden = true;
            }
        },
        None => is_hidden = true
    }
    is_hidden
}
