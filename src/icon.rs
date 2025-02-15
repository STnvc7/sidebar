#[allow(dead_code)]
enum IconType {
    Python,
    Rust,
    Go,
    Cpp,
    C,
    Cs,
    Php,
    Ruby,
    R,
    Java,
    Kotlin,
    Swift,
    Dart,
    JavaScript,
    TypeScript,
    React,
    Html,
    Css,
    Json,
    Yaml,
    Markdown,
    Toml,
    Git,
    Docker,
    Text,
    Image,
    Sound,
    Pdf,
    Other,
}

pub fn get_folder_icon(is_open: bool, nerd_font: bool) -> String {
    if nerd_font {
        if is_open {
            return String::from("\u{f115}")
        }else {
            return String::from("\u{f114}")
        }
    }else {
        if is_open {
            return String::from("📂")
        }else {
            return String::from("📁")
        }
    }
}

pub fn get_file_icon(name: &String, nerd_font: bool) -> String {
    let icon_type = get_file_icon_type(name);

    // nerd fontがインストールされてないとき
    if !nerd_font {
        return String::from("≡")
    }

    // nerd fontがあるとき
    let icon = match icon_type {
        IconType::Python => "\u{e606}",
        IconType::Rust => "\u{e68b}",
        IconType::Go => "\u{e65e}",
        IconType::Cpp => "\u{e61d}",
        IconType::C => "\u{e61e}",
        IconType::Cs => "\u{e7b2}",
        IconType::Php => "\u{ed6d}",
        IconType::Ruby => "\u{e605}",
        IconType::R => "\u{e881}",
        IconType::Java => "\u{e738}",
        IconType::Kotlin => "\u{e634}",
        IconType::Swift => "\u{e755}",
        IconType::Dart => "\u{e64c}",
        IconType::JavaScript => "\u{f2ee}",
        IconType::TypeScript => "\u{e69d}",
        IconType::React => "\u{e7ba}",
        IconType::Html => "\u{f13b}",
        IconType::Css => "\u{f13c}",
        IconType::Json => "\u{e60b}",
        IconType::Yaml => "\u{e615}",
        IconType::Markdown => "\u{e609}",
        IconType::Toml => "\u{e615}",
        IconType::Git => "\u{f02a2}",
        IconType::Docker => "\u{f21f}",
        IconType::Text => "\u{e64e}",
        IconType::Image => "\u{f03e}",
        IconType::Sound => "\u{f147d}",
        IconType::Pdf => "\u{eaeb}",
        IconType::Other => "\u{e64e}"
    };

    return String::from(icon);
}

fn get_file_icon_type(name: &String) -> IconType {

    let splited : Vec<&str> = name.split('.').collect();
    if splited.len() == 0 {
        return IconType::Other
    }

	let extension : &str = splited.last().unwrap();
    let icon_type = match extension {
        "py" => IconType::Python,
        "rs" => IconType::Rust,
        "go" => IconType::Go,
        "cpp" | "hpp" => IconType::Cpp,
        "c" | "h" => IconType::C,
        "cs" | "hs" => IconType::Cs,
        "rb" => IconType::Ruby,
        "r" => IconType::R,
        "php" => IconType::Php,
        "java" => IconType::Java,
        "dart" => IconType::Dart,
        "kt" | "kts" => IconType::Kotlin,
        "swift" => IconType::Swift,
        "js" => IconType::JavaScript,
        "ts" => IconType::TypeScript,
        "jsx" | "tsx" => IconType::React,
        "html" => IconType::Html,
        "css" => IconType::Css,
        "json" => IconType::Json,
        "yaml" => IconType::Yaml,
        "md" => IconType::Markdown,
        "toml" => IconType::Toml,
        "gitignore" => IconType::Git,
        "dockerfile" => IconType::Docker,
        "txt" => IconType::Text,
        "png" | "jpeg" | "jpg" | "gif" => IconType::Image,
        "wav" | "mp3" | "aiff" | "aac" | "flac" => IconType::Sound,
        "pdf" => IconType::Pdf,
        _ => IconType::Other,
    };

    return icon_type;
}