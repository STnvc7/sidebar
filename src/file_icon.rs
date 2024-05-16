
enum FileType{
	Python,
	Rust,
	Cpp,
	JavaScript,
	Html,
	Css,
	Json,
	Yaml,
	Markdown,
	Text,
	Image,
	Sound,
	Other,
}

pub fn get_file_icon(filename : &String) -> &str{

	let splited : Vec<&str> = filename.split('.').collect();
	let suffix : &str = splited.last().unwrap();

	let filetype = 
	match suffix{
		"py" 	=> {FileType::Python},
		"rs" 	=> {FileType::Rust},
		"cpp"	=> {FileType::Cpp},
		"hpp"	=> {FileType::Cpp},
		"js"	=> {FileType::JavaScript},
		"html"	=> {FileType::Html},
		"css"	=> {FileType::Css},
		"json"	=> {FileType::Json},
		"yaml"  => {FileType::Yaml},
		"md"	=> {FileType::Markdown},
		"txt"	=> {FileType::Text},
		"png"	=> {FileType::Image},
		"jpg"	=> {FileType::Image},
		"jpeg"	=> {FileType::Image},
		"wav"	=> {FileType::Sound},
		"mp3"	=> {FileType::Sound},
		_    	=> {FileType::Other},
	};

	let icon = 
	match filetype{
		FileType::Python => {"\x1b[38;5;33mp\x1b[38;5;3my "},
		FileType::Rust   => {"\x1b[38;5;130mrs "},
		FileType::Cpp	 => {"\x1b[38;5;26mcp "},
		FileType::JavaScript => {"\x1b[38;5;226mjs "},
		FileType::Html	 => {"\x1b[38;5;166m<> "},
		FileType::Css	 => {"\x1b[38;5;32m{} "}
		FileType::Json	 => {"\x1b[38;5;172m{} "},
		FileType::Yaml   => {"\x1b[38;5;136m:- "},
		FileType::Markdown => {"\x1b[38;5;109m•- "},
		FileType::Text   => {"📄 "},
		FileType::Image	 => {"🖼️ "}
		FileType::Sound	 => {"🔊 "}
		FileType::Other  => {"📄 "},
	};

	return icon
}
