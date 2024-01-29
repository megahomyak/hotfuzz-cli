use pancurses::Input;

enum ModuleNameChar {
    Hot(char),
    Regular(char),
}

struct ModuleName {
    chars: Vec<ModuleNameChar>
}

fn parse_file_name(name: &str) -> ModuleName {
    let mut hot = false;
    let mut chars = Vec::new();
    for c in name.chars() {
        match c {
            '[' => hot = true,
            ']' => hot = false,
            _ => chars.push(if hot {
                ModuleNameChar::Hot(c)
            } else {
                ModuleNameChar::Regular(c)
            })
        }
    }
    ModuleName { chars }
}

struct Matcher {
    items: Vec<ModuleName>,
}

fn main() {
    let modules_path = std::env::var("HOTFUZZ_MODULES_PATH").expect("`HOTFUZZ_MODULES_PATH` must be set");
    let module_names: Vec<_> = std::fs::read_dir(modules_path).expect("could not read at `HOTFUZZ_MODULES_PATH`").map(
        |entry| {
            let name = entry.unwrap().file_name();
            let name = name.to_str().unwrap();
            parse_file_name(name)
        }
    ).collect();

    fn up() {}
    fn down() {}
    fn select() {}

    let window = pancurses::initscr();
    window.keypad(true); // To handle arrows
    let (mut width, mut height) = window.get_max_yx();
    loop {
        match window.getch() {
            Some(Input::KeyResize) => (width, height) = window.get_max_yx(),
            Some(Input::KeyUp) => up(),
            Some(Input::KeyDown) => down(),
            Some(Input::KeyEnter | Input::Character('\n')) => select(),
            Some(Input::Character('\x1b')) => break, // "Escape"
            _ => (),
        }
        println!("tick");
    }
}
